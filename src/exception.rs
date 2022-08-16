use crate::binds::MonoException;
use core::ptr::null_mut;
use std::ffi::CString;
use crate::Domain;
use crate::Image;
/// Safe representation of MonoException
pub struct Exception{
    exc_ptr:*mut MonoException,
} 
impl Exception{
    /// Raise exception (it can be then cathed by cath clause in managed code)
    /// # Example
    /// ## C#
    ///```csharp
    /// using System.Runtime.CompilerServices;
    /// class SomeClass{
    ///     [MethodImplAttribute(MethodImplOptions.InternalCall)]   
    ///     void ExceptionThrower();
    ///     void SomeMethod(){
    ///         try{
    ///             ExceptionThrower();
    ///         }
    ///         catch(exception){
    ///             Console.WriteLine("This will always catch exceptions raised in ExceptionThrower");
    ///         }
    ///     }
    /// }
    ///```
    /// ## Rust
    ///```rust
    /// #[invokable]
    /// fn exception_thrower(){
    ///     let exception = Exception::not_implemented("This function will just throw exceptions!");
    ///     exception.raise();
    /// }
    ///
    pub fn raise(&self){
        unsafe{crate::binds::mono_raise_exception(self.exc_ptr)};
    }
    pub fn from_name_domain(domain:&Domain,image:&Image,namespace:&str,name:&str)->Option<Self>{
        let ns_cstr = CString::new(namespace).expect("Could not create CString!");
        let nm_cstr = CString::new(name).expect("Could not create CString!");
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_exception_from_name_domain(domain.get_ptr(),image.get_ptr(),ns_cstr.as_ptr(),nm_cstr.as_ptr())
        )};
        drop(ns_cstr);
        drop(nm_cstr);
        return res;
    }
    pub fn from_name(image:&Image,namespace:&str,name:&str)->Option<Self>{
        let ns_cstr = CString::new(namespace).expect("Could not create CString!");
        let nm_cstr = CString::new(name).expect("Could not create CString!");
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_exception_from_name(image.get_ptr(),ns_cstr.as_ptr(),nm_cstr.as_ptr())
        )};
        drop(ns_cstr);
        drop(nm_cstr);
        return res;
    }
    /// Creates [`Exception`] with message *msg*
    pub fn from_name_msg(image:&Image,namespace:&str,name:&str,msg:&str)->Option<Self>{
        let ns_cstr = CString::new(namespace).expect("Could not create CString!");
        let nm_cstr = CString::new(name).expect("Could not create CString!");
        let msg_cstr = CString::new(msg).expect("Could not create CString!");
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_exception_from_name_msg(image.get_ptr(),ns_cstr.as_ptr(),nm_cstr.as_ptr(),msg_cstr.as_ptr())
        )};
        drop(msg_cstr);
        drop(ns_cstr);
        drop(nm_cstr);
        return res;
    }
    /// Returns [`Exception`] that is instance of **System.ArgumentException**
    pub fn argument_exception(arg:&str,msg:&str)->Self{
        let arg_cstr = CString::new(arg).expect("Could not create CString!");
        let msg_cstr = CString::new(msg).expect("Could not create CString!");
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_get_exception_argument(arg_cstr.as_ptr(),msg_cstr.as_ptr())
        )};
        drop(arg_cstr);
        drop(msg_cstr);
        return res.expect("Could not create ArgumentException!");
    }
    /// Returns [`Exception`] that is instance of **System.NotImplementedException**
    pub fn not_implemented(msg:&str)->Self{
        let msg_cstr = CString::new(msg).expect("Could not create CString!");
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_get_exception_not_implemented(msg_cstr.as_ptr())
        )};
        drop(msg_cstr);
        return res.expect("Could not create NotImplementedException!");
    }
    pub unsafe fn from_ptr(exc_ptr:*mut MonoException)->Option<Self>{
        if exc_ptr == null_mut(){
            return None;
        }
        else {return Some(Self{exc_ptr:exc_ptr})};
    }
}
use core::fmt::Formatter;
impl core::fmt::Debug for Exception{
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        //TODO: get exception string and write it.
        return Ok(());
    }
}
