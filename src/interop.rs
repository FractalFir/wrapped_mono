/// Tratit specifing how to convert a type when transfering it between managed and unmanaged code. It specifies how to convert
/// SourceType used by MonoRuntime to type implementing this trait.
pub trait InteropRecive{
    ///Souce type used by MonoRuntime when calling functions exposed by add_internal_call, that can be converted to a rust type. 
    type SourceType:Copy;
    ///Function converting SourceType to type implementing InteropRecive trait.
    fn get_rust_rep(mono_arg:Self::SourceType)->Self;  
}
/// Tratit specifing how to convert a type when transfering it between managed and unmanaged code. It specifies how to convert type implementing this trait
/// to TargetType used by MonoRuntime.
pub trait InteropSend{
    ///Type used by MonoRuntime, that type implementing InteropSend trait should be converted to when returnig it to MonoRuntime.
    type TargetType:Copy;
    ///Function converting type implementing InteropRecive trait to type that should be returned to MonoRuntime.
    fn get_mono_rep(rust_arg:Self)->Self::TargetType;
}
impl InteropRecive for usize{
    type SourceType = usize;
    fn get_rust_rep(mono_arg:Self::SourceType)->Self{
       return mono_arg;
    }
}
impl InteropRecive for isize{
    type SourceType = isize;
    fn get_rust_rep(mono_arg:Self::SourceType)->Self{
       return mono_arg;
    }
}
impl InteropRecive for i32{
    type SourceType = i32;
    fn get_rust_rep(mono_arg:Self::SourceType)->Self{
        return mono_arg;
    }
}
impl InteropRecive for i64{
    type SourceType = i64;
    fn get_rust_rep(mono_arg:Self::SourceType)->Self{
        return mono_arg;
    }
}
impl InteropRecive for String{
    type SourceType = *mut crate::binds::MonoString;
    fn get_rust_rep(mono_arg:Self::SourceType)->Self{
        use std::ffi::CString;
        let cstr = unsafe{CString::from_raw(crate::binds::mono_string_to_utf8(mono_arg))};  
        let res = cstr.to_str().expect("Could not convert MonoString to String!").to_owned();
        unsafe{crate::binds::mono_free(cstr.into_raw() as *mut std::os::raw::c_void)};
        return res;
    }
}
impl InteropRecive for f32{
    type SourceType = f32;
    fn get_rust_rep(mono_arg:Self::SourceType)->Self{
        return mono_arg;
    }
}
impl InteropRecive for f64{
    type SourceType = f64;
    fn get_rust_rep(mono_arg:Self::SourceType)->Self{
        return mono_arg;
    }
}
impl<T> InteropRecive for *mut T{
    type SourceType = *mut T;
    fn get_rust_rep(mono_arg:Self::SourceType)->Self{
        return mono_arg;
    }
}
//return section
impl InteropSend for i32{
    type TargetType = i32;
    fn get_mono_rep(rust_arg:Self)->Self::TargetType{
        return rust_arg;
    }
}
impl InteropSend for i64{
    type TargetType = i64;
    fn get_mono_rep(rust_arg:Self)->Self::TargetType{
        return rust_arg;
    }
}
impl InteropSend for f32{
    type TargetType = f32;
    fn get_mono_rep(rust_arg:Self)->Self::TargetType{
        return rust_arg;
    }
}
impl InteropSend for f64{
    type TargetType = f64;
    fn get_mono_rep(rust_arg:Self)->Self::TargetType{
        return rust_arg;
    }
}
impl InteropSend for usize{
    type TargetType = usize;
    fn get_mono_rep(rust_arg:Self)->Self::TargetType{
        return rust_arg;
    }
}
impl InteropSend for isize{
    type TargetType = isize;
    fn get_mono_rep(rust_arg:Self)->Self::TargetType{
        return rust_arg;
    }
}
impl<T> InteropSend for *mut T{
    type TargetType = *mut T;
    fn get_mono_rep(mono_arg:Self::TargetType)->Self{
        return mono_arg;
    }
}

use crate::class::Class;
///Trait allowing for boxing and unboxing type from objects 
/// # Safety
/// Type of value `Self::InteropSend::TargetType` must match managed type represented by [`Class`] returned by `get_mono_class`,
/// otherwise this function may either crash or box garbage value.
/// Managed type returned by `get_mono_class` **must** be boxable, otherwise a crash will occur.
pub trait InteropBox where Self: InteropRecive + InteropSend{
    fn get_mono_class()->Class;
}
impl InteropBox for i64{
    fn get_mono_class()->Class{
        return Class::get_int_64();
    }
}
impl InteropBox for i32{
    fn get_mono_class()->Class{
        return Class::get_int_32();
    }
}
impl InteropBox for f32{
    fn get_mono_class()->Class{
        return Class::get_single();
    }
}
impl InteropBox for f64{
    fn get_mono_class()->Class{
        return Class::get_double();
    }
}
impl InteropBox for isize{
    fn get_mono_class()->Class{
        return Class::get_int_ptr();
    }
}