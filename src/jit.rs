use crate::binds::{mono_jit_init,mono_jit_init_version,mono_config_parse,mono_jit_cleanup,mono_jit_exec};
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
pub fn cleanup(domain:Domain){
    unsafe{mono_jit_cleanup(domain.get_ptr())};
}
use crate::assembly::{Assembly,AssemblyTrait};
pub fn exec(domain:Domain,assembly:Assembly,args:Vec<&str>)->i32{
    let argc:i32 = args.len() as i32;
    let mut cstr_args:Vec<CString> = Vec::new();
    let mut argv:Vec<*mut i8> = Vec::new();
    for arg in args{
        let cstr_arg = CString::new(arg).unwrap();
        argv.push(cstr_arg.as_ptr() as *mut i8);  
        cstr_args.push(cstr_arg); 
    }
    let res = unsafe{mono_jit_exec(domain.get_ptr(),assembly.get_ptr(),argc,argv.as_mut_ptr())};
    drop(cstr_args);
    return res;
}
/*
pub fn mono_jit_exec(
        domain: *mut MonoDomain,
        assembly: *mut MonoAssembly,
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
*/