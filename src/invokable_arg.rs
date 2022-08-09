pub trait InvokableArg{
    type SourceType;
    type ResultType;
    fn get_iarg(arg:Self::SourceType)->Self;
}
impl InvokableArg for usize{
    type SourceType = usize;
    type ResultType = usize;
    fn get_iarg(arg:Self::SourceType)->Self{
       return arg;
    }
}
impl InvokableArg for i32{
    type SourceType = i32;
    type ResultType = i32;
    fn get_iarg(arg:Self::SourceType)->Self{
       return arg;
    }
}
impl InvokableArg for String{
    type SourceType = *mut crate::binds::MonoString;
    type ResultType = *mut crate::binds::MonoString;
    fn get_iarg(arg:Self::SourceType)->Self{
        use std::ffi::CString;
        let cstr = unsafe{let sptr = crate::binds::mono_string_to_utf8(arg); CString::from_raw(sptr)};  
        return cstr.into_string().expect("Could not convert MonoString to String!");
    }
}