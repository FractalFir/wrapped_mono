use crate::binds::{MonoDelegate,MonoMethod};
use crate::gc::GCHandle;
use crate::{InteropSend,InteropRecive,InteropClass,Class,Exception,MString,Domain,Object};
use std::marker::PhantomData;
use crate::tupleutilis::{CompareClasses,TupleToPtrs};
use core::ptr::null_mut;
use std::ffi::c_void;
use crate::binds::{MonoObject,MonoException};
use crate::ObjectTrait;
pub struct Delegate<Args:InteropSend>{
    #[cfg(not(feature = "referneced_objects"))]
    dptr:*mut MonoDelegate,
    #[cfg(feature = "referneced_objects")]
    handle:GCHandle,
    args_type:PhantomData<Args>,
} 
impl<Args:InteropSend> Delegate<Args>{
    fn get_ptr(&self)->*mut MonoDelegate{
        #[cfg(not(feature = "referneced_objects"))]
        {self.dptr}
        #[cfg(feature = "referneced_objects")]
        {self.handle.get_target() as *mut MonoDelegate}
    }
    /// Counts number of parameters(arguments) this function accepts.
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |self|&[Delegate]|Rust representation of a delegate to get argument count of|
    pub fn get_param_count(&self)->u32{
        let sig = unsafe{crate::binds::mono_method_signature(self.get_method_ptr())};
        unsafe{crate::binds::mono_signature_get_param_count(sig)}
    }
    /// Returns list of classes of parameters of delegate *self*.
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |self|&[`Delegate`]|Rust representation of a delegate to get argument types off|
    pub fn get_params(&self)->Vec<Class>{
        let sig = unsafe{crate::binds::mono_method_signature(self.get_method_ptr())};
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
    fn get_method_ptr(&self)->*mut MonoMethod{
        unsafe{crate::binds::mono_get_delegate_invoke(self.get_class().get_ptr())}
    }
}
pub trait DelegateTrait<Args:InteropSend>{
    fn from_ptr(ptr:*mut MonoDelegate)->Option<Self> where Self:Sized;
    fn from_ptr_checked(ptr:*mut MonoDelegate)->Option<Self> where Self:Sized;
    fn invoke(&self,params:Args)->Result<Option<Object>,Exception>;
}
impl<Args:InteropSend> DelegateTrait<Args> for Delegate<Args>{
    default fn from_ptr(ptr:*mut MonoDelegate)->Option<Self>{
        if ptr.is_null(){
            None
        }
        else{
            #[cfg(not(feature = "referneced_objects"))]
            {Some(Self{dptr:ptr,args_type:PhantomData})}
            #[cfg(feature = "referneced_objects")]
            {Some(Self{handle:GCHandle::create_default(ptr as *mut crate::binds::MonoObject),args_type:PhantomData})}                
        }

    }
    default fn from_ptr_checked(ptr:*mut MonoDelegate)->Option<Self>{
        if ptr.is_null(){
            None
        }
        else{
            #[cfg(not(feature = "referneced_objects"))]
            {Some(Self{dptr:ptr,args_type:PhantomData})}
            #[cfg(feature = "referneced_objects")]
            {Some(Self{handle:GCHandle::create_default(ptr as *mut crate::binds::MonoObject),args_type:PhantomData})}                
        }
    }
    default fn invoke(&self,params:Args)->Result<Option<Object>,Exception>{
        let mut expect: *mut MonoException = null_mut();
        //convert argument types
        let mut args = <Args as InteropSend>::get_mono_rep(params);
        //convert arguments to pointers
        let mut params = &mut args as *mut _ as *mut c_void;
        //invoke the delegate itself
        let res_ptr = unsafe{crate::binds::mono_runtime_delegate_invoke(
            self.get_ptr() as *mut _,
            &mut params as *mut *mut c_void,
            &mut expect as *mut *mut MonoException as *mut *mut MonoObject,
        )};
        //hold args as long as params lives.
        crate::hold(&args);
        //get result
        let res = unsafe{Object::from_ptr(res_ptr)};
        println!("expect:{}",expect as usize);
        if expect.is_null(){
            Ok(res)
        }
        else {
            let except = unsafe{Exception::from_ptr(expect).expect("Imposible: pointer is null and not null at the same time.")};
            Err(except)
        }
    }
}
impl<Args:InteropSend> DelegateTrait<Args> for Delegate<Args> where <Args as InteropSend>::TargetType:TupleToPtrs+CompareClasses{
    default fn from_ptr(ptr:*mut MonoDelegate)->Option<Self>{
        let res = {if ptr.is_null(){
            return None
        }
        else{
            #[cfg(not(feature = "referneced_objects"))]
            {Self{dptr:ptr,args_type:PhantomData}}
            #[cfg(feature = "referneced_objects")]
            {Self{handle:GCHandle::create_default(ptr as *mut crate::binds::MonoObject),args_type:PhantomData}}                
        }};
        // Do type checks
        let params = res.get_params();
        if !<<Args as InteropSend>::TargetType as CompareClasses>::compare(&params){
            use std::fmt::Write;
            let mut msg = format!("Delegate Type Mismatch! Got a deleagte accepting {} arguments of types:",params.len());
            for param in params{
                write!(msg,",\"{}\"",param.get_name_sig()).expect("Could not print inproper function argument types!");
            }
            panic!("{}",msg);
        }
        Some(res)
    }
    default fn from_ptr_checked(ptr:*mut MonoDelegate)->Option<Self>{
        let res = {if ptr.is_null(){
            return None
        }
        else{
            #[cfg(not(feature = "referneced_objects"))]
            {Self{dptr:ptr,args_type:PhantomData}}
            #[cfg(feature = "referneced_objects")]
            {Self{handle:GCHandle::create_default(ptr as *mut crate::binds::MonoObject),args_type:PhantomData}}                
        }};
        // Do type checks
        let params = res.get_params();
        if !<<Args as InteropSend>::TargetType as CompareClasses>::compare(&params){
            None
        }
        else {
            Some(res)
        }
    }
    default fn invoke(&self,params:Args)->Result<Option<Object>,Exception>{
        let mut expect: *mut MonoException = null_mut();
        //convert argument types
        let mut args = <Args as InteropSend>::get_mono_rep(params);
        let mut params = <<Args as InteropSend>::TargetType as TupleToPtrs>::get_ptrs(&mut args as *mut _);
        //invoke the delegate itself
        let res_ptr = unsafe{crate::binds::mono_runtime_delegate_invoke(
            self.get_ptr() as *mut _,
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
}

impl <Args:InteropSend> InteropRecive for Delegate<Args>{
    type SourceType = *mut MonoDelegate;
    fn get_rust_rep(ptr:*mut MonoDelegate)->Delegate<Args>{
       Self::from_ptr(ptr).expect("Expected non-null value but got null")
    }
}
impl <Args:InteropSend> InteropRecive for Option<Delegate<Args>>{
    type SourceType = *mut MonoDelegate;
    fn get_rust_rep(ptr:*mut MonoDelegate)->Option<Delegate<Args>>{
        Delegate::from_ptr(ptr)
    }
}
impl <Args:InteropSend> InteropClass for Delegate<Args>{
    fn get_mono_class()->Class{
        Class::get_delegate_class()
    }
}
impl <Args:InteropSend> InteropSend for Delegate<Args>{
    type TargetType = *mut MonoDelegate;
    fn get_mono_rep(input:Delegate<Args>)->Self::TargetType{
        input.get_ptr()
    }
}
impl<Args:InteropSend> ObjectTrait for Delegate<Args>{
    fn hash(&self)->i32{
        unsafe{crate::binds::mono_object_hash(self.get_ptr() as *mut _)}
    }
    fn get_domain(&self)->Domain{
        unsafe{Domain::from_ptr(crate::binds::mono_object_get_domain(self.get_ptr() as *mut _))}
    }
    fn get_size(&self)->u32{
        unsafe{crate::binds:: mono_object_get_size(self.get_ptr() as *mut _)}
    }
    fn reflection_get_token(&self)->u32{
        unsafe{crate::binds::mono_reflection_get_token(self.get_ptr() as *mut _)}
    }
    fn get_class(&self)->Class{
        unsafe{Class::from_ptr(
            crate::binds::mono_object_get_class(self.get_ptr() as *mut _)
        ).expect("Could not get class of an object")}
    }
    fn to_mstring(&self)->Result<Option<MString>,Exception>{
        let mut exc:*mut crate::binds::MonoException = core::ptr::null_mut();
        let res = unsafe{MString::from_ptr(
            crate::binds::mono_object_to_string(self.get_ptr() as *mut _,&mut exc as *mut *mut crate::binds::MonoException as *mut *mut crate::binds::MonoObject)
        )};
        let exc = unsafe{Exception::from_ptr(exc)};
        match exc{
            Some(e)=>Err(e),
            None=>Ok(res),
        }
    }
    fn cast_to_object(&self)->Object{unsafe{Object::from_ptr(self.get_ptr() as *mut _)}.unwrap()/*Faliure impossible, object is always an object.*/}
    fn cast_from_object(obj:&Object)->Option<Self>{
        if !Self::get_mono_class().is_assignable_from(&obj.get_class()){
            None
        }
        else {Self::from_ptr_checked(obj.get_ptr() as *mut _)}
    }
}
