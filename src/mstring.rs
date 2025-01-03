use crate::binds::MonoString;
use crate::domain::Domain;
use crate::gc::{gc_unsafe_enter, gc_unsafe_exit, GCHandle};
use crate::interop::InteropClass;
///needed for docs
#[allow(unused_imports)]
use crate::object::Object;
use crate::Class;
use crate::ObjectTrait;
use std::ffi::CString;
#[warn(unused_imports)]
///Representaiton of [`Object`] of type **System.String**.
pub struct MString {
    #[cfg(not(feature = "referenced_objects"))]
    s_ptr: *mut MonoString,
    #[cfg(feature = "referenced_objects")]
    handle: GCHandle,
}
impl MString {
    ///Creates new managed **String** in *domain* with content of *string*.
    #[must_use]
    pub fn new(domain: &Domain, string: &str) -> Self {
        let cstr = CString::new(string).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr(crate::binds::mono_string_new(domain.get_ptr(), cstr.as_ptr()).cast())
        }
        .expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        drop(cstr);
        res
    }
    ///Compares two managed strings. Returns true if their **content** is equal, not if they are the same **object**.
    #[must_use]
    pub fn is_equal(&self, other: &Self) -> bool {
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let equ = unsafe {
            crate::binds::mono_string_equal(self.get_ptr().cast(), other.get_ptr().cast()) != 0
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        equ
    }
    ///Creates hash of a [`String`].
    #[must_use]
    pub fn hash(&self) -> u32 {
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let hsh = unsafe { crate::binds::mono_string_hash(self.get_ptr().cast()) };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        hsh
    }
}
impl InteropClass for MString {
    fn get_mono_class() -> Class {
        Class::get_string()
    }
}
impl ToString for MString {
    ///Converts [`MString`] to [`String`]  
    fn to_string(&self) -> String {
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let cstr = unsafe {
            CString::from_raw(crate::binds::mono_string_to_utf8(
                self.get_ptr().cast::<MonoString>(),
            ))
        };
        let res = cstr.to_str().expect("Colud not create String!").to_owned();
        unsafe { crate::binds::mono_free(cstr.into_raw().cast::<std::os::raw::c_void>()) };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        res
    }
}
use crate::binds::MonoObject;
use crate::Exception;
impl ObjectTrait for MString {
    #[must_use]
    fn get_ptr(&self) -> *mut MonoObject {
        #[cfg(not(feature = "referenced_objects"))]
        {
            self.s_ptr.cast()
        }
        #[cfg(feature = "referenced_objects")]
        {
            self.handle.get_target()
        }
    }
    /// Creates [`MString`] form pointer , or returns [`None`] if pointer equal to null.
    /// # Safety
    /// *ptr* must be either a valid [`MonoString`] pointer or null. Pasing any other value will lead to undefined behaviour.
    unsafe fn from_ptr_unchecked(ptr: *mut MonoObject) -> Self {
        #[cfg(not(feature = "referenced_objects"))]
        {
            Self { s_ptr: ptr.cast() }
        }
        #[cfg(feature = "referenced_objects")]
        {
            Self {
                handle: GCHandle::create_default(ptr.cast::<MonoObject>()),
            }
        }
    }
    fn to_mstring(&self) -> Result<Option<MString>, Exception> {
        Ok(Some(self.clone()))
    }
}
impl Clone for MString {
    fn clone(&self) -> Self {
        unsafe { Self::from_ptr(self.get_ptr()).unwrap() } //If object exists then it can't be null
    }
}
impl<O: ObjectTrait> PartialEq<O> for MString {
    fn eq(&self, other: &O) -> bool {
        self.get_ptr().cast() == other.get_ptr()
    }
}
