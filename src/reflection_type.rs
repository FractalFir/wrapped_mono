use crate::gc::{GCHandle,gc_unsafe_enter,gc_unsafe_exit};
use crate::binds::{MonoReflectionType,MonoType};
use crate::{Class,Image,Domain,Array,Method,MethodTrait};
use std::ffi::CString;
use crate::{Object,ObjectTrait};
use crate::{Exception,MString};
use crate::{InteropSend,InteropClass,InteropRecive};
/// Rust representation of managed object derived form class `System.Type` 
pub struct ReflectionType{
    #[cfg(not(feature = "referneced_objects"))]
    type_ptr:*mut MonoReflectionType,
    #[cfg(feature = "referneced_objects")]
    handle:GCHandle,
}
use crate::PointerConversion;
impl PointerConversion for ReflectionType{
    type PtrType = MonoReflectionType;
    fn get_ptr(&self)->*mut Self::PtrType{
        #[cfg(not(feature = "referneced_objects"))]
          {
                self.type_ptr 
          }
          #[cfg(feature = "referneced_objects")]
          {
                self.handle.get_target() as *mut _
          }
    }
    unsafe fn from_ptr(type_ptr:*mut Self::PtrType)->Option<Self>{
        #[cfg(not(feature = "referneced_objects"))]
        {
            if type_ptr.is_null(){
                return None;
            }
            Some(Self{type_ptr})
        }
        #[cfg(feature = "referneced_objects")]
        {
            if type_ptr.is_null(){
                return None;
            }
            Some(Self{handle:GCHandle::create_default(type_ptr as *mut _)})
        }
    }
}
impl ReflectionType{
    /// Converts a class to a [`MonoReflectionType`]
    pub fn from_class(class:&Class)->Self{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let ptr = unsafe{crate::binds::mono_class_get_type(class.get_ptr())};
        let res = unsafe{Self::from_type_ptr(ptr).unwrap()}; // Converting class to ReflectionType should never fail
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns a pointer to unmanaged representation of `System.Type`
    pub fn get_type_ptr(&self)->*mut MonoType{
        unsafe{crate::binds::mono_reflection_type_get_type(self.get_ptr())}
    }
    /// Creates an new instance from a pointer to unmanaged representation of `System.Type`
    /// # Safety
    /// The pointer must be either a pointer to valid *mut [`MonoType`] or null.
    pub unsafe fn from_type_ptr(type_ptr:*mut MonoType)->Option<Self>{
        if type_ptr.is_null(){
            return None;
        }
        let dom = Domain::get_current().expect("Can't convert *MonoType to ReflecionType before JIT started.");
        Some(unsafe{Self::from_ptr(crate::binds::mono_type_get_object(dom.get_ptr(),type_ptr))}
        .expect("Could not convert MonoType pointer to a ReflectionType!"))
    }
    /// Gets type with *name* inside image *img*
    pub fn from_name(name:&str,img:&Image)->Option<Self>{
        let cstr = CString::new(name).expect("Could not convert string to CString");
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let ptr = unsafe{crate::binds::mono_reflection_type_from_name(cstr.as_ptr() as *mut i8,img.get_ptr())};
        if ptr.is_null(){
            return None
        }
        let res = unsafe{Self::from_type_ptr(ptr)};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    #[doc(hidden)] // Unfinished
    pub fn create_generic(gtype_img:&Image,gtype:&str,gargs:&[Self])->Option<Self>{
        let garg_count = gargs.len();
        let gtype_str = format!("{}`{}",gtype,garg_count);
        println!("{}",gtype_str);
        let res = match ReflectionType::from_name(&gtype_str,gtype_img){
            Some(res)=>res,
            None=>return None,
        };
        let arr:Array<1,ReflectionType> = gargs.into();
        let res = MAKE_GENERIC_TYPE_MET.invoke(Some(res.cast_to_object()),arr);
        // handle exceptions
        let res = match res{
            Ok(res)=>res,
            Err(err)=>panic!("EXCEPTION:'{}'",err),//return None,
        };
        // handle null 
        let res = match res{
            Some(res)=>res,
            None=>return None,
        };
        Self::cast_from_object(&res)
    }
}
impl InteropSend for ReflectionType{
    type TargetType = *mut MonoReflectionType;
    fn get_mono_rep(rarg:Self)->Self::TargetType{
        rarg.get_ptr()
    }
}
impl InteropRecive for ReflectionType{
    type SourceType = *mut MonoReflectionType;
    // unless this function is abused, this argument should come from the mono runtime, so it should be always valid.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn get_rust_rep(rarg: Self::SourceType)->Self{
        unsafe{Self::from_ptr(rarg).expect("Recived null on a not nullable type")}
    }
}
impl InteropClass for ReflectionType{
    fn get_mono_class()->Class{
        *TYPE_CLASS
    }
}
impl From<Class> for ReflectionType{
    fn from(class:Class)->Self{
        Self::from_class(&class)
    }
}
impl ObjectTrait for ReflectionType{
    fn hash(&self)->i32{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let hsh = unsafe{crate::binds::mono_object_hash(self.get_ptr() as *mut _)};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        hsh
    }
    fn get_domain(&self)->Domain{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let dom = unsafe{Domain::from_ptr(crate::binds::mono_object_get_domain(self.get_ptr() as *mut _))};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        dom
    }
    fn get_size(&self)->u32{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let size = unsafe{crate::binds:: mono_object_get_size(self.get_ptr() as *mut _)};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        size
    }
    fn reflection_get_token(&self)->u32{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let tok = unsafe{crate::binds::mono_reflection_get_token(self.get_ptr() as *mut _)};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        tok
    }
    fn get_class(&self)->Class{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let class = unsafe{Class::from_ptr(
            crate::binds::mono_object_get_class(self.get_ptr() as *mut _)
        ).expect("Could not get class of an object")};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        class
    }
    fn to_mstring(&self)->Result<Option<MString>,Exception>{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let mut exc:*mut crate::binds::MonoException = core::ptr::null_mut();
        let res = unsafe{MString::from_ptr(
            crate::binds::mono_object_to_string(self.get_ptr() as *mut _,&mut exc as *mut *mut crate::binds::MonoException as *mut *mut crate::binds::MonoObject)
        )};
        let exc = unsafe{Exception::from_ptr(exc)};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        match exc{
            Some(e)=>Err(e),
            None=>Ok(res),
        }
    }
    fn cast_to_object(&self)->Object{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let obj = unsafe{Object::from_ptr(self.get_ptr() as *mut _)}.unwrap();//Faliure impossible, object is always an object.
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        obj
    }
    fn cast_from_object(obj:&Object)->Option<Self>{
        //TODO: adjust this after including GCHandles to speed things up.
        let cast = match obj.is_inst(&<Self as InteropClass>::get_mono_class()){
            Some(cast)=>cast,
            None=>return None,
        };
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let cast = unsafe{Self::from_ptr(cast.get_ptr() as *mut _)};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        cast
    }
}
use lazy_static::lazy_static;
lazy_static!{
    static ref TYPE_CLASS:Class = {
        let img = crate::Assembly::assembly_loaded("mscorlib").expect("Assembly mscorlib not loaded, could not get System.Type class!").get_image();
        Class::from_name_case(&img,"System","Type").expect("Could not get System.Type class form mscorlib!")
    };
    static ref MAKE_GENERIC_TYPE_MET: Method<Array<1,ReflectionType>> = {
        let img = crate::Assembly::assembly_loaded("mscorlib").expect("Assembly mscorlib not loaded, could not get System.Type class!").get_image();
        let class = Class::from_name_case(&img,"System","Type").expect("Could not get System.Type class form mscorlib!");
        Method::get_from_name(&class,"MakeGenericType",1).expect("Could not get System.Type::MakeGenericType method!")
    };
}
impl Clone for ReflectionType{
    fn clone(&self)->Self{
        unsafe{Self::from_ptr(self.get_ptr())}.unwrap()
    }
}
