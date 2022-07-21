use std::sync::Arc;
use crate::binds::{MonoDomain,
    mono_domain_free,mono_jit_cleanup,
    mono_domain_create,mono_domain_create_appdomain,
    mono_domain_set_config,
    mono_jit_init,mono_jit_init_version
};
use std::ffi::{CString};
pub type Domain = Arc<_Domain>;
pub struct _Domain {
    pub ptr:*mut MonoDomain,
}
impl Drop for _Domain{
    fn drop(&mut self){
        unsafe{mono_domain_free(self.ptr,0)};
    }
}
pub trait DomainTraits{
    //this function creates root domain and initlizes mono jit
    fn init_jit(name:Option<&str>,version:Option<&str>)->Domain;
    //fucntion used to create new domains with default names and no config
    fn create()->Self;
    fn create_appdomain(name:&str,cfg_file:&str)->Self;
    //function used to set domain config
    fn set_config(&mut self,base_dir:&str,cfg_path:&str);
    //function returning pointer to MonoDomai
    unsafe fn get_ptr(&self)->*mut MonoDomain;
    //this function stops jit. WARNIG! you will not be able to reinitialize mono after calling this function! Does not work yet.
    fn jit_cleanup(&self);
}
impl DomainTraits for Domain{
    fn init_jit(name:Option<&str>,version:Option<&str>)->Domain{
        let cstr_name = CString::new(match name{
            Some(name)=>name,
            None=>"Defult",
        }).expect("Could not create cstring!");
        let res = Arc::new(_Domain{ptr:match version{
            Some(version) => {
                let cstr_ver = CString::new(version).expect("Could not create cstring!");
                let res = unsafe{mono_jit_init_version(cstr_name.as_ptr()as *mut i8,cstr_ver.as_ptr() as *mut i8)};
                drop(cstr_ver);
                res
            },
            None=>unsafe{mono_jit_init(cstr_name.as_ptr()as *mut i8)},
        }});
        drop(cstr_name);
        return res;
    }
    fn jit_cleanup(&self){
        panic!("Function not implemented yet.");
        unsafe{mono_jit_cleanup(self.ptr)};
    }
    fn create()->Domain{
        return Arc::new(_Domain{ptr:unsafe{mono_domain_create()}});
    }
    fn create_appdomain(name:&str,cfg_file:&str)->Domain{
        let cstr_name = CString::new(name).expect("Could not create cstring!");
        let cst_cfg_file = CString::new(cfg_file).expect("Could not create cstring!");
        return Arc::new(_Domain{ptr:unsafe{mono_domain_create_appdomain(cstr_name.as_ptr() as *mut i8,cst_cfg_file.as_ptr() as *mut i8)}});
    }
    unsafe fn get_ptr(&self)->*mut MonoDomain{
        return self.ptr;
    }
    fn set_config(&mut self,base_dir:&str,cfg_file:&str){
        let cstr_base_dir = CString::new(base_dir).expect("Could not create cstring!");
        let cst_cfg_file = CString::new(cfg_file).expect("Could not create cstring!");
        unsafe{mono_domain_set_config(self.get_ptr(),cstr_base_dir.as_ptr() as *mut i8,cst_cfg_file.as_ptr() as *mut i8)};
    }
}   