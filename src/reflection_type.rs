use crate::binds::{MonoClass, MonoObject, MonoReflectionType, MonoType};
use crate::dimensions::Dim1D;
use crate::gc::{gc_unsafe_enter, gc_unsafe_exit, GCHandle};
use crate::{Array, Class, Domain, Image, Method};
use crate::{InteropClass, InteropReceive, InteropSend};
use crate::{Object, ObjectTrait};
use std::ffi::CString;
use std::sync::LazyLock;
/// Rust representation of managed object derived form class `System.Type`
pub struct ReflectionType {
    #[cfg(not(feature = "referenced_objects"))]
    type_ptr: *mut MonoReflectionType,
    #[cfg(feature = "referenced_objects")]
    handle: GCHandle,
}
impl ReflectionType {
    /// Converts a class to a [`MonoReflectionType`]
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn from_class(class: &Class) -> Self {
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let ptr = unsafe { crate::binds::mono_class_get_type(class.get_ptr()) };
        let res = unsafe { Self::from_type_ptr(ptr).unwrap() }; // Converting class to ReflectionType should never fail
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns a pointer to unmanaged representation of `System.Type`
    #[must_use]
    pub fn get_type_ptr(&self) -> *mut MonoType {
        unsafe { crate::binds::mono_reflection_type_get_type(self.get_ptr().cast()) }
    }
    /// Creates an new instance from a pointer to unmanaged representation of `System.Type`
    /// # Safety
    /// The pointer must be either a pointer to valid *mut [`MonoType`] or null.
    pub unsafe fn from_type_ptr(type_ptr: *mut MonoType) -> Option<Self> {
        if type_ptr.is_null() {
            return None;
        }
        let dom = Domain::get_current()
            .expect("Can't convert *MonoType to ReflecionType before JIT started.");
        Some(
            unsafe {
                Self::from_ptr(crate::binds::mono_type_get_object(dom.get_ptr(), type_ptr).cast())
            }
            .expect("Could not convert MonoType pointer to a ReflectionType!"),
        )
    }
    /// Gets type with *name* inside image *img*
    #[must_use]
    pub fn from_name(name: &str, img: Image) -> Option<Self> {
        let cstr = CString::new(name).expect("Could not convert string to CString");
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let ptr = unsafe {
            #[allow(clippy::cast_possible_truncation)]
            crate::binds::mono_reflection_type_from_name(cstr.as_ptr() as *mut i8, img.get_ptr())
        };
        if ptr.is_null() {
            return None;
        }
        let res = unsafe { Self::from_type_ptr(ptr) };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        res
    }
}
/*
impl InteropReceive for ReflectionType {
    type SourceType = *mut MonoReflectionType;
    // unless this function is abused, this argument should come from the mono runtime, so it should be always valid.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn get_rust_rep(rarg: Self::SourceType) -> Self {
        unsafe { Self::from_ptr(rarg.cast()).expect("Recived null on a not nullable type") }
    }
}*/
impl InteropClass for ReflectionType {
    fn get_mono_class() -> Class {
        *TYPE_CLASS
    }
}
impl From<Class> for ReflectionType {
    fn from(class: Class) -> Self {
        Self::from_class(&class)
    }
}
impl ObjectTrait for ReflectionType {
    fn get_ptr(&self) -> *mut MonoObject {
        #[cfg(not(feature = "referenced_objects"))]
        {
            self.type_ptr.cast()
        }
        #[cfg(feature = "referenced_objects")]
        {
            self.handle.get_target()
        }
    }
    unsafe fn from_ptr_unchecked(type_ptr: *mut MonoObject) -> Self {
        #[cfg(not(feature = "referenced_objects"))]
        {
            Self {
                type_ptr: type_ptr.cast(),
            }
        }
        #[cfg(feature = "referenced_objects")]
        {
            Self {
                handle: GCHandle::create_default(type_ptr.cast()),
            }
        }
    }
}

static TYPE_CLASS: LazyLock<Class> = LazyLock::new(|| {
    let img = crate::Assembly::assembly_loaded("mscorlib")
        .expect("Assembly mscorlib not loaded, could not get System.Type class!")
        .get_image();
    Class::from_name_case(&img, "System", "Type")
        .expect("Could not get System.Type class form mscorlib!")
});

impl Clone for ReflectionType {
    fn clone(&self) -> Self {
        unsafe { Self::from_ptr(self.get_ptr()) }.unwrap()
    }
}
