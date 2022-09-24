/// Safe representation of a managed method.
pub struct Method{
    met_ptr:*mut crate::binds::MonoMethod,
} 
use crate::array::Array;
use crate::object::Object;
use crate::exception::Exception;
use crate::binds::{MonoObject};
use std::os::raw::c_void;
//necesary for docs
#[allow(unused_imports)]
use crate::class::Class;
#[warn(unused_imports)]
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
    ///Get's method named *name* in [`Class`] *class* with *param_count* parameters. If *param_count* is -1, function with any number of parameters is returned.
    /// # Example
    /// ## C#
    ///```csharp
    /// class SomeClass{
    ///     void SomeMethod(){}
    ///     void OtherMethod(int arg1, int arg2){}
    /// }
    ///```
    /// # Rust
    ///```rust
    /// let some_method = get_method_from_name(&some_class,"SomeMethod",0);
    /// let other_method = get_method_from_name(&some_class,"OtherMethod",2);
    ///```
    pub fn get_method_from_name(class:&crate::class::Class,name:&str,param_count:i32)->Option<Self>{
        let cstr = std::ffi::CString::new(name).expect(crate::STR2CSTR_ERR);
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
    //TODO:finish this documentaion
    pub fn get_index(&self)->u32{
        return unsafe{crate::binds::mono_method_get_index(self.get_ptr())};
    }
    ///Get ammount of parameters of this [`Method`]
    pub fn get_param_count(&self)->u32{
        let sig = unsafe{crate::binds::mono_method_signature(self.get_ptr())};
        return unsafe{crate::binds::mono_signature_get_param_count(sig)};
    }
    ///Returns the name of this method
    pub fn get_name(&self)->String{
        let cstr = unsafe{std::ffi::CString::from_raw(crate::binds::mono_method_get_name(self.get_ptr()) as *mut i8)};
        let s = cstr.to_str().expect("Could not converted ptr to String!").to_owned();
        let _ = cstr.into_raw();
        return s;
    }
    ///TODO:finish this function
    fn invoke_array(&self,_obj:Option<Object>,_arr:Array<Option<Object>>)->Result<Object,Exception>{
        unimplemented!("Not done yet");
    }
    ///Gets class this method is attached to
    pub fn get_class(&self)->crate::class::Class{
        return unsafe{crate::class::Class::from_ptr(
            crate::binds::mono_method_get_class(self.get_ptr())
        ).expect("Could not get class of a method")};
    }
    //mono_signature_get_return_type(sig: *mut MonoMethodSignature) -> *mut MonoType;
    ///Gets names of all parameters this function accepts.
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
            let _ = cstr.into_raw();
        }
        drop(ptrs);
        return res;
    }
    ///Returns list of all parameters this function accepts.
    pub fn get_params(&self)->Vec<Class>{
        use std::ptr::null_mut;
        let sig = unsafe{crate::binds::mono_method_signature(self.met_ptr)};
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
    ///Invokes method *self*. *obj* is the `this` object(object method is called on). Pass [`None`] if method is static.
    ///**Doesn't** handle virtual methods, calls the method passed. To handle virtual methods, first get virtual method from object it is called on.
    ///Use `method_invoke!` instead for safety reasons. 
    pub unsafe fn invoke_unsafe(&self,obj:Option<&Object>,params:&[*mut std::os::raw::c_void])->Result<Option<Object>,Exception>{
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
        let res = Object::from_ptr(res_ptr);
        if expect == null_mut(){
            return Ok(res);
        }
        else {
            let except = Exception::from_ptr(expect).expect("Imposible: pointer is null and not null at the same time.");
            return Err(except); 
        }
    }
    //Returns return type of the function
    pub fn get_return(&self)->Option<Class>{
        use std::ptr::null_mut;
        let sig = unsafe{crate::binds::mono_method_signature(self.met_ptr)};
        let ptr = unsafe{crate::binds:: mono_signature_get_return_type(sig)};
        let res = unsafe{Class::from_ptr({if ptr == null_mut(){null_mut()}else{crate::binds::mono_class_from_mono_type(ptr)}})};
        return res;
    } 
}
impl std::fmt::Display for Method{
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
        match self.get_return(){
            Some(ret)=>write!(f,"{} ",&ret.get_name())?,
            None=>(),
        }
        write!(f,"{}:{}(",&self.get_class().get_name_sig(),&self.get_name())?;
        let param_types = self.get_params();
        let param_names = self.get_param_names();
        let pcount:usize = self.get_param_count() as usize;
        for i in 0..pcount{
            write!(f,"{}:{}",param_names[i],&param_types[i].get_name())?;
            if i < pcount - 1{
                write!(f,", ")?;
            }
        }
        write!(f,")")
    }
}
use core::marker::PhantomData;
use crate::{InteropRecive,InteropSend};
use crate::binds::MonoMethod;
use crate::tupleutilis::*;
//#![feature(specialization)]
//New Mehtod type, WIP
struct NewMethod<Args:InteropSend>{
    method:*mut MonoMethod,
    args_type:PhantomData<Args>,
}
trait MethodTrait<Args:InteropSend> {
    fn invoke(&self,object:Option<Object>,args:Args)->Result<Option<Object>,Exception>;
}
impl <Args:InteropSend> NewMethod<Args> {
    pub unsafe fn from_ptr(met_ptr:*mut crate::binds::MonoMethod)->Option<Self>{
        if met_ptr == core::ptr::null_mut(){
            return None;
        }
        return Some(Self{method:met_ptr,args_type:PhantomData});
    }
    pub fn get_ptr(&self)->*mut crate::binds::MonoMethod{
        return self.method;
    }
}
impl <Args:InteropSend> MethodTrait<Args> for NewMethod<Args>{
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
impl <Args:InteropSend> MethodTrait<Args> for NewMethod<Args> where <Args as InteropSend>::TargetType:TupleToPtrs{
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