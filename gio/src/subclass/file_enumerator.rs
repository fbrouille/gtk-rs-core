use glib::{prelude::*, subclass::prelude::*, translate::*, Error};

use crate::{ffi, Cancellable, FileEnumerator, FileInfo};

pub trait FileEnumeratorImpl: ObjectImpl + FileEnumeratorImplExt {
    fn next_file(&self, cancellable: Option<&Cancellable>) -> Result<Option<FileInfo>, Error> {
        self.parent_next_file(cancellable)
    }

    fn close(&self, cancellable: Option<&Cancellable>) -> Result<bool, Error> {
        self.parent_close(cancellable)
    }
}

pub trait FileEnumeratorImplExt: ObjectSubclass {
    fn parent_next_file(
        &self,
        cancellable: Option<&Cancellable>,
    ) -> Result<Option<FileInfo>, Error>;
    fn parent_close(&self, cancellable: Option<&Cancellable>) -> Result<bool, Error>;
}

impl<T: FileEnumeratorImpl> FileEnumeratorImplExt for T {
    fn parent_next_file(
        &self,
        cancellable: Option<&Cancellable>,
    ) -> Result<Option<FileInfo>, Error> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *const ffi::GFileEnumeratorClass;

            let f = (*parent_class)
                .next_file
                .expect("No parent class implementation for \"next_file\"");

            let mut error = std::ptr::null_mut();
            let res = f(
                self.obj()
                    .unsafe_cast_ref::<FileEnumerator>()
                    .to_glib_none()
                    .0,
                cancellable.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(res))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn parent_close(&self, cancellable: Option<&Cancellable>) -> Result<bool, Error> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *const ffi::GFileEnumeratorClass;

            let f = (*parent_class)
                .close_fn
                .expect("No parent class implementation for \"close_fn\"");

            let mut error = std::ptr::null_mut();
            let res = f(
                self.obj()
                    .unsafe_cast_ref::<FileEnumerator>()
                    .to_glib_none()
                    .0,
                cancellable.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib(res))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

unsafe impl<T: FileEnumeratorImpl> IsSubclassable<T> for FileEnumerator {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.next_file = Some(next_file::<T>);
        klass.close_fn = Some(close_fn::<T>);
    }
}

unsafe extern "C" fn next_file<T: FileEnumeratorImpl>(
    enumerator: *mut ffi::GFileEnumerator,
    cancellable: *mut ffi::GCancellable,
    error: *mut *mut glib::ffi::GError,
) -> *mut ffi::GFileInfo {
    let instance = &*(enumerator as *mut T::Instance);
    let imp = instance.imp();
    let cancellable = Option::<Cancellable>::from_glib_none(cancellable);

    let res = imp.next_file(cancellable.as_ref());

    match res {
        Ok(fileinfo) => fileinfo.to_glib_full(),
        Err(err) => {
            if !error.is_null() {
                *error = err.to_glib_full()
            }
            std::ptr::null_mut()
        }
    }
}

unsafe extern "C" fn close_fn<T: FileEnumeratorImpl>(
    enumerator: *mut ffi::GFileEnumerator,
    cancellable: *mut ffi::GCancellable,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(enumerator as *mut T::Instance);
    let imp = instance.imp();
    let cancellable = Option::<Cancellable>::from_glib_none(cancellable);

    let res = imp.close(cancellable.as_ref());

    match res {
        Ok(res) => res.into_glib(),
        Err(err) => {
            if !error.is_null() {
                *error = err.to_glib_full()
            }
            false.into_glib()
        }
    }
}
