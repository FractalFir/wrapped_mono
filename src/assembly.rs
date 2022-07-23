use std::sync::Arc;
use crate::binds::{MonoAssembly,mono_assembly_close,mono_assembly_open,MonoImageOpenStatus}; 
use std::ffi::CString;
use crate::domain::{Domain};
use core::ptr::null_mut;
//must be public
pub struct _Assembly{
    ptr:*mut MonoAssembly,
    domain:Domain,
}
pub type Assembly = Arc<_Assembly>;
pub trait AssemblyTraits{
    fn create_from_ptr(ptr:*mut MonoAssembly,domain:Domain)->Assembly;
    fn open(domain:Domain,fpath:&str)->Result<Assembly,MonoImageOpenStatus>;
}
impl AssemblyTraits for Assembly{
    fn create_from_ptr(ptr:*mut MonoAssembly,domain:Domain)->Assembly{
        assert!(ptr!= null_mut());
        return Arc::new(_Assembly{ptr:ptr,domain:domain});
    }
    fn open(domain:Domain,fpath:&str)->Result<Assembly,MonoImageOpenStatus>{
        let mut status:MonoImageOpenStatus = 0;
        let cstr_fpath = CString::new(fpath).expect("Could not create cstring!");
        let res = unsafe{Assembly::create_from_ptr(mono_assembly_open(cstr_fpath.as_ptr(),&mut status),domain)};
        if status != 0{
            return Err(status);
        }
        return Ok(res);
    }
}

impl Drop for _Assembly{
    fn drop(&mut self){
        unsafe{mono_assembly_close(self.ptr)};
    }
}