use glib::{prelude::*, subclass::prelude::*, translate::*};

use crate::{ffi, FileMonitor};

pub trait FileMonitorImpl: ObjectImpl + FileMonitorImplExt {
    fn cancel(&self) -> bool {
        self.parent_cancel()
    }
}

pub trait FileMonitorImplExt: ObjectSubclass {
    fn parent_cancel(&self) -> bool;
}

impl<T: FileMonitorImpl> FileMonitorImplExt for T {
    fn parent_cancel(&self) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *const ffi::GFileMonitorClass;

            let f = (*parent_class)
                .cancel
                .expect("No parent class implementation for \"cancel\"");

            let res = f(self.obj().unsafe_cast_ref::<FileMonitor>().to_glib_none().0);
            from_glib(res)
        }
    }
}

unsafe impl<T: FileMonitorImpl> IsSubclassable<T> for FileMonitor {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.cancel = Some(cancel::<T>);
    }
}

unsafe extern "C" fn cancel<T: FileMonitorImpl>(
    monitor: *mut ffi::GFileMonitor,
) -> glib::ffi::gboolean {
    let instance = &*(monitor as *mut T::Instance);
    let imp = instance.imp();

    let res = imp.cancel();

    res.into_glib()
}
