/// Trait used in creating wrappers around functions with #\[invokable\] atribute. It specifies how to convert
/// SourceType used by MonoRuntime to type implementing this trait.
pub trait InvokePass{
    ///Souce type used by MonoRuntime when calling functions exposed by add_internal_call, that can be converted to a rust type. 
    type SourceType:Copy;
    ///Function converting SourceType to type implementing InvokePass trait.
    fn get_rust_rep(mono_arg:Self::SourceType)->Self;  
}
/// Trait used in creating wrappers around functions with #\[invokable\] atribute. It specifies how to convert type implementing this trait
/// to ReturnType used by MonoRuntime.
pub trait InvokeReturn{
    ///Type used by MonoRuntime, that type implementing InvokeReturn trait should be converted to when returnig it to MonoRuntime.
    type ReturnType:Copy;
    ///Function converting type implementing InvokePass trait to type that should be returned to MonoRuntime.
    fn get_mono_rep(rust_arg:Self)->Self::ReturnType;
}
impl InvokePass for usize{
    type SourceType = usize;
    fn get_rust_rep(mono_arg:Self::SourceType)->Self{
       return mono_arg;
    }
}
impl InvokePass for i32{
    type SourceType = i32;
    fn get_rust_rep(mono_arg:Self::SourceType)->Self{
        return mono_arg;
    }
}
impl InvokePass for String{
    type SourceType = *mut crate::binds::MonoString;
    fn get_rust_rep(mono_arg:Self::SourceType)->Self{
        use std::ffi::CString;
        let cstr = unsafe{let sptr = crate::binds::mono_string_to_utf8(mono_arg); CString::from_raw(sptr)};  
        return cstr.into_string().expect("Could not convert MonoString to String!");
    }
}
impl InvokePass for f32{
    type SourceType = f32;
    fn get_rust_rep(mono_arg:Self::SourceType)->Self{
        return mono_arg;
    }
}
//return section
impl InvokeReturn for i32{
    type ReturnType = i32;
    fn get_mono_rep(rust_arg:Self)->Self::ReturnType{
        return rust_arg;
    }
}
impl InvokeReturn for f32{
    type ReturnType = f32;
    fn get_mono_rep(rust_arg:Self)->Self::ReturnType{
        return rust_arg;
    }
}
