///
pub struct Method{
    met_ptr:*mut crate::binds::MonoMethod,
} 
// 1213
use crate::object::Object;
use crate::binds::{MonoObject};
impl Method{
    pub unsafe fn from_ptr(met_ptr:*mut crate::binds::MonoMethod)->Option<Self>{
        if met_ptr == core::ptr::null_mut(){
            return None;
        }
        return Some(Self{met_ptr:met_ptr});
    }
    pub fn get_ptr(&self)->*mut crate::binds::MonoMethod{
        return self.met_ptr;
    }
    pub fn get_method_from_name(class:crate::class::Class,name:&str,param_count:i32)->Option<Self>{
        let cstr = std::ffi::CString::new(name).expect("Could not crate CString");
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_class_get_method_from_name(class.get_ptr(),cstr.as_ptr(),param_count)
        )};
        drop(cstr);
        return res;
    }
    ///Function returning true if method *self* can call method *called*.
    pub fn can_acces_method(&self,called:&Method)->bool{
        return unsafe{crate::binds::mono_method_can_access_method(
            self.get_ptr(),called.get_ptr(),
        ) } != 0;
    }
    pub fn get_token(&self)->u32{
        return unsafe{crate::binds::mono_method_get_token(self.get_ptr())};
    }
    pub fn get_index(&self)->u32{
        return unsafe{crate::binds::mono_method_get_index(self.get_ptr())};
    }
    pub fn get_param_count(&self)->u32{
        let sig = unsafe{crate::binds::mono_method_signature(self.get_ptr())};
        return unsafe{crate::binds::mono_signature_get_param_count(sig)};
    }
    pub fn get_name(&self)->String{
        let cstr = unsafe{std::ffi::CString::from_raw(crate::binds::mono_method_get_name(self.get_ptr()) as *mut i8)};
        let s = cstr.to_str().expect("Could not converted ptr to String!").to_owned();
        cstr.into_raw();
        return s;
    }
    pub fn get_class(&self)->crate::class::Class{
        return unsafe{crate::class::Class::from_ptr(
            crate::binds::mono_method_get_class(self.get_ptr())
        ).expect("Could not get class of a method")};
    }
    //mono_signature_get_return_type(sig: *mut MonoMethodSignature) -> *mut MonoType;
    pub fn get_param_names(&self)->Vec<String>{
        use std::ffi::CString;
        let pcount = self.get_param_count() as usize;
        let mut ptrs:Vec<*const i8> = Vec::with_capacity(pcount);
        ptrs.resize(pcount,0 as *const i8);
        unsafe{crate::binds::mono_method_get_param_names(self.get_ptr(),ptrs.as_ptr() as *mut *const i8)};
        let mut res:Vec<String> = Vec::with_capacity(pcount);
        for ptr in &ptrs{
            let cstr = unsafe{CString::from_raw(*ptr as *mut i8)};
            res.push(cstr.to_str().expect("Could not create String from ptr").to_owned());
            cstr.into_raw();
        }
        drop(ptrs);
        return res;
    }
    //TODO: return exception instead of () && write macro for auto params conversion.
    ///Simple, fast(does not convert types) version of method_invoke! macro. **Doesn't** handle virtual methods, calls 
    pub unsafe fn invoke_unsafe(&self,obj:Option<&Object>,params:&mut Vec<*mut std::os::raw::c_void>)->Result<Object,()>{
        use core::ffi::c_void;
        use crate::binds::MonoException;
        use std::ptr::null_mut;
        let obj_ptr = match obj{
            Some(obj)=>obj.get_ptr(),
            None=>core::ptr::null_mut(),
        };
        let mut expect: *mut MonoException = null_mut();
        let res_ptr = crate::binds::mono_runtime_invoke(
            self.get_ptr(),
            obj_ptr as *mut std::os::raw::c_void,
            params.as_ptr() as *mut *mut c_void,
            &mut expect as *mut *mut MonoException as *mut *mut MonoObject,
        );
        let res = unsafe{Object::from_ptr(res_ptr)};
        match res{
            Some(obj)=>return Ok(obj),
            None=>return Err(()),//TODO: return new Exceprtion type instead of ().
        }
    }
}
