use crate::binds::MonoString;
use crate::domain::Domain;
use core::ptr::null_mut;
use std::ffi::CString;
use crate::interop::{InteropRecive,InteropSend,InteropClass};
use crate::Class;
use crate::gc::GCHandle;
use crate::ObjectTrait;

///needed for docs
#[allow(unused_imports)]
use crate::object::Object;
#[warn(unused_imports)]
///Representaiton of [`Object`] of type **System.String**. 
pub struct MString{
    #[cfg(not(feature = "referneced_objects"))]
    s_ptr:*mut MonoString,
    #[cfg(feature = "referneced_objects")]
    handle:GCHandle,
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
    ///Compares two managed strings. Returns true if their **content** is equal, not if they are the same **object**.
    pub fn is_equal(&self,other:&Self)->bool{
        unsafe{crate::binds::mono_string_equal(self.get_ptr(),other.get_ptr()) != 0}
    }
    ///Creates hash of a [`String`].
    pub fn hash(&self)->u32{
        unsafe{crate::binds::mono_string_hash(self.get_ptr())}
    }
    //Cretes [`MString`] form pointer , or returns None if pointer equal to null.
    /// # Safety
    /// *ptr* must be either a valid [`MonoString`] pointer or null. Pasing any other value will lead to undefined behaviour.
    pub unsafe fn from_ptr(ptr:*mut MonoString)->Option<Self>{
        #[cfg(not(feature = "referneced_objects"))]{
            if ptr.is_null(){
                None
            }
            else{
                Some(Self{s_ptr:ptr})
            }
        }
        #[cfg(feature = "referneced_objects")]
        {
            if ptr.is_null(){
                return None;
            }
            Some(Self{handle:GCHandle::create_default(ptr as *mut MonoObject)})
        }
    }
    pub fn get_ptr(&self)->*mut MonoString{
        #[cfg(not(feature = "referneced_objects"))]{
            self.s_ptr
        }
        #[cfg(feature = "referneced_objects")]
        {
            self.handle.get_target() as *mut MonoString
        }
    }
}
impl InteropRecive for MString{
    type SourceType = *mut MonoString;
    fn get_rust_rep(src:Self::SourceType)->Self{
        use crate::exception::except_managed;
        // As long as this function is used in its intended way, there should be no problems, because pointer received from mono runtime must always be either valid or null. (If it is not then that is a bug in mono runtime and there is nothing we can do about it.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
        let opt = unsafe{Self::from_ptr(src)};
        except_managed(opt,"got null in a non-nullable string. For nullabe support use Option<MString>")
    }
}
impl InteropRecive for Option<MString>{
    type SourceType = *mut MonoString;
    fn get_rust_rep(src:Self::SourceType)->Self{
        // As long as this function is used in its intended way, there should be no problems, because pointer received from mono runtime must always be either valid or null. (If it is not then that is a bug in mono runtime and there is nothing we can do about it.
        #[allow(clippy::not_unsafe_ptr_arg_deref)]
        unsafe{MString::from_ptr(src)}
    }
}
impl InteropSend for MString{
    type TargetType = *mut MonoString;
    fn get_mono_rep(src:Self)->Self::TargetType{
        src.get_ptr()
    }
}
impl InteropSend for Option<MString>{
    type TargetType = *mut MonoString;
    fn get_mono_rep(src:Self)->Self::TargetType{
        match src{Some(src)=>src.get_ptr(),None=>null_mut()}
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
        let cstr = unsafe{CString::from_raw(crate::binds::mono_string_to_utf8(self.get_ptr()))};
        let res = cstr.to_str().expect("Colud not create String!").to_owned();
        unsafe{crate::binds::mono_free(cstr.into_raw() as *mut std::os::raw::c_void)};
        res
    }
}
use crate::Exception;
use crate::binds::MonoObject;
impl ObjectTrait for MString{
    fn hash(&self)->i32{
        unsafe{crate::binds::mono_object_hash(self.get_ptr() as *mut MonoObject)}
    }
    fn get_domain(&self)->crate::domain::Domain{
        unsafe{crate::domain::Domain::from_ptr(crate::binds::mono_object_get_domain(self.get_ptr() as *mut MonoObject))}
    }
    fn get_size(&self)->u32{
        unsafe{crate::binds:: mono_object_get_size(self.get_ptr() as *mut MonoObject)}
    }
    fn reflection_get_token(&self)->u32{
        unsafe{crate::binds::mono_reflection_get_token(self.get_ptr() as *mut MonoObject)}
    }
    fn get_class(&self)->crate::class::Class{
        unsafe{crate::class::Class::from_ptr(
            crate::binds::mono_object_get_class(self.get_ptr() as *mut MonoObject)
        ).expect("Could not get class of an object")}
    }
    fn to_mstring(&self)->Result<Option<MString>,Exception>{
        let mut exc:*mut crate::binds::MonoException = core::ptr::null_mut();
        let res = unsafe{MString::from_ptr(
            crate::binds::mono_object_to_string(self.get_ptr() as *mut crate::binds::MonoObject,&mut exc as *mut *mut crate::binds::MonoException as *mut *mut crate::binds::MonoObject)
        )};
        let exc = unsafe{Exception::from_ptr(exc)};
        match exc{
            Some(e)=>Err(e),
            None=>Ok(res),
        }
    }
    fn cast_to_object(&self)->Object{
        unsafe{Object::from_ptr(self.get_ptr() as *mut MonoObject)}.unwrap() //impossible. If string exists, then object exists too.
    }
    fn cast_from_object(obj:&Object)->Option<MString>{
        //TODO: adjust this after including GCHandles to speed things up.
        let cast = match obj.is_inst(&<Self as InteropClass>::get_mono_class()){
            Some(cast)=>cast,
            None=>return None,
        };
        unsafe{Self::from_ptr(cast.get_ptr() as *mut _)}
    }
}
impl Clone for MString{
    fn clone(&self)->Self{
        unsafe{Self::from_ptr(self.get_ptr()).unwrap()}//If object exists then it can't be null
    }
}
impl<O:ObjectTrait> PartialEq<O> for MString{
    fn eq(&self,other:&O)->bool{
        self.get_ptr() as *mut _ == other.cast_to_object().get_ptr()
    }
}
