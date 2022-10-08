use crate::binds::{MonoDelegate,MonoMethod};
use crate::gc::GCHandle;
use crate::{InteropSend,InteropRecive};
use std::marker::PhantomData;
use crate::tupleutilis::{CompareClasses,TupleToPtrs};
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
    /// |self|&[`Method`]|Rust representation of a method to get argument count of|
    pub fn get_param_count(&self)->u32{
        let sig = unsafe{crate::binds::mono_method_signature(self.get_ptr() as *mut MonoMethod)};
        unsafe{crate::binds::mono_signature_get_param_count(sig)}
    }
       
}
pub trait DelegateTrait<Args:InteropSend>{
    fn from_ptr(ptr:*mut MonoDelegate)->Option<Self> where Self:Sized;
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
}
impl<Args:InteropSend> DelegateTrait<Args> for Delegate<Args> where <Args as InteropSend>::TargetType:TupleToPtrs+CompareClasses{
    default fn from_ptr(ptr:*mut MonoDelegate)->Option<Self>{
        let res = {if ptr.is_null(){
            return None
        }
        else{
            #[cfg(not(feature = "referneced_objects"))]
            {Some(Self{dptr:ptr,args_type:PhantomData})}
            #[cfg(feature = "referneced_objects")]
            {Some(Self{handle:GCHandle::create_default(ptr as *mut crate::binds::MonoObject),args_type:PhantomData})}                
        }};
        /// Do type checks
        res
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