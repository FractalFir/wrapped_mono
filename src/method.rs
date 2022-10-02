use core::{marker::PhantomData,ffi::c_void};
use crate::{InteropSend,Object,Exception,Class};
use crate::tupleutilis::*;
use crate::binds::{MonoMethod,MonoException,MonoObject};
use std::ptr::null_mut;
use std::ffi::CString;
//Depends on: #![feature(specialization)]
//New Mehtod type, WIP
pub struct Method<Args:InteropSend>{
    method:*mut MonoMethod,
    args_type:PhantomData<Args>,
}
pub trait MethodTrait<Args:InteropSend> where Self: Sized{
    fn invoke(&self,object:Option<Object>,args:Args)->Result<Option<Object>,Exception>;
    unsafe fn from_ptr(met_ptr:*mut MonoMethod)->Option<Self>;
    unsafe fn from_ptr_checked(met_ptr:*mut MonoMethod)->Option<Self>;
}
impl<Args:InteropSend> Method<Args> {
    ///Gets the internal pointer to [`MonoMethod`]
    pub fn get_ptr(&self)->*mut MonoMethod{
        self.method
    }
    ///Function returning true if method *self* can call method *called*.
    pub fn can_acces_method<T:InteropSend>(&self,called:&Method<T>)->bool{
        (unsafe{crate::binds::mono_method_can_access_method(
            self.method,called.method,
        ) } != 0)
    }
    pub fn get_token(&self)->u32{
        unsafe{crate::binds::mono_method_get_token(self.method)}
    }
    //TODO:finish this documentaion
    pub fn get_index(&self)->u32{
        unsafe{crate::binds::mono_method_get_index(self.method)}
    }
    ///Get ammount of parameters of this [`Method`]
    pub fn get_param_count(&self)->u32{
        let sig = unsafe{crate::binds::mono_method_signature(self.method)};
        unsafe{crate::binds::mono_signature_get_param_count(sig)}
    }
    ///Gets method in *class* named *name* with *param_count* params.
    pub fn get_method_from_name(class:&crate::class::Class,name:&str,param_count:i32)->Option<Self>{
        let cstr = CString::new(name).expect(crate::STR2CSTR_ERR);
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_class_get_method_from_name(class.get_ptr(),cstr.as_ptr(),param_count)
        )};
        drop(cstr);
        res
    }
    //Gets names of parameters method *self* accepts.
    pub fn get_param_names(&self)->Vec<String>{
        let pcount = self.get_param_count() as usize;
        let mut ptrs:Vec<*const i8> = Vec::with_capacity(pcount);
        ptrs.resize(pcount,std::ptr::null::<i8>());
        unsafe{crate::binds::mono_method_get_param_names(self.method,ptrs.as_ptr() as *mut *const i8)};
        let mut res:Vec<String> = Vec::with_capacity(pcount);
        for ptr in &ptrs{
            let cstr = unsafe{CString::from_raw(*ptr as *mut i8)};
            res.push(cstr.to_str().expect("Could not create String from ptr").to_owned());
            let _ = cstr.into_raw();
        }
        drop(ptrs);
        res
    }
    ///Returns list of types of parameters of method *self*.
    pub fn get_params(&self)->Vec<Class>{
        let sig = unsafe{crate::binds::mono_method_signature(self.method)};
        let mut iter:usize = 0;
        let mut res = Vec::with_capacity(self.get_param_count() as usize);
        while let Some(class) = unsafe{Class::from_ptr(
            {
                let ptr = crate::binds::mono_signature_get_params(sig,&mut iter as *mut usize as *mut *mut c_void);
                if ptr.is_null(){null_mut()}else{crate::binds::mono_class_from_mono_type(ptr)}
            }
        )}{
            res.push(class);
        }
        res
    } 
    ///Returns the return type ofmethod *self*
    pub fn get_return(&self)->Option<Class>{
        let sig = unsafe{crate::binds::mono_method_signature(self.method)};
        let ptr = unsafe{crate::binds:: mono_signature_get_return_type(sig)};
        unsafe{Class::from_ptr(if ptr.is_null(){null_mut()}else{crate::binds::mono_class_from_mono_type(ptr)})}
    } 
}
impl <Args:InteropSend> MethodTrait<Args> for Method<Args>{
    default fn invoke(&self,object:Option<Object>,args:Args)->Result<Option<Object>,Exception>{
        //convert object to invoke on to a pointer.
        let obj_ptr = match object{
            Some(obj)=>obj.get_ptr(),
            None=>core::ptr::null_mut(),
        };
        let mut expect: *mut MonoException = null_mut();
        //convert argument types
        let mut args = <Args as InteropSend>::get_mono_rep(args);
        //convert arguments to pointers
        let mut params = &mut args as *mut _ as *mut c_void;
        //invoke the method itself
        let res_ptr = unsafe{crate::binds::mono_runtime_invoke(
            self.get_ptr(),
            obj_ptr as *mut std::os::raw::c_void,
            &mut params as *mut *mut c_void,
            &mut expect as *mut *mut MonoException as *mut *mut MonoObject,
        )};
        //hold args as long as params lives.
        crate::hold(&args);
        //get result
        let res = unsafe{Object::from_ptr(res_ptr)};
        if expect.is_null(){
            Ok(res)
        }
        else {
            let except = unsafe{Exception::from_ptr(expect).expect("Imposible: pointer is null and not null at the same time.")};
            Err(except)
        }
    }
    default unsafe fn from_ptr(met_ptr:*mut MonoMethod)->Option<Self>{
        if met_ptr.is_null(){
            return None;
        }
        let res = Self{method:met_ptr,args_type:PhantomData};
        let params = res.get_params();
        if params.len() != 1 && params.len() != 0{
            use std::fmt::Write;
            let mut msg = format!("Expected method accepting 1 argument but got a method accepting {} arguments of types:",params.len());
            for param in params{
                write!(msg,",\"{}\"",param.get_name_sig()).expect("Could not print inproper function argument types!");
            }
            panic!("{}",msg);
        }
        //assert!(params[0] == <Args as InteropClass>::get_mono_class());
        Some(res)
    }
    default unsafe fn from_ptr_checked(met_ptr:*mut MonoMethod)->Option<Self>{
        if met_ptr.is_null(){
            return None;
        }
        let res = Self{method:met_ptr,args_type:PhantomData};
        let params = res.get_params();
        Some(res)
    }
}
impl <Args:InteropSend> MethodTrait<Args> for Method<Args> where <Args as InteropSend>::TargetType:TupleToPtrs+CompareClasses
{
    default fn invoke(&self,object:Option<Object>,args:Args)->Result<Option<Object>,Exception>{
        //convert object to invoke on to a pointer.
        let obj_ptr = match object{
            Some(obj)=>obj.get_ptr(),
            None=>core::ptr::null_mut(),
        };
        let mut expect: *mut MonoException = null_mut();
        //convert argument types
        let mut args = <Args as InteropSend>::get_mono_rep(args);
        let mut params = <<Args as InteropSend>::TargetType as TupleToPtrs>::get_ptrs(&mut args as *mut _);
        //invoke the method itself
        let res_ptr = unsafe{crate::binds::mono_runtime_invoke(
            self.get_ptr(),
            obj_ptr as *mut std::os::raw::c_void,
            &mut params as *mut _ as *mut *mut c_void,
            &mut expect as *mut *mut MonoException as *mut *mut MonoObject,
        )};
        //hold args as long as params lives.
        crate::hold(&args);
        //get result
        let res = unsafe{Object::from_ptr(res_ptr)};
        if expect.is_null(){
            Ok(res)
        }
        else {
            let except = unsafe{Exception::from_ptr(expect).expect("Imposible: pointer is null and not null at the same time.")};
            Err(except)
        }
    }
    default unsafe fn from_ptr(met_ptr:*mut MonoMethod)->Option<Self>{
        if met_ptr.is_null(){
            return None;
        }
        let res = Self{method:met_ptr,args_type:PhantomData};
        let params = res.get_params();
        assert!(<<Args as InteropSend>::TargetType as CompareClasses>::compare(params));
        Some(res)
    }
    default unsafe fn from_ptr_checked(met_ptr:*mut MonoMethod)->Option<Self>{
        if met_ptr.is_null(){
            return None;
        }
        let res = Self{method:met_ptr,args_type:PhantomData};
        let params = res.get_params();
        if !(<<Args as InteropSend>::TargetType as CompareClasses>::compare(params)){
            return None;
        }
        Some(res)
    }
}