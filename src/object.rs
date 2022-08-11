pub struct Object{
    obj_ptr:*mut crate::binds::MonoObject,
} 
pub trait ObjectTrait{
    fn hash(&self)->i32;
    fn get_domain(&self)->i32;
    unsafe fn unbox(&self)->*mut std::ffi::c_void;
    fn get_size(&self)->u32;
    fn reflection_get_token(&self)->u32;
}
impl Object{
    fn hash(&self)->i32{
        return unsafe{crate::binds::mono_object_hash(self.obj_ptr)};
    }
    fn get_domain(&self)->crate::domain::Domain{
        return unsafe{crate::domain::Domain::from_ptr(crate::binds::mono_object_get_domain(self.obj_ptr))};
    }
    unsafe fn unbox(&self)->*mut std::ffi::c_void{
        return unsafe{crate::binds::mono_object_unbox(self.obj_ptr)};
    }
    fn get_size(&self)->u32{
        return unsafe{crate::binds:: mono_object_get_size(self.obj_ptr)};
    }
    fn reflection_get_token(&self)->u32{
        return unsafe{crate::binds::mono_reflection_get_token(self.obj_ptr)};
    }
}
impl Object{
    unsafe fn from_ptr(obj_ptr:*mut crate::binds::MonoObject)->Option<Self>{
        if obj_ptr == core::ptr::null_mut(){
            return None;
        }
        return Some(Self{obj_ptr:obj_ptr});
    }
}
impl crate::invokable::InvokePass for Object{
    type SourceType = *mut  crate::binds::MonoObject;
    fn get_rust_rep(arg:Self::SourceType)->Self{
        return unsafe{Self::from_ptr(arg)}.expect("passed MonoObject argument is invalid");
    }
}
impl Clone for Object{
    fn clone(&self)->Self{
        //if clone fails, it means that there is a much bigger problem with mono runtime, so we jus
        return unsafe{Self::from_ptr(crate::binds::mono_object_clone(self.obj_ptr))}.expect("MonoRuntime could not clone object!");
    }
}