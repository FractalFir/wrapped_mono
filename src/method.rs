use crate::binds::{MonoException, MonoMethod, MonoObject};
use crate::tupleutilis::*;
use crate::{Class, Exception, InteropSend, Object};
use core::{ffi::c_void, marker::PhantomData};
use std::ffi::CString;
use std::ptr::null_mut;
//Depends on: #![feature(specialization)]
/// Rust representation of a managed method(function of code loaded into mono runtime).
/// Args - Tuple type of types of all arguments accepted by this particular method.
/// # Safety
/// ## Type Mismatch
/// When a method is received from mono runtime it's argument types are checked, but those checks are not yet made for a method with either 1 or no arguments.
/// This is not a bug, it only means that safety features will not catch some of your errors(wrong types provided by the user of this crate). As long as the signature on the Rust side matches the signature on the C#/F# side, you will never encounter this problem.
/// ## All arguments **must** implement InteropClass!
/// While this is not enforced jet because of limitations of the API(no support for C# tuples), **IT IS STILL NECESSARY**. Ignoring this warning and using Methods with
/// arguments not implementing InteropClass **will lead to crashes and undefined behaviour**. Before filing bug reports, check that all arguments of your function implement InteropClass.
pub struct Method<Args: InteropSend + CompareClasses>
where
    <Args as InteropSend>::TargetType: TupleToPtrs,
{
    method: *mut MonoMethod,
    args_type: PhantomData<Args>,
}
/// Trait implemented only for [`Method`] type. Spiliting it from main [`Method`] type allows for different amount of method arguments.
/*
pub trait MethodTrait<Args: InteropSend + CompareClasses>
where
    Self: Sized,
{
    /// Invoke this method on object *object* with arguments *args*
    /// # Arguments
    /// | Name   | Type   | Description|
    /// |--------|--------|-------|
    /// | self   | &`Self`|Reference to method to invoke. |
    /// | object | [`Option<Object>`] |Object to invoke method on. Pass [`None`] if method is static. |
    /// | args   | `Args`|Arguments to pass to method |
    fn invoke(&self, object: Option<Object>, args: Args) -> Result<Option<Object>, Exception>;
    /// Creates new Method type from a *mut MonoMethod, checks if arguments of [`MonoMethod`] and rust representation of a [`Method`] match and if not panic.
    /// Returns [`None`] if pointer is null.
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |met_ptr|*mut [`MonoMethod`]|Pointer to method to create a representation for.|
    /// # Safety
    /// Pointer must be either a valid pointer to [`MonoMethod`] recived from mono runtime, or a null pointer.
    /// **WARNING** argument types not yet checked for methods with 1 or 0 arguments. This results from limitations of Rust type system, and can't be solved without some realy nasty hacks,
    /// but will be fixed in the future
    unsafe fn from_ptr(met_ptr: *mut MonoMethod) -> Option<Self>;
    /// Creates new Method type from a *mut MonoMethod, checks if arguments of [`MonoMethod`] and rust representation of a [`Method`] match and returns [`None`] if so.
    /// Returns [`None`] if pointer is null.
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |met_ptr|*mut [`MonoMethod`]|Pointer to method to create a representation for.|
    /// # Safety
    /// Pointer must be either a valid pointer to [`MonoMethod`] recived from mono runtime, or a null pointer.
    /// **WARNING** argument types not yet checked for methods with 1 or 0 arguments. This results from limitations of Rust type system, and can't be solved without some realy nasty hacks,
    /// but will be fixed in the future
    unsafe fn from_ptr_checked(met_ptr: *mut MonoMethod) -> Option<Self>;
}
*/
impl<Args: InteropSend + CompareClasses> Method<Args>
where
    <Args as InteropSend>::TargetType: TupleToPtrs,
{
    /// Gets the internal pointer to [`MonoMethod`].
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |self|&[`Method`]|Rust representation of a method to get pointer to.|
    pub fn get_ptr(&self) -> *mut MonoMethod {
        self.method
    }
    /// Checks if method *self* can call method *called*.
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |self   |&[`Method`]|   Rust representation of the method preforiming the call.|
    /// |called |&[`Method`]|   Rust representation of the method beeing called.|
    pub fn can_acces_method<T: InteropSend + CompareClasses>(&self, called: &Method<T>) -> bool
    where
        <T as InteropSend>::TargetType: TupleToPtrs,
    {
        (unsafe { crate::binds::mono_method_can_access_method(self.method, called.method) } != 0)
    }
    ///Metadata token. Not working without MetadataAPI
    #[doc(hidden)]
    pub fn get_token(&self) -> u32 {
        unsafe { crate::binds::mono_method_get_token(self.method) }
    }
    //??? mono docs do not say what does it do, nut the educated guess is that it returns which method of a class it is.
    #[doc(hidden)]
    pub fn get_index(&self) -> u32 {
        unsafe { crate::binds::mono_method_get_index(self.method) }
    }
    /// Counts number of parameters(arguments) this function accepts.
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |self|&[`Method`]|Rust representation of a method to get argument count of|
    pub fn get_param_count(&self) -> u32 {
        let sig = unsafe { crate::binds::mono_method_signature(self.method) };
        unsafe { crate::binds::mono_signature_get_param_count(sig) }
    }
    /// Gets method in *class* named *name* with *param_count* parameters. Returns [`None`] if could not find method or if its arguments did not match.
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |class|&[`Class`]|Class the sought method belongs to|
    /// |name|&[`str`]|Name of the method|
    /// |param_count|&[`i32`]|Ammount of parameters(arguments) method accepts|
    pub fn get_from_name(
        class: &crate::class::Class,
        name: &str,
        param_count: i32,
    ) -> Option<Self> {
        let cstr = CString::new(name).expect(crate::STR2CSTR_ERR);
        let res = unsafe {
            Self::from_ptr_checked(crate::binds::mono_class_get_method_from_name(
                class.get_ptr(),
                cstr.as_ptr(),
                param_count,
            ))
        };
        drop(cstr);
        res
    }
    /// Gets names of all parameters method *self* accepts.
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |self|&[`Method`]|Rust representation of a method to get names of arguments off|
    pub fn get_param_names(&self) -> Vec<String> {
        let pcount = self.get_param_count() as usize;
        let mut ptrs: Vec<*const i8> = Vec::with_capacity(pcount);
        ptrs.resize(pcount, std::ptr::null::<i8>());
        unsafe {
            crate::binds::mono_method_get_param_names(self.method, ptrs.as_ptr() as *mut *const i8)
        };
        let mut res: Vec<String> = Vec::with_capacity(pcount);
        for ptr in &ptrs {
            let cstr = unsafe { CString::from_raw(*ptr as *mut i8) };
            res.push(
                cstr.to_str()
                    .expect("Could not create String from ptr")
                    .to_owned(),
            );
            let _ = cstr.into_raw();
        }
        drop(ptrs);
        res
    }
    /// Returns list of classes of parameters of method *self*.
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |self|&[`Method`]|Rust representation of a method to get argument types off|
    pub fn get_params(&self) -> Vec<Class> {
        let sig = unsafe { crate::binds::mono_method_signature(self.method) };
        let mut iter: usize = 0;
        let mut res = Vec::with_capacity(self.get_param_count() as usize);
        while let Some(class) = unsafe {
            Class::from_ptr({
                let ptr = crate::binds::mono_signature_get_params(
                    sig,
                    &mut iter as *mut usize as *mut *mut c_void,
                );
                if ptr.is_null() {
                    null_mut()
                } else {
                    crate::binds::mono_class_from_mono_type(ptr)
                }
            })
        } {
            res.push(class);
        }
        res
    }
    /// Returns the return type of method *self*, if no return type returns *System.Void*
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |self|&[`Method`]|Rust representation of a method to get return type off|
    pub fn get_return(&self) -> Class {
        let sig = unsafe { crate::binds::mono_method_signature(self.method) };
        let ptr = unsafe { crate::binds::mono_signature_get_return_type(sig) };
        unsafe {
            Class::from_ptr(crate::binds::mono_class_from_mono_type(ptr)).expect("Got no method return type, but no return type should be signaled by System.Void type!")
        }
    }
}
impl<Args: InteropSend + CompareClasses> Method<Args>
where
    <Args as InteropSend>::TargetType: TupleToPtrs,
{
    /// Invoke this method on object *object* with arguments *args*
    /// # Arguments
    /// | Name   | Type   | Description|
    /// |--------|--------|-------|
    /// | self   | &`Self`|Reference to method to invoke. |
    /// | object | [`Option<Object>`] |Object to invoke method on. Pass [`None`] if method is static. |
    /// | args   | `Args`|Arguments to pass to method |
    pub fn invoke(&self, object: Option<Object>, args: Args) -> Result<Option<Object>, Exception> {
        //convert object to invoke on to a pointer.
        let obj_ptr = match object {
            Some(obj) => obj.get_ptr(),
            None => core::ptr::null_mut(),
        };
        let mut expect: *mut MonoException = null_mut();
        //convert argument types
        let mut args = <Args as InteropSend>::get_mono_rep(args);
        let mut params =
            <<Args as InteropSend>::TargetType as TupleToPtrs>::get_ptrs(&mut args as *mut _);
        //invoke the method itself
        let res_ptr = unsafe {
            crate::binds::mono_runtime_invoke(
                self.get_ptr(),
                obj_ptr as *mut std::os::raw::c_void,
                &mut params as *mut _ as *mut *mut c_void,
                &mut expect as *mut *mut MonoException as *mut *mut MonoObject,
            )
        };
        //ensure args lives  as long as params lives.
        let _ = &args;
        //get result
        let res = unsafe { Object::from_ptr(res_ptr) };
        if expect.is_null() {
            Ok(res)
        } else {
            let except = unsafe {
                Exception::from_ptr(expect)
                    .expect("Imposible: pointer is null and not null at the same time.")
            };
            Err(except)
        }
    }
    /// Creates new Method type from a *mut MonoMethod, checks if arguments of [`MonoMethod`] and rust representation of a [`Method`] match and if not panic.
    /// Returns [`None`] if pointer is null.
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |met_ptr|*mut [`MonoMethod`]|Pointer to method to create a representation for.|
    /// # Safety
    /// Pointer must be either a valid pointer to [`MonoMethod`] recived from mono runtime, or a null pointer.
    pub unsafe fn from_ptr(met_ptr: *mut MonoMethod) -> Option<Self> {
        if met_ptr.is_null() {
            return None;
        }
        let res = Self {
            method: met_ptr,
            args_type: PhantomData,
        };
        let params = res.get_params();
        if !<Args as CompareClasses>::compare(&params) {
            use std::fmt::Write;
            let mut msg = format!(
                "Method Type Mismatch! Got a method accepting {} arguments of types:",
                params.len()
            );
            for param in params {
                write!(msg, ",\"{}\"", param.get_name_sig())
                    .expect("Could not print inproper function argument types!");
            }
            panic!("{}", msg);
        }
        Some(res)
    }
    /// Creates new Method type from a *mut MonoMethod, checks if arguments of [`MonoMethod`] and rust representation of a [`Method`] match and returns [`None`] if so.
    /// Returns [`None`] if pointer is null.
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |met_ptr|*mut [`MonoMethod`]|Pointer to method to create a representation for.|
    /// # Safety
    /// Pointer must be either a valid pointer to [`MonoMethod`] recived from mono runtime, or a null pointer.
    pub unsafe fn from_ptr_checked(met_ptr: *mut MonoMethod) -> Option<Self> {
        if met_ptr.is_null() {
            return None;
        }
        let res = Self {
            method: met_ptr,
            args_type: PhantomData,
        };
        let params = res.get_params();
        if !<Args as CompareClasses>::compare(&params) {
            return None;
        }
        Some(res)
    }
}
unsafe impl<Args: InteropSend + CompareClasses> Sync for Method<Args> where
    <Args as InteropSend>::TargetType: TupleToPtrs
{
}
