use crate::binds::MonoDelegate;
use crate::gc::GCHandle;
use crate::InteropSend;
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