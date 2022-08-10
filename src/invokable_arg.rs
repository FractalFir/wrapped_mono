/// Trait used in creating wrappers around functions with #\[invokable\] atribute. It specifies how to convert a type
/// from SourceType used by MonoRuntime to type implementing this trait.
pub trait InvokableArg{
    ///Souce type used by MonoRuntime when calling functions exposed by add_internal_call, that can be converted to a rust type. 
    type SourceType;
    ///Type used by MonoRuntime, that type implementing InvokableArg tratit should be converted to when returnig it to MonoRuntime.
    type ReturnType;
    ///Function converting SourceType to type implementing InvokableArg tratit.
    fn get_rust_rep(mono_arg:Self::SourceType)->Self;
    ///Function converting type implementing InvokableArg tratit to type that should be returned to MonoRuntime.
    fn get_mono_rep(rust_arg:Self)->Self::ReturnType;
}
impl InvokableArg for usize{
    type SourceType = usize;
    type ReturnType = usize;
    fn get_rust_rep(mono_arg:Self::SourceType)->Self{
       return arg;
    }
    fn get_mono_rep(rust_arg:Self)->Self::ReturnType{
        return arg;
    }
}
impl InvokableArg for i32{
    type SourceType = i32;
    type ReturnType = i32;
    fn get_rust_rep(mono_arg:Self::SourceType)->Self{
       return arg;
    }
    fn get_mono_rep(rust_arg:Self)->Self::ReturnType{
        return arg;
    }
}
impl InvokableArg for String{
    type SourceType = *mut crate::binds::MonoString;
    type ReturnType = *mut crate::binds::MonoString;
    fn get_rust_rep(mono_arg:Self::SourceType)->Self{
        use std::ffi::CString;
        let cstr = unsafe{let sptr = crate::binds::mono_string_to_utf8(arg); CString::from_raw(sptr)};  
        return cstr.into_string().expect("Could not convert MonoString to String!");
    }
    fn get_mono_rep(rust_arg:Self)->Self::ReturnType{
        panic!("Not implemented yet!");
        //return arg;
    }
}