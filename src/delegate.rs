use crate::binds::{MonoDelegate,MonoMethod};
use crate::gc::{GCHandle,gc_unsafe_exit,gc_unsafe_enter};
use crate::{InteropSend,InteropRecive,InteropClass,Class,Exception,MString,Domain,Object};
use std::marker::PhantomData;
use crate::tupleutilis::{CompareClasses,TupleToPtrs};
use core::ptr::null_mut;
use std::ffi::c_void;
use crate::binds::{MonoObject,MonoException};
use crate::ObjectTrait;
#[allow(unused_imports)] // for docs
use crate::Method;
/// A safe representation of a delegate.
/// Args - a Tuple type made from types of all arguments accepted by this particular delegate
/// # Safety
/// ## Type Mismatch
/// When a delegate is received from mono runtime it's argument types are checked, but those checks are not yet made for a delegate with either 1 or no arguments.
/// This is not a bug, it only means that safety features will not catch some of your errors(wrong types provided by the user of this crate). As long as the signature on the Rust side matches the signature on the C#/F# side, you will never encounter this problem.
/// ## All arguments **must** implement InteropClass!
/// While this is not enforced jet because of limitations of the API(no support for C# tuples), **IT IS STILL NECESSARY**. Ignoring this warning and using Delegates with arguments not implementing InteropClass **will lead to crashes and undefined behaviour**. Before filing bug reports, check that all arguments of your function implement InteropClass.
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
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let sig = unsafe{crate::binds::mono_method_signature(self.get_method_ptr())};
        let pcount = unsafe{crate::binds::mono_signature_get_param_count(sig)};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        pcount
    }
    /// Returns list of classes of parameters of delegate *self*.
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |self|&[`Delegate`]|Rust representation of a delegate to get argument types off|
    pub fn get_params(&self)->Vec<Class>{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
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
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    } 
    // Gets names of all parameters delegate *self* accepts.
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |self|&[`Delegate`]|Rust representation of a delegate to get names of arguments off|
    pub fn get_param_names(&self)->Vec<String>{
        use std::ffi::CString;
        let pcount = self.get_param_count() as usize;
        let mut ptrs:Vec<*const i8> = Vec::with_capacity(pcount);
        ptrs.resize(pcount,std::ptr::null::<i8>());
         #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        unsafe{crate::binds::mono_method_get_param_names(self.get_method_ptr(),ptrs.as_ptr() as *mut *const i8)};
        let mut res:Vec<String> = Vec::with_capacity(pcount);
        for ptr in &ptrs{
            let cstr = unsafe{CString::from_raw(*ptr as *mut i8)};
            res.push(cstr.to_str().expect("Could not create String from ptr").to_owned());
            let _ = cstr.into_raw();
        }
        drop(ptrs);
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns the return type of delegate *self*, if no return type returns *System.Void*
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |self|&[`Delegate`]|Rust representation of a delegate to get return type off|
    pub fn get_return(&self)->Class{
        let sig = unsafe{crate::binds::mono_method_signature(self.get_method_ptr())};
        let ptr = unsafe{crate::binds:: mono_signature_get_return_type(sig)};
        unsafe{Class::from_ptr(crate::binds::mono_class_from_mono_type(ptr)).expect("Got no method return type, but no return type should be signaled by System.Void type!")}
    } 
    fn get_method_ptr(&self)->*mut MonoMethod{
        unsafe{crate::binds::mono_get_delegate_invoke(self.get_class().get_ptr())}
    }
}
/// Trait implemented only for [`Delegate`] type. Splits some functions up from from main [`Method`] type, allowing for different amount of delegate arguments.
pub trait DelegateTrait<Args:InteropSend>{
    /// Creates new Delegate type from a *mut MonoDelegate. Checks if arguments of [`MonoDelegate`] and rust representation of a [`Delegate`] match and if not panic. 
    /// Returns [`None`] if pointer is null.
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |met_ptr|*mut [`MonoDelegate`]|Pointer to delegate to create a representation for.|
    /// # Safety 
    /// Pointer must be either a valid pointer to [`MonoDelegate`] recived from mono runtime, or a null pointer.
    /// **WARNING** argument types not yet checked for delegates with 1 or 0 arguments. This results from limitations of Rust type system and this version of the API, and can't be solved without some realy nasty hacks,
    /// but will be fixed in the future.
    unsafe fn from_ptr(ptr:*mut MonoDelegate)->Option<Self> where Self:Sized;
    /// Creates new Delegate type from a *mut MonoDelegate. Checks if arguments of [`MonoDelegate`] and rust representation of a [`Delegate`] match and if not returns None.
    /// Returns [`None`] if pointer is null.
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |met_ptr|*mut [`MonoDelegate`]|Pointer to delegate to create a representation for.|
    /// # Safety 
    /// Pointer must be either a valid pointer to [`MonoDelegate`] recived from mono runtime, or a null pointer.
    /// **WARNING** argument types not yet checked for delegates with 1 or 0 arguments. This results from limitations of Rust type system and this version of the API, and can't be solved without some realy nasty hacks,
    /// but will be fixed in the future.
    unsafe fn from_ptr_checked(ptr:*mut MonoDelegate)->Option<Self> where Self:Sized;
    /// Invokes this delegate.
    /// # Arguments
    /// | Name   | Type   | Description|
    /// |--------|--------|-------|
    /// | self   | &`Self`|Reference to delegate to invoke. |
    /// | args   | `Args`|Arguments to pass to delegate |
    fn invoke(&self,params:Args)->Result<Option<Object>,Exception>;
}
impl<Args:InteropSend> DelegateTrait<Args> for Delegate<Args>{
    default unsafe fn from_ptr(ptr:*mut MonoDelegate)->Option<Self>{
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
    default unsafe fn from_ptr_checked(ptr:*mut MonoDelegate)->Option<Self>{
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
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
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
            #[cfg(feature = "referneced_objects")]
            gc_unsafe_exit(marker);
            Ok(res)
        }
        else {
            let except = unsafe{Exception::from_ptr(expect).expect("Imposible: pointer is null and not null at the same time.")};
            #[cfg(feature = "referneced_objects")]
            gc_unsafe_exit(marker);
            Err(except)
        }
    }
}
impl<Args:InteropSend> DelegateTrait<Args> for Delegate<Args> where <Args as InteropSend>::TargetType:TupleToPtrs+CompareClasses{
    default unsafe fn from_ptr(ptr:*mut MonoDelegate)->Option<Self>{
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
    default unsafe fn from_ptr_checked(ptr:*mut MonoDelegate)->Option<Self>{
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
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
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
            #[cfg(feature = "referneced_objects")]
            gc_unsafe_exit(marker);
            Ok(res)
        }
        else {
            let except = unsafe{Exception::from_ptr(expect).expect("Imposible: pointer is null and not null at the same time.")};
            #[cfg(feature = "referneced_objects")]
            gc_unsafe_exit(marker);
            Err(except)
        }
    }
}

impl <Args:InteropSend> InteropRecive for Delegate<Args>{
    type SourceType = *mut MonoDelegate;
    // unless this function is abused, this argument should come from the mono runtime, so it should be always valid.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn get_rust_rep(ptr:*mut MonoDelegate)->Delegate<Args>{
       unsafe{Self::from_ptr(ptr).expect("Expected non-null value but got null")}
    }
}
impl <Args:InteropSend> InteropRecive for Option<Delegate<Args>>{
    type SourceType = *mut MonoDelegate;
    // unless this function is abused, this argument should come from the mono runtime, so it should be always valid.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn get_rust_rep(ptr:*mut MonoDelegate)->Option<Delegate<Args>>{
        unsafe{Delegate::from_ptr(ptr)}
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
        else {unsafe{Self::from_ptr_checked(obj.get_ptr() as *mut _)}}
    }
}
impl<O:ObjectTrait,Args:InteropSend> PartialEq<O> for Delegate<Args>{
    fn eq(&self,other:&O)->bool{
        self.get_ptr() as *mut _ == other.cast_to_object().get_ptr()
    }
}

