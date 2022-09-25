use core::{marker::PhantomData,ffi::c_void};
use crate::{InteropRecive,InteropSend,Object,Exception,Class};
use crate::tupleutilis::*;
use crate::binds::{MonoMethod,MonoException,MonoObject};
//Depends on: #![feature(specialization)]
//New Mehtod type, WIP
pub struct Method<Args:InteropSend>{
    method:*mut MonoMethod,
    args_type:PhantomData<Args>,
}
pub trait MethodTrait<Args:InteropSend> {
    fn invoke(&self,object:Option<Object>,args:Args)->Result<Option<Object>,Exception>;
}
impl<Args:InteropSend> Method<Args> {
    pub unsafe fn from_ptr(met_ptr:*mut MonoMethod)->Option<Self>{
        if met_ptr == core::ptr::null_mut(){
            return None;
        }
        return Some(Self{method:met_ptr,args_type:PhantomData});
    }
    ///Gets the internal pointer to [`MonoMethod`]
    pub fn get_ptr(&self)->*mut MonoMethod{
        return self.method;
    }
    ///Function returning true if method *self* can call method *called*.
    pub fn can_acces_method<T:InteropSend>(&self,called:&Method<T>)->bool{
        return unsafe{crate::binds::mono_method_can_access_method(
            self.method,called.method,
        ) } != 0;
    }
    pub fn get_token(&self)->u32{
        return unsafe{crate::binds::mono_method_get_token(self.method)};
    }
    //TODO:finish this documentaion
    pub fn get_index(&self)->u32{
        return unsafe{crate::binds::mono_method_get_index(self.method)};
    }
    ///Get ammount of parameters of this [`Method`]
    pub fn get_param_count(&self)->u32{
        let sig = unsafe{crate::binds::mono_method_signature(self.method)};
        return unsafe{crate::binds::mono_signature_get_param_count(sig)};
    }
    ///Gets method in *class* named *name* with *param_count* params.
    pub fn get_method_from_name(class:&crate::class::Class,name:&str,param_count:i32)->Option<Self>{
        let cstr = std::ffi::CString::new(name).expect(crate::STR2CSTR_ERR);
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_class_get_method_from_name(class.get_ptr(),cstr.as_ptr(),param_count)
        )};
        match &res{
            Some(res)=>(),
            None=>(),
        }
        drop(cstr);
        return res;
    }
    //Gets names of parameters method *self* accepts.
    pub fn get_param_names(&self)->Vec<String>{
        use std::ffi::CString;
        let pcount = self.get_param_count() as usize;
        let mut ptrs:Vec<*const i8> = Vec::with_capacity(pcount);
        ptrs.resize(pcount,0 as *const i8);
        unsafe{crate::binds::mono_method_get_param_names(self.method,ptrs.as_ptr() as *mut *const i8)};
        let mut res:Vec<String> = Vec::with_capacity(pcount);
        for ptr in &ptrs{
            let cstr = unsafe{CString::from_raw(*ptr as *mut i8)};
            res.push(cstr.to_str().expect("Could not create String from ptr").to_owned());
            let _ = cstr.into_raw();
        }
        drop(ptrs);
        return res;
    }
    ///Returns list of types of parameters of method *self*.
    pub fn get_params(&self)->Vec<Class>{
        use std::ptr::null_mut;
        let sig = unsafe{crate::binds::mono_method_signature(self.method)};
        let mut iter:usize = 0;
        let mut res = Vec::with_capacity(self.get_param_count() as usize);
        while let Some(class) = unsafe{Class::from_ptr(
            {
                let ptr = crate::binds::mono_signature_get_params(sig,&mut iter as *mut usize as *mut *mut c_void);
                if ptr == null_mut(){null_mut()}else{crate::binds::mono_class_from_mono_type(ptr)}
            }
        )}{
            res.push(class);
        }
        return res;
    } 
    ///Returns the return type ofmethod *self*
    pub fn get_return(&self)->Option<Class>{
        use std::ptr::null_mut;
        let sig = unsafe{crate::binds::mono_method_signature(self.method)};
        let ptr = unsafe{crate::binds:: mono_signature_get_return_type(sig)};
        let res = unsafe{Class::from_ptr({if ptr == null_mut(){null_mut()}else{crate::binds::mono_class_from_mono_type(ptr)}})};
        return res;
    } 
}
impl <Args:InteropSend> MethodTrait<Args> for Method<Args>{
    default fn invoke(&self,object:Option<Object>,args:Args)->Result<Option<Object>,Exception>{
        use std::ptr::null_mut;
        use crate::binds::MonoException;
        //convert object to invoke on to a pointer.
        let obj_ptr = match object{
            Some(obj)=>obj.get_ptr(),
            None=>core::ptr::null_mut(),
        };
        let mut expect: *mut MonoException = null_mut();
        //convert argument types
        let mut args = <Args as InteropSend>::get_mono_rep(args);
        //convert arguments to pointers
        let mut params = unsafe{&mut args as *mut _ as *mut c_void};
        //invoke the method itself
        let res_ptr = unsafe{crate::binds::mono_runtime_invoke(
            self.get_ptr(),
            obj_ptr as *mut std::os::raw::c_void,
            &mut params as *mut *mut c_void,
            &mut expect as *mut *mut MonoException as *mut *mut MonoObject,
        )};
        //hold args as long as params lives.
        drop(args);
        //get result
        let res = unsafe{Object::from_ptr(res_ptr)};
        if expect == null_mut(){
            return Ok(res);
        }
        else {
            let except = unsafe{Exception::from_ptr(expect).expect("Imposible: pointer is null and not null at the same time.")};
            return Err(except); 
        }
    }
}
impl <Args:InteropSend> MethodTrait<Args> for Method<Args> where <Args as InteropSend>::TargetType:TupleToPtrs{
    default fn invoke(&self,object:Option<Object>,args:Args)->Result<Option<Object>,Exception>{
        use std::ptr::null_mut;
        use crate::binds::MonoException;
        //convert object to invoke on to a pointer.
        let obj_ptr = match object{
            Some(obj)=>obj.get_ptr(),
            None=>core::ptr::null_mut(),
        };
        let mut expect: *mut MonoException = null_mut();
        //convert argument types
        let mut args = <Args as InteropSend>::get_mono_rep(args);
        //convert arguments to pointers
        let mut params = <<Args as InteropSend>::TargetType as TupleToPtrs>::get_ptrs(&mut args as *mut _);
        //invoke the method itself
        let res_ptr = unsafe{crate::binds::mono_runtime_invoke(
            self.get_ptr(),
            obj_ptr as *mut std::os::raw::c_void,
            params.as_ptr()  as *mut *mut c_void,
            &mut expect as *mut *mut MonoException as *mut *mut MonoObject,
        )};
        //hold args as long as params lives.
        drop(args);
        //get result
        let res = unsafe{Object::from_ptr(res_ptr)};
        if expect == null_mut(){
            return Ok(res);
        }
        else {
            let except = unsafe{Exception::from_ptr(expect).expect("Imposible: pointer is null and not null at the same time.")};
            return Err(except); 
        }
    }
}