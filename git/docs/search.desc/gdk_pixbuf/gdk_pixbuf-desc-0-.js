searchState.loadedDescShard("gdk_pixbuf", 0, "Rust GDK-PixBuf bindings\nA bad option was passed to a pixbuf save module.\nA bilevel clipping mask (black and white) will be created …\nBest quality/speed balance; use this mode by default. …\nRotate by 270 degrees.\nThis enumeration defines the color spaces that are …\nAn image file was broken somehow.\nRotate by 90 degrees.\nGeneric failure code, something went wrong.\nFor now falls back to #GDK_PIXBUF_ALPHA_BILEVEL. In the …\nThis is the slowest and highest quality reconstruction …\nOnly part of the animation was loaded.\nNot enough memory.\nInterpolation modes for scaling functions.\nNearest neighbor sampling; this is the fastest and lowest …\nNo rotation.\nA pixel buffer.\nControl the alpha channel for drawables.\nAn opaque object representing an animation.\nAn opaque object representing an iterator which points to a\nAn error code in the <code>GDK_PIXBUF_ERROR</code> domain.\nA <code>PixbufFormat</code> contains information about the image format …\nFlags which allow a module to specify further details …\nIncremental image loader.\nImplements\nThe possible rotations which can be passed to …\nAn opaque struct representing a simple animation.\nIndicates a red/green/blue additive color space.\nthe image format is scalable\nthe module is threadsafe. gdk-pixbuf ignores modules that …\nThis is an accurate simulation of the PostScript image …\nUnknown image type.\nDon’t know how to perform the given operation on the …\nRotate by 180 degrees.\nthe module can write out images in the format.\nTakes an existing pixbuf and adds an alpha channel to it.\nAdds a new frame to @self. The @pixbuf must have the …\nPossibly advances an animation to a new frame.\nGet a flags value with all known bits set.\nTakes an existing pixbuf and checks for the presence of an …\nReturn the inner pointer to the underlying C value.\nThe bitwise and (<code>&amp;</code>) of the bits in two flags values.\nThe bitwise and (<code>&amp;</code>) of the bits in two flags values.\nThe bitwise or (<code>|</code>) of the bits in two flags values.\nThe bitwise or (<code>|</code>) of the bits in two flags values.\nGet the underlying bits value.\nQueries the number of bits per color sample in a pixbuf.\nThe bitwise exclusive-or (<code>^</code>) of the bits in two flags …\nThe bitwise exclusive-or (<code>^</code>) of the bits in two flags …\nReturns the length of the pixel data, in bytes.\nCalculates the rowstride that an image created with those …\nMakes a clone of this shared reference.\nMakes a clone of this shared reference.\nMakes a clone of this shared reference.\nMakes a clone of this shared reference.\nMakes a clone of this shared reference.\nCopies the boxed type with the type-specific copy function.\nMakes a clone of this shared reference.\nComparison for two GObjects.\nComparison for two GObjects.\nComparison for two GObjects.\nComparison for two GObjects.\nComparison for two GObjects.\nComparison for two GObjects.\nQueries the color space of a pixbuf.\nThe bitwise negation (<code>!</code>) of the bits in a flags value, …\nCreates a transformation of the source image @self by …\nCreates a transformation of the source image @self by …\nCreates a new pixbuf by scaling <code>src</code> to <code>dest_width</code> x …\nWhether all set bits in a source flags value are also set …\nCopies a rectangular area from <code>src_pixbuf</code> to <code>dest_pixbuf</code>.\nCopies the key/value pair options attached to a <code>Pixbuf</code> to …\nGets the number of milliseconds the current pixbuf should …\nReturns a description of the format.\nThe intersection of a source flags value with the …\nGet a flags value with all bits unset.\nEquality for two GObjects.\nEquality for two GObjects.\nEquality for two GObjects.\nEquality for two GObjects.\nEquality for two GObjects.\nEquality for two GObjects.\nThe bitwise or (<code>|</code>) of the bits in each flags value.\nReturns the filename extensions typically used for files …\nParses an image file far enough to determine its format …\nAsynchronously parses an image file far enough to …\nClears a pixbuf to the given RGBA value, converting the …\nFlips a pixbuf horizontally or vertically and returns the …\nObtains the available information about the image formats …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConvert from a bits value.\nConvert from a bits value exactly.\nConvert from a bits value, unsetting any unknown bits.\nCreates a new #GdkPixbuf out of in-memory readonly image …\nCreates a new pixbuf by loading an image from a file.\nCreates a new animation by loading it from a file.\nCreates a new pixbuf by loading an image from a file.\nCreates a new pixbuf by loading an image from a file.\nBorrows the underlying C value.\nBorrows the underlying C value mutably.\nThe bitwise or (<code>|</code>) of the bits in each flags value.\nGet a flags value with the bits of a flag with the given …\nCreates a <code>Pixbuf</code> from a type implementing <code>Read</code> (like <code>File</code>).\nCreates a new pixbuf by loading an image from an resource.\nCreates a new pixbuf animation by loading an image from an …\nCreates a new pixbuf by loading an image from an resource.\nCreates a new pixbuf by loading an image from an input …\nCreates a new animation by loading it from an input stream.\nCreates a new animation by asynchronously loading an image …\nCreates a new pixbuf by loading an image from an input …\nCreates a new pixbuf by parsing XPM data in memory.\nQueries whether a pixbuf has an alpha channel (opacity …\nHashes the memory address of this object.\nHashes the memory address of this object.\nHashes the memory address of this object.\nHashes the memory address of this object.\nHashes the memory address of this object.\nHashes the memory address of this object.\nQueries the height of a pixbuf.\nInitalizes the gdk-pixbuf loader modules referenced by the …\nThe bitwise or (<code>|</code>) of the bits in two flags values.\nThe bitwise and (<code>&amp;</code>) of the bits in two flags values.\nWhether any set bits in a source flags value are also set …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nWhether all known bits in this flags value are set.\nReturns whether this image format is disabled.\nWhether all bits in this flags value are unset.\nGets whether @self should loop indefinitely when it …\nReturns <code>TRUE</code> if the save option specified by @option_key …\nReturns whether this image format is scalable.\nReturns whether pixbufs can be saved in the given format.\nYield a set of contained flags values.\nYield a set of contained named flags values.\nReturns information about the license of the image loader …\nReturns the mime types supported by the format.\nQueries the number of channels of a pixbuf.\nReturns the name of the format.\nCreates a new <code>Pixbuf</code> structure and allocates a buffer for …\nCreates a new pixbuf loader object.\nCreates a new, empty animation.\nCreates a new pixbuf which represents a sub-region of …\nThe bitwise negation (<code>!</code>) of the bits in a flags value, …\nUsed to determine how to respond to the area_updated …\nLooks up @key in the list of options that may have been …\nPartial comparison for two GObjects.\nPartial comparison for two GObjects.\nPartial comparison for two GObjects.\nPartial comparison for two GObjects.\nPartial comparison for two GObjects.\nPartial comparison for two GObjects.\nGets the current pixbuf which should be displayed.\nReturns a mutable slice to the pixbuf’s pixel data.\nTraits intended for blanket imports.\nProvides a #GBytes buffer containing the raw pixel data; …\nThe intersection of a source flags value with the …\nRemoves the key/value pair option attached to a <code>Pixbuf</code>.\nRotates a pixbuf by a multiple of 90 degrees, and returns …\nQueries the rowstride of a pixbuf, which is the number of …\nModifies saturation and optionally pixelates <code>src</code>, placing …\nVector version of <code>gdk_pixbuf_save_to_buffer()</code>.\nSaves <code>pixbuf</code> to an output stream.\nSaves <code>pixbuf</code> to an output stream asynchronously.\nVector version of <code>gdk_pixbuf_save()</code>.\nCreates a transformation of the source image @self by …\nCreate a new pixbuf containing a copy of <code>src</code> scaled to …\nCall <code>insert</code> when <code>value</code> is <code>true</code> or <code>remove</code> when <code>value</code> is …\nDisables or enables an image format.\nSets whether @self should loop indefinitely when it …\nAttaches a key/value pair as an option to a <code>Pixbuf</code>.\nThe intersection of a source flags value with the …\nThe intersection of a source flags value with the …\nTraits intended for creating custom types.\nThe bitwise exclusive-or (<code>^</code>) of the bits in two flags …\nThe bitwise exclusive-or (<code>^</code>) of the bits in two flags …\nThe bitwise or (<code>|</code>) of the bits in two flags values.\nQueries the width of a pixbuf.\nCreates a new pixbuf loader object that always attempts to …\nCreates a new pixbuf loader object that always attempts to …\nTrait containing all <code>PixbufAnimation</code> methods.\nTrait containing all <code>PixbufLoader</code> methods.\nQueries the #GdkPixbufAnimation that a pixbuf loader is …\nQueries the #GdkPixbufAnimation that a pixbuf loader is …\nInforms a pixbuf loader that no further writes with …\nInforms a pixbuf loader that no further writes with …\nThis signal is emitted when the pixbuf loader has …\nThis signal is emitted when the pixbuf loader has …\nThis signal is emitted when a significant area of the …\nThis signal is emitted when a significant area of the …\nThis signal is emitted when gdk_pixbuf_loader_close() is …\nThis signal is emitted when gdk_pixbuf_loader_close() is …\nThis signal is emitted when the pixbuf loader has been fed …\nThis signal is emitted when the pixbuf loader has been fed …\nObtains the available information about the format of the …\nObtains the available information about the format of the …\nQueries the height of the bounding box of a pixbuf …\nQueries the height of the bounding box of a pixbuf …\nChecks whether the animation is a static image.\nChecks whether the animation is a static image.\nGet an iterator for displaying an animation.\nGet an iterator for displaying an animation.\nQueries the #GdkPixbuf that a pixbuf loader is currently …\nQueries the #GdkPixbuf that a pixbuf loader is currently …\nCauses the image to be scaled while it is loaded.\nCauses the image to be scaled while it is loaded.\nRetrieves a static image for the animation.\nRetrieves a static image for the animation.\nQueries the width of the bounding box of a pixbuf …\nQueries the width of the bounding box of a pixbuf …\nParses the next <code>count</code> bytes in the given image buffer.\nParses the next <code>count</code> bytes in the given image buffer.\nParses the next contents of the given image buffer.\nParses the next contents of the given image buffer.\nTraits intended for subclassing <code>PixbufAnimation</code>.\nTraits intended for subclassing <code>PixbufAnimationIter</code>.\nTraits intended for subclassing <code>PixbufLoader</code>.\nChecks whether the animation is a static image.\nGet an iterator for displaying an animation.\nfills @width and @height with the frame size of the …\nRetrieves a static image for the animation.\nPossibly advances an animation to a new frame.\nTime in milliseconds, returning <code>None</code> implies showing the …\nUsed to determine how to respond to the area_updated …\nGets the current pixbuf which should be displayed.\nIf this subclass is an abstract class or not.\nTrait for defining boxed types.\nThe C class struct.\nTrait implemented by structs that implement a <code>GObject</code> C …\nTrait containing only the property related functions of …\nThe inner type\nThe C instance struct.\nTrait implemented by structs that implement a <code>GObject</code> C …\nTrait implemented by any type implementing <code>InstanceStruct</code> …\nThe C class struct.\nTrait implemented by structs that implement a …\nList of interfaces implemented by this type.\nTrait for implementable interfaces.\nTrait for subclassable class structs.\n<code>GObject</code> type name.\n<code>GObject</code> type name.\nBoxed type name.\nShared type name.\nExtension trait for <code>glib::Object</code>’s class struct.\nTrait for implementors of <code>glib::Object</code> subclasses.\nThe central trait for defining a <code>GObject</code> interface.\nType methods required for an <code>ObjectInterface</code> …\nThe central trait for subclassing a <code>GObject</code> type.\nExtension methods for all <code>ObjectSubclass</code> impls.\nTrait implemented by any type implementing <code>ObjectSubclassIs</code>…\nType methods required for an <code>ObjectSubclass</code> implementation.\nParent Rust type to inherit from.\nPrerequisites for this interface.\nThe inner refcounted type\nTrait for defining shared types.\nCorresponding object subclass type for this instance …\nCorresponding object subclass type for this class struct.\nCorresponding object interface type for this class struct.\nWrapper around this subclass defined with <code>wrapper!</code>\nActivates the application.\ninvoked on the primary instance after ‘activate’, ‘…\nProvides access to a raw pointer to InnerType\ninvoked on the primary instance before ‘activate’, ‘…\nReturns the class struct for this specific instance.\nOverride the vfuncs of all parent types.\nOverride the virtual methods of this class for the given …\nClass initialization.\ninvoked on the primary instance when a command-line is not …\nConstructed.\nProperties installed for this type.\nSimilar to <code>ObjectImpl</code> but auto-generated by the <code>Properties</code> …\nSimilar to <code>ObjectImpl</code> but auto-generated by the <code>Properties</code> …\nemits property change notification for a bunch of …\nDisposes of the object.\nForces a write of all user-space buffered data for the …\nReturns the implementation from an instance.\nGet interface from an instance.\nReturns the implementation from an instance.\nGet interface from an instance.\nConverts a raw pointer to InnerType to a RefCounted object\nConstructs a SharedType from a RefCountedType\ninvoked locally after the parsing of the commandline …\nReturns the implementation for from this instance struct, …\nReturns the implementation (the private Rust struct) of …\nGets the input stream for this object. This is used for …\nReturns the corresponding object instance.\nReturns a pointer to the instance implementation specific …\nInstance specific initialization.\nInstance specific initialization.\nInstance specific initialization.\nPerforms additional instance initialization.\nSet up default implementations for interface vfuncs.\nOverride the virtual methods of this interface for the …\nInterface initialization.\nConverts the RefCounted object to a raw pointer to …\nConverts the SharedType into its inner RefCountedType\nReturns the “level” (i.e. the originating protocol) of …\nLoads the module, registers one or more object subclasses …\nThis virtual function is always invoked in the local …\nConstructor.\nFunction to be called when property change is notified for …\nReturns the corresponding object instance.\nOpens the given files.\nGets the output stream for this object. This is used for …\nChain up to the parent class’ implementation of …\nChain up to the parent class’ implementation of …\nChain up to the parent class’ implementation of …\nProperties installed for this interface.\nProperties installed for this type.\nProperty getter.\nUsed to be invoked on the primary instance when the use …\nThe function used to increment the inner type refcount\nReturns a new reference-counted wrapper around <code>self</code>.\nUsed to be invoked on the primary instance from …\nConverts the data in the message to bytes placed in the …\nProperty setter.\ninvoked only on the registered primary instance immediately\nChain up to parent class signal handler.\nSignals installed for this interface.\nSignals installed for this type.\nReturns the space required for the control message, not …\nTries to skip @count bytes from the stream. Will block …\nSplices an input stream into an output stream.\ninvoked on the primary instance immediately after …\nReturns the <code>glib::Type</code> ID of the subclass.\nReturns the <code>glib::Type</code> ID of the interface.\nStorage for the type-specific data used during …\nAdditional type initialization.\nAdditional type initialization.\nUnloads the module (see <code>TypeModuleExt::unuse</code>).\nConstructor.")