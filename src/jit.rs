use crate::binds::{mono_jit_init,mono_jit_init_version,mono_config_parse};
use crate::domain::{Domain};
use std::ffi::CString;
use core::ptr::null_mut;
pub fn init(name:&str,version:Option<&str>)->Domain{
    let n_cstr = CString::new(name).expect("could not create cstring!");
    let res = unsafe{Domain::create_from_ptr( match version{
        Some(s)=>{
            let v_cstr = CString::new(s).expect("could not create cstring!");
            let ptr = mono_jit_init_version(n_cstr.as_ptr(),v_cstr.as_ptr());
            drop(v_cstr);
            ptr
        },
        None=>{
            mono_jit_init(n_cstr.as_ptr())
        }
    })};
    unsafe{mono_config_parse (null_mut())};
    drop(n_cstr);
    return res;
}
