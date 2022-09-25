use crate::binds::MonoString;
use crate::domain::Domain;
use core::ptr::null_mut;
use std::ffi::CString;
use crate::interop::{InteropRecive,InteropSend,InteropClass};
use crate::Class;
///needed for docs
#[allow(unused_imports)]
use crate::object::Object;
#[warn(unused_imports)]
///Representaiton of [`Object`] of type **System.String**. 
pub struct MString{
    s_ptr:*mut MonoString,
} 
impl MString{
    ///Creates new managed **String** in *domain* with content of *string*.
    pub fn new(domain:&Domain,string:&str)->Self{
        let cstr = CString::new(string).expect(crate::STR2CSTR_ERR);
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_string_new(domain.get_ptr(),cstr.as_ptr())

        )}.expect(crate::STR2CSTR_ERR);
        drop(cstr);
        res
    }
    ///Cast [`Object`] to [`String`]. Returns [`None`] if cast failed. 
    pub fn cast_from_object(obj:&Object)->Option<MString>{
        use crate::object::ObjectTrait;
        if obj.get_class() != Class::get_string(){
            return None;
        }
        Some(Self{s_ptr:obj.get_ptr() as *mut MonoString})
    }
    ///Compares two managed strings. Returns true if their **content** is equal, not if they are the same **object**.
    pub fn is_equal(&self,other:&Self)->bool{
        unsafe{crate::binds::mono_string_equal(self.s_ptr,other.s_ptr) != 0}
    }
    ///Creates hash of a [`String`].
    pub fn hash(&self)->u32{
        unsafe{crate::binds::mono_string_hash(self.s_ptr)}
    }
    //Cretes [`MString`] form pointer , or returns None if pointer equal to null.
    /// # Safety
    /// *ptr* must be either a valid [`MonoString`] pointer or null. Pasing any other value will lead to undefined behaviour.
    pub unsafe fn from_ptr(ptr:*mut MonoString)->Option<Self>{
        if ptr.is_null(){
            None
        }
        else{
            Some(Self{s_ptr:ptr})
        }
    }
    pub fn get_ptr(&self)->*mut MonoString{
        self.s_ptr
    }
    ///Returns this [`MString`] as [`Object`]. Both original and return value still reference the same managed object.
    pub fn to_object(&self)->Object{
        unsafe{Object::from_ptr(self.s_ptr as *mut crate::binds::MonoObject)}.expect("Impossible condition reached! object null and not null at the same time!")
    }
}
impl InteropRecive for MString{
    type SourceType = *mut MonoString;
    fn get_rust_rep(src:Self::SourceType)->Self{
        use crate::exception::ExceptManaged;
        let opt = unsafe{Self::from_ptr(src)};
        <MString as ExceptManaged<MString>>::expect_managed_arg(opt,"got null in a non-nullable string. For nullabe support use Option<MString>")
    }
}
impl InteropRecive for Option<MString>{
    type SourceType = *mut MonoString;
    fn get_rust_rep(src:Self::SourceType)->Self{
        unsafe{MString::from_ptr(src)}
    }
}
impl InteropSend for MString{
    type TargetType = *mut MonoString;
    fn get_mono_rep(src:Self)->Self::TargetType{
        src.s_ptr
    }
}
impl InteropSend for Option<MString>{
    type TargetType = *mut MonoString;
    fn get_mono_rep(src:Self)->Self::TargetType{
        match src{Some(src)=>src.s_ptr,None=>null_mut()}
    }
}
impl InteropClass for MString{
    fn get_mono_class()->Class{
        Class::get_string()
    }
}
impl ToString for MString{
    ///Converts [`MString`] to [`String`]  
    fn to_string(&self)->String{
        let cstr = unsafe{CString::from_raw(crate::binds::mono_string_to_utf8(self.s_ptr))};
        let res = cstr.to_str().expect("Colud not create String!").to_owned();
        unsafe{crate::binds::mono_free(cstr.into_raw() as *mut std::os::raw::c_void)};
        res
    }
}