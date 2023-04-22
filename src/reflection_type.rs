use crate::binds::{MonoObject, MonoReflectionType, MonoType};
use crate::dimensions::Dim1D;
use crate::gc::{gc_unsafe_enter, gc_unsafe_exit, GCHandle};
use crate::{Array, Class, Domain, Image, Method};
use crate::{InteropClass, InteropRecive, InteropSend};
use crate::{Object, ObjectTrait};
use std::ffi::CString;
/// Rust representation of managed object derived form class `System.Type`
pub struct ReflectionType {
    #[cfg(not(feature = "referneced_objects"))]
    type_ptr: *mut MonoReflectionType,
    #[cfg(feature = "referneced_objects")]
    handle: GCHandle,
}
impl ReflectionType {
    /// Converts a class to a [`MonoReflectionType`]
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn from_class(class: &Class) -> Self {
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let ptr = unsafe { crate::binds::mono_class_get_type(class.get_ptr()) };
        let res = unsafe { Self::from_type_ptr(ptr).unwrap() }; // Converting class to ReflectionType should never fail
        #[cfg(feature = "referneced_objects")]
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
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let ptr = unsafe {
            #[allow(clippy::cast_possible_truncation)]
            crate::binds::mono_reflection_type_from_name(cstr.as_ptr() as *mut i8, img.get_ptr())
        };
        if ptr.is_null() {
            return None;
        }
        let res = unsafe { Self::from_type_ptr(ptr) };
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    // Unfinished
    #[allow(dead_code)]
    fn create_generic(gtype_img: Image, gtype: &str, gargs: &[Self]) -> Option<Self> {
        let garg_count = gargs.len();
        let gtype_str = format!("{gtype}`{garg_count}");
        println!("{gtype_str}");
        let Some(res) = Self::from_name(&gtype_str, gtype_img) else { return None };
        let arr: Array<Dim1D, Self> = gargs.into();
        let obj = res.cast::<Object>().unwrap();
        let res = MAKE_GENERIC_TYPE_MET.invoke(Some(obj), (arr,));
        // handle exceptions
        let res = match res {
            Ok(res) => res,
            Err(except_msg) => panic!("EXCEPTION:'{except_msg}'"), //return None,
        };
        // handle null
        let Some(res) = res else { return None };
        Object::cast::<Self>(&res)
    }
}
/*
impl InteropRecive for ReflectionType {
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
        #[cfg(not(feature = "referneced_objects"))]
        {
            self.type_ptr.cast()
        }
        #[cfg(feature = "referneced_objects")]
        {
            self.handle.get_target()
        }
    }
    unsafe fn from_ptr_unchecked(type_ptr: *mut MonoObject) -> Self {
        #[cfg(not(feature = "referneced_objects"))]
        {
            Self {
                type_ptr: type_ptr.cast(),
            }
        }
        #[cfg(feature = "referneced_objects")]
        {
            Self {
                handle: GCHandle::create_default(type_ptr.cast()),
            }
        }
    }
}
use lazy_static::lazy_static;
lazy_static! {
    static ref TYPE_CLASS: Class = {
        let img = crate::Assembly::assembly_loaded("mscorlib")
            .expect("Assembly mscorlib not loaded, could not get System.Type class!")
            .get_image();
        Class::from_name_case(&img, "System", "Type")
            .expect("Could not get System.Type class form mscorlib!")
    };
    static ref MAKE_GENERIC_TYPE_MET: Method<(Array<Dim1D, ReflectionType>,)> = {
        let img = crate::Assembly::assembly_loaded("mscorlib")
            .expect("Assembly mscorlib not loaded, could not get System.Type class!")
            .get_image();
        let class = Class::from_name_case(&img, "System", "Type")
            .expect("Could not get System.Type class form mscorlib!");
        Method::get_from_name(&class, "MakeGenericType", 1)
            .expect("Could not get System.Type::MakeGenericType method!")
    };
}
impl Clone for ReflectionType {
    fn clone(&self) -> Self {
        unsafe { Self::from_ptr(self.get_ptr()) }.unwrap()
    }
}
