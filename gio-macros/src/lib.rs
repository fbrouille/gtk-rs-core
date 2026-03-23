// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    GenericArgument, PathArguments, Token, Type, TypeParamBound, braced, parenthesized,
    parse::{Parse, ParseStream},
};

/// A single `name: ty` parameter pair.
struct Param {
    name: Ident,
    _colon: Token![:],
    ty: Type,
}

impl Parse for Param {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Param {
            name: input.parse()?,
            _colon: input.parse()?,
            ty: input.parse()?,
        })
    }
}

/// Attribute arguments parsed from e.g. `#[gio_macros::async_finish]`
/// or `#[gio_macros::async_finish(out_param = true)]`.
struct AsyncFinishAttr {
    has_out_param: bool,
}

impl Parse for AsyncFinishAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut has_out_param = false;

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            if ident == "out_param" {
                let _eq: Token![=] = input.parse()?;
                let value: syn::LitBool = input.parse()?;
                has_out_param = value.value;
            } else {
                return Err(syn::Error::new(ident.span(), "expected `out_param`"));
            }
            if !input.is_empty() {
                let _comma: Token![,] = input.parse()?;
            }
        }

        Ok(AsyncFinishAttr { has_out_param })
    }
}

/// Parsed from the annotated function item:
///
/// ```rust,ignore
/// #[gio_macros::async_finish]
/// async fn #func_basename#_async_result<T: #impl_iface>(imp: &T,  #param_name_1:  #param_type_1, ..., io_priority: i32) -> Result<#result_type, glib::Error> {
///     imp.#func_basename#_future(from_glib(#param_name_1), ..., from_glib(io_priority))
/// }
/// ```
///
/// The generics (`<T: #impl_iface>`), `imp: &T`, and `io_priority: i32` are
/// parsed as markers. The macro generates their actual bindings. Extra parameters between `imp`
/// and `io_priority` become FFI function parameters.
///
/// The result type is inferred from `Result<#result_type, glib::Error>` in the return type.
struct AsyncFinishInput {
    fn_name: Ident,
    generics: syn::Generics,
    extra_params: Vec<Param>,
    rust_ret_type: Type,
    result_expr: proc_macro2::TokenStream,
}

impl Parse for AsyncFinishInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // async fn fn_name
        let _async: Token![async] = input.parse()?;
        let _fn_token: Token![fn] = input.parse()?;
        let fn_name: Ident = input.parse()?;

        // <T: #impl_iface>
        let generics: syn::Generics = input.parse()?;

        // (imp: &T, [extra_params...,] io_priority: i32)
        let params_content;
        parenthesized!(params_content in input);

        // Parse all params as name: type
        let all_params: Vec<Param> = params_content
            .parse_terminated(Param::parse, Token![,])?
            .into_iter()
            .collect();

        // First must be `imp`, last must be `io_priority`
        if all_params.is_empty() || all_params.first().unwrap().name != "imp" {
            return Err(input.error("first parameter must be `imp: &T`"));
        }
        if all_params.len() < 2 || all_params.last().unwrap().name != "io_priority" {
            return Err(input.error("last parameter must be `io_priority: i32`"));
        }

        // Middle params are the extra FFI params
        let extra_params = all_params
            .into_iter()
            .skip(1) // skip imp
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .skip(1) // skip io_priority
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();

        // -> Result<T, glib::Error>
        let _arrow: Token![->] = input.parse()?;
        let ret_type: Type = input.parse()?;

        // Extract T from Result<T, ...>
        let rust_ret_type = extract_result_ok_type(&ret_type)?;

        // { body }
        let body_content;
        braced!(body_content in input);
        let result_expr: proc_macro2::TokenStream = body_content.parse()?;

        Ok(AsyncFinishInput {
            fn_name,
            generics,
            extra_params,
            rust_ret_type,
            result_expr,
        })
    }
}

/// Extract `T` from `Result<T, E>`.
fn extract_result_ok_type(ty: &Type) -> syn::Result<Type> {
    if let Type::Path(type_path) = ty {
        let last_seg = type_path.path.segments.last().unwrap();
        if last_seg.ident == "Result" {
            if let PathArguments::AngleBracketed(ref args) = last_seg.arguments {
                if let Some(GenericArgument::Type(ok_type)) = args.args.first() {
                    return Ok(ok_type.clone());
                }
            }
        }
    }
    Err(syn::Error::new_spanned(
        ty,
        "expected `Result<T, glib::Error>` return type",
    ))
}

/// Convert a Rust type to its FFI equivalent.
/// Maps common gtk-rs types to their FFI counterparts.
fn rust_to_ffi_type(ty: &Type) -> Type {
    if let Type::Path(type_path) = ty {
        if let Some(last_seg) = type_path.path.segments.last() {
            let ident_str = last_seg.ident.to_string();
            
            // Check for known Rust types and convert them to FFI types
            let ffi_type_str = match ident_str.as_str() {
                "GString" => "*const libc::c_char".to_string(),
                "Option" => "*const libc::c_char".to_string(),
                "FileQueryInfoFlags" => "ffi::GFileQueryInfoFlags".to_string(),
                "FileCreateFlags" => "ffi::GFileCreateFlags".to_string(),
                "FileInfo" => "*mut ffi::GFileInfo".to_string(),
                "FileMeasureFlags" => "ffi::GFileMeasureFlags".to_string(),
                _ => {
                    // For other types, check if they're primitive types that need wrapping
                    if ident_str == "bool" {
                        "glib::ffi::gboolean".to_string()
                    } else {
                        // Keep the original type if no mapping found
                        return ty.clone();
                    }
                }
            };
            
            return syn::parse_str::<Type>(&ffi_type_str)
                .unwrap_or_else(|_| ty.clone());
        }
    }
    ty.clone()
}

/// Generate conversion code from FFI type to Rust type.
/// Returns (variable_name, conversion_code) for use in the wrapper.
fn generate_ffi_to_rust_conversion(param_name: &Ident, param_type: &Type) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    if let Type::Path(type_path) = param_type {
        if let Some(last_seg) = type_path.path.segments.last() {
            let ident_str = last_seg.ident.to_string();
            
            let conversion = match ident_str.as_str() {
                "GString" => {
                    quote! {
                        let #param_name: glib::GString = from_glib_none(#param_name);
                    }
                }
                "Option" => {
                    quote! {
                        let #param_name: Option<glib::GString> = from_glib_none(#param_name);
                    }
                }
                "FileQueryInfoFlags" => {
                    quote! {
                        let #param_name: FileQueryInfoFlags = from_glib(#param_name);
                    }
                }
                "FileCreateFlags" => {
                    quote! {
                        let #param_name: FileCreateFlags = from_glib(#param_name);
                    }
                }
                "FileInfo" => {
                    quote! {
                        let #param_name: FileInfo = from_glib_none(#param_name);
                    }
                }
                "FileMeasureFlags" => {
                    quote! {
                        let #param_name: FileMeasureFlags = from_glib(#param_name);
                    }
                }
                "bool" => {
                    quote! {
                        let #param_name: bool = from_glib(#param_name);
                    }
                }
                _ => return (quote! { #param_name }, quote! {}),
            };
            
            return (quote! { #param_name }, conversion);
        }
    }
    (quote! { #param_name }, quote! {})
}

/// Generates a pair of `unsafe extern "C"` FFI functions for the async/finish pattern.
///
/// Given `fn_name`, this generates:
/// - `{fn_name}_async<T: #impl_iface>(...)` — the async launcher
/// - `{fn_name}_finish(...)` — the finish callback
///
/// # Standard form
///
/// ```rust,ignore
/// #[gio_macros::async_finish]
/// async fn #func_basename#_async_result<T: #impl_iface>(imp: &T,  #param_name_1:  #param_type_1, ..., io_priority: i32) -> Result<#result_type, glib::Error> {
///     imp.#func_basename#_future(from_glib(#param_name_1), from_glib(io_priority))
/// }
/// ```
///
/// The `_async_result` suffix is stripped to derive the base name; `_async` and `_finish` FFI functions
/// are generated from it (e.g. `#func_basename#_async`, `#func_basename#_finish`).
///
/// The result type is inferred from `Result<#result_type, glib::Error>` in the return type.
///
/// The finish function returns `*mut <#result_type as ObjectType>::GlibType` (nullable pointer).
///
/// # Out-parameter form
///
/// ```rust,ignore
/// #[gio_macros::async_finish(out_param = true)]
/// async fn #func_basename#_async_result<T: #impl_iface>(imp: &T,  #param_name_1:  #param_type_1, ..., io_priority: i32) -> Result<#result_type, glib::Error> {
///     imp.#func_basename#_future(from_glib(#param_name_1), from_glib(io_priority))
/// }
/// ```
///
/// The finish function has an extra `out_ptr: *mut *mut <#result_type as ObjectType>::GlibType`
/// parameter and returns `glib::ffi::gboolean`.
///
/// The body can reference `imp` and `io_priority` which are provided by the generated code.
#[proc_macro_attribute]
pub fn async_finish(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = syn::parse_macro_input!(attr as AsyncFinishAttr);
    let has_out_param = attr.has_out_param;

    let input = syn::parse_macro_input!(item as AsyncFinishInput);

    let AsyncFinishInput {
        fn_name,
        generics,
        extra_params,
        rust_ret_type,
        result_expr,
    } = input;

    let (impl_generics, _ty_generics, where_clause) = generics.split_for_impl();

    // Extract object type from trait bound: <T: TypeImpl:> → Type
    let object_type = {
        let type_param = generics.type_params().next()
            .expect("expected a type parameter like `T: #impl_iface where #impl_iface: ObjectImpl + ObjectSubclass<Type: IsA<#object_type>> and #object_type: glib::object::ObjectType`");
        let first_bound = type_param
            .bounds
            .first()
            .expect("expected a trait bound on the type parameter");
        let trait_bound = match first_bound {
            TypeParamBound::Trait(tb) => tb,
            _ => panic!("expected a trait bound"),
        };
        let trait_ident = &trait_bound.path.segments.last().unwrap().ident;
        let trait_name = trait_ident.to_string();
        let type_name = trait_name
            .strip_suffix("Impl")
            .unwrap_or_else(|| panic!("trait bound `{trait_name}` must end with `Impl`"));
        Ident::new(type_name, trait_ident.span())
    };

    let fn_name_str = fn_name.to_string();
    let base_name = fn_name_str
        .strip_suffix("_async_result")
        .unwrap_or_else(|| panic!("function name `{fn_name_str}` must end with `_async_result`"));
    let async_fn_name = Ident::new(&format!("{base_name}_async"), Span::call_site());
    let finish_fn_name = Ident::new(&format!("{base_name}_finish"), Span::call_site());

    let param_names: Vec<&Ident> = extra_params.iter().map(|p| &p.name).collect();
    
    // Generate FFI types for the async wrapper function signature
    let ffi_param_types: Vec<Type> = extra_params.iter().map(|p| rust_to_ffi_type(&p.ty)).collect();
    
    // Generate conversions from FFI types to Rust types
    let param_conversions: Vec<proc_macro2::TokenStream> = extra_params.iter().map(|p| {
        let (_, conversion) = generate_ffi_to_rust_conversion(&p.name, &p.ty);
        conversion
    }).collect();
    
    // Always generate conversion for io_priority (from i32 to glib::Priority)
    let io_priority_conversion = quote! {
        let io_priority: glib::Priority = from_glib(io_priority);
    };

    let finish_fn = if has_out_param {
        quote! {
            unsafe extern "C" fn #finish_fn_name(
                _obj: *mut <#object_type as glib::object::ObjectType>::GlibType,
                res_ptr: *mut ffi::GAsyncResult,
                out_ptr: *mut *mut <#rust_ret_type as glib::object::ObjectType>::GlibType,
                error_ptr: *mut *mut glib::ffi::GError,
            ) -> glib::ffi::gboolean {
                unsafe {
                    let res: AsyncResult = from_glib_none(res_ptr);
                    let t = res.downcast::<LocalTask<#rust_ret_type>>().unwrap();
                    match t.propagate() {
                        Ok(val) => {
                            let ptr = val.to_glib_full();
                            if !out_ptr.is_null() {
                                *out_ptr = ptr;
                            }
                            glib::ffi::GTRUE
                        }
                        Err(e) => {
                            if !error_ptr.is_null() {
                                *error_ptr = e.into_glib_ptr();
                            }
                            glib::ffi::GFALSE
                        }
                    }
                }
            }
        }
    } else {
        quote! {
            unsafe extern "C" fn #finish_fn_name(
                _obj: *mut <#object_type as glib::object::ObjectType>::GlibType,
                res_ptr: *mut ffi::GAsyncResult,
                error_ptr: *mut *mut glib::ffi::GError,
            ) -> *mut <#rust_ret_type as glib::object::ObjectType>::GlibType {
                unsafe {
                    let res: AsyncResult = from_glib_none(res_ptr);
                    let t = res.downcast::<LocalTask<#rust_ret_type>>().unwrap();
                    match t.propagate() {
                        Ok(val) => val.to_glib_full(),
                        Err(e) => {
                            if !error_ptr.is_null() {
                                *error_ptr = e.into_glib_ptr();
                            }
                            Ptr::from::<()>(std::ptr::null_mut())
                        }
                    }
                }
            }
        }
    };

    let expanded = quote! {
        unsafe extern "C" fn #async_fn_name #impl_generics (
            obj: *mut <#object_type as glib::object::ObjectType>::GlibType,
            #( #param_names : #ffi_param_types, )*
            io_priority: i32,
            cancellable: *mut ffi::GCancellable,
            callback: ffi::GAsyncReadyCallback,
            user_data: glib::ffi::gpointer,
        ) #where_clause {
            let (imp, cancellable, t) = unsafe {
                let instance = &*(obj as *mut T::Instance);
                let imp = instance.imp();

                let cancellable: Option<Cancellable> = from_glib_none(cancellable);

                let closure = move |task: LocalTask<#rust_ret_type>, source_object: Option<&glib::Object>| {
                    let result: *mut ffi::GAsyncResult = task.upcast_ref::<AsyncResult>().to_glib_none().0;
                    let source_object: *mut glib::gobject_ffi::GObject = source_object.to_glib_none().0;
                    callback.unwrap()(source_object, result, user_data)
                };

                let t = LocalTask::new(
                    Some(imp.obj().upcast_ref::<glib::Object>()),
                    cancellable.as_ref(),
                    closure,
                );

                (imp, cancellable, t)
            };

            #( #param_conversions )*
            glib::MainContext::ref_thread_default().spawn_local(async move {
                #io_priority_conversion
                let res = #result_expr;
                t.return_result(res);
            });
        }

        #finish_fn
    };

    expanded.into()
}
