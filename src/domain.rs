use crate::binds::{MonoDomain, mono_domain_create,mono_domain_assembly_open};
use crate::assembly::{Assembly,AssemblyTrait};
use core::ptr::null_mut;
pub struct Domain{
    ptr:*mut MonoDomain,
} 
use std::ffi::CString;
impl Domain{
    pub unsafe fn create_from_ptr(ptr:*mut MonoDomain)->Domain{
        return Self{ptr:ptr};
    }
    pub unsafe fn get_ptr(&self)->*mut MonoDomain{
        return self.ptr;
    }
    pub fn assembly_open(&self,path:&str)->Option<Assembly>{
        let cstr = CString::new(path).expect("Couldn't create cstring!");
        let ptr = unsafe{mono_domain_assembly_open(self.get_ptr(),cstr.as_ptr())};
        if ptr == null_mut(){
            return None;
        }
        drop(cstr);
        return Some(unsafe{Assembly::create_from_ptr(ptr)});
    }
    pub fn create()->Domain{
        return unsafe{Self::create_from_ptr(mono_domain_create())};
    }
}