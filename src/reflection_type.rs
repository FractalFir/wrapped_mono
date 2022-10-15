use crate::gc::{GCHandle,gc_unsafe_enter,gc_unsafe_exit};
use crate::binds::MonoReflectionType;
use crate::{Class,Image,Domain,Array,Method};
use std::ffi::CString;
use crate::{InteropSend,InteropClass,InteropRecive};
/// Rust representation of managed object derived form class `System.Type` 
pub struct ReflectionType{
    #[cfg(not(feature = "referneced_objects"))]
    type_ptr:*mut MonoReflectionType,
    #[cfg(feature = "referneced_objects")]
    handle:GCHandle,
}
impl ReflectionType{
    /// Creates [`ReflectionType`] from a pointer to [`MonoReflectionType`]. The pointer must be either a valid pointer to [`MonoReflectionType`] received from mono runtime, or a null pointer.
    pub fn from_ptr(type_ptr:*mut MonoReflectionType)->Option<Self>{
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
    pub fn get_ptr(&self)->*mut MonoReflectionType{
          #[cfg(not(feature = "referneced_objects"))]
          {
                self.type_ptr 
          }
          #[cfg(feature = "referneced_objects")]
          {
                self.handle.get_target() as *mut _
          }
    }
    pub fn from_class(class:&Class)->Self{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let ptr = unsafe{crate::binds::mono_class_get_type(class.get_ptr())};
        let res = unsafe{Self::from_type_ptr(ptr).unwrap()}; // Converting class to ReflectionType should never fail
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    fn get_type_ptr(&self)->*mut crate::binds::MonoType{
        unsafe{crate::binds::mono_reflection_type_get_type(self.get_ptr())}
    }
    unsafe fn from_type_ptr(type_ptr:*mut crate::binds::MonoType)->Option<Self>{
        if type_ptr.is_null(){
            return None;
        }
        let dom = Domain::get_current().expect("Can't convert *MonoType to ReflecionType before JIT started.");
        Some(unsafe{Self::from_ptr(crate::binds::mono_type_get_object(dom.get_ptr(),type_ptr))}
        .expect("Could not convert MonoType pointer to a ReflectionType!"))
    }
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
    pub fn create_generic(gtype_img:&Image,gtype:&str,gargs:&[Self])->Option<Self>{
        let garg_count = gargs.len();
        let gtype_str = format!("{}`{}",gtype,garg_count);
        println!("{}",gtype_str);
        let res = match ReflectionType::from_name(&gtype_str,gtype_img){
            Some(res)=>res,
            None=>return None,
        };
        //let arr = Array::new(
        //MAKE_GENERIC_TYPE_MET.invoke(res.cast_to_object(),
        unimplemented!();
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
    fn get_rust_rep(rarg: Self::SourceType)->Self{
        Self::from_ptr(rarg).expect("Recived null on a not nullable type")
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
use lazy_static::lazy_static;
lazy_static!{
    static ref TYPE_CLASS:Class = {
        let img = crate::Assembly::assembly_loaded("mscorlib").expect("Assembly mscorlib not loaded, could not get System.Type class!").get_image();
        Class::from_name_case(&img,"System","Type").expect("Could not get System.Type class form mscorlib!")
    };
    static ref MAKE_GENERIC_TYPE_MET: Method<Array<1,ReflectionType>> = {
        let img = crate::Assembly::assembly_loaded("mscorlib").expect("Assembly mscorlib not loaded, could not get System.Type class!").get_image();
        let class = Class::from_name_case(&img,"System","Type").expect("Could not get System.Type class form mscorlib!");
        Method::get_method_from_name(&class,"MakeGenericType",1).expect("Could not get System.Type::MakeGenericType method!")
    };
}
