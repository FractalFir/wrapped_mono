use crate::binds::MonoString;
use crate::domain::Domain;
use core::ptr::null_mut;
use std::ffi::CString;
use crate::object::Object;
use crate::invokable::{InvokePass,InvokeReturn};
///Representaiton of [`Object`] of type **System.String**. 
pub struct MString{
    s_ptr:*mut MonoString,
} 
impl MString{
    ///Creates new managed **String** in *domain* with content of *string*.
    pub fn new(domain:&Domain,string:&str)->Self{
        let cstr = CString::new(string).expect("Could not create CString!");
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_string_new(domain.get_ptr(),cstr.as_ptr())

        )}.expect("Could not create a new CString!");
        drop(cstr);
        return res;
    }
    ///Converts [`MString`] to [`String`]  
    pub fn to_string(&self)->String{
        let cstr = unsafe{CString::from_raw(crate::binds::mono_string_to_utf8(self.s_ptr))};
        let s = cstr.to_str().expect("Colud not create String!").to_owned();
        unsafe{crate::binds::mono_free(cstr.into_raw() as *mut std::os::raw::c_void)};
        return s;
    }
    ///Compares two managed strings. Returns true if their **content** is equal, not if they are the same **object**.
    pub fn is_equal(&self,other:&Self)->bool{
        return unsafe{crate::binds::mono_string_equal(self.s_ptr,other.s_ptr) != 0};
    }
    ///Creates hash of a [`String`].
    pub fn hash(&self)->u32{
        return unsafe{crate::binds::mono_string_hash(self.s_ptr)};
    }
    //Cretes [`MString`] form pointer , or returns None if pointer equal to null.
    /// # Safety
    /// *ptr* must be either a valid [`MonoString`] pointer or null. Pasing any other value will lead to undefined behaviour.
    pub fn from_ptr(ptr:*mut MonoString)->Option<Self>{
        if ptr == null_mut(){
            return None;
        }
        else{
            return Some(Self{s_ptr:ptr});
        }
    }
    pub fn get_ptr(&self)->*mut MonoString{
        return self.s_ptr;
    }
}
impl InvokePass for MString{
    type SourceType = *mut MonoString;
    fn get_rust_rep(src:Self::SourceType)->Self{
        return unsafe{Self::from_ptr(src)}.expect("got null in a non-nullable string. For nullabe support use Option<MString>");
    }
}
impl InvokePass for Option<MString>{
    type SourceType = *mut MonoString;
    fn get_rust_rep(src:Self::SourceType)->Self{
        return unsafe{MString::from_ptr(src)};
    }
}
impl InvokeReturn for MString{
    type ReturnType = *mut MonoString;
    fn get_mono_rep(src:Self)->Self::ReturnType{
        return src.s_ptr;
    }
}
impl InvokeReturn for Option<MString>{
    type ReturnType = *mut MonoString;
    fn get_mono_rep(src:Self)->Self::ReturnType{
        match src{
            Some(src)=>return src.s_ptr,
            None=>return null_mut(),
        }
    }
}