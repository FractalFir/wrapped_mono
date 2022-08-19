use std::ffi::CString;
use core::ptr::null_mut;
// Necesaty for docs to work
#[allow(unused_imports)]
use crate::jit;
#[warn(unused_imports)]
///Sets paths to directories contining manged assemblies and config files. If [`None`] passed for *assembly_dir*, 
///default system location for assemblies will be used. If [`None`] passed for *config_dir* defalut system configs will be used.
pub fn set_dirs(assembly_dir:Option<&str>,config_dir:Option<&str>){
    match assembly_dir{
        Some(assembly_dir)=>{
            let asm_cstr = CString::new(assembly_dir).expect("Could not create CString");
            match config_dir{
                Some(config_dir)=>{
                    let cfg_cstr = CString::new(config_dir).expect("Could not create CString");
                    unsafe{crate::binds::mono_set_dirs(asm_cstr.as_ptr(),cfg_cstr.as_ptr())};
                    drop(cfg_cstr);
                }
                None=>{
                    unsafe{crate::binds::mono_set_dirs(asm_cstr.as_ptr(),null_mut())};
                }
            }
            drop(asm_cstr)
        }
        None=>{
            match config_dir{
                Some(config_dir)=>{
                    let cfg_cstr = CString::new(config_dir).expect("Could not create CString");
                    unsafe{crate::binds::mono_set_dirs(null_mut(),cfg_cstr.as_ptr())};
                    drop(cfg_cstr);
                }
                None=>{
                    unsafe{crate::binds::mono_set_dirs(null_mut(),null_mut())};
                }
            }
        }
    }
}
///Load config from file *fname*, or defalut config if *fname* is none. Defalut config will be either the defalut system config or
///file in drectory *config_dir* if set using [`set_dirs`] function.
pub fn config_parse(fname:Option<&str>){
    match fname{
        Some(fname)=>{
            let cstr = CString::new(fname).expect("Could not create CString");
            unsafe{crate::binds::mono_config_parse(cstr.as_ptr())};
            drop(cstr);
        },
        None=>unsafe{crate::binds::mono_config_parse(null_mut())},
    }
}
///Load config from string in memory. *config* must be an string representing XML configuration.
pub fn config_parse_memory(config:&str){
    let cstr = CString::new(config).expect("Could not create CString");
    unsafe{crate::binds::mono_config_parse_memory(config.as_ptr() as *const i8)};
    drop(cstr);
}
//TODO: impl mono_jit_set_aot_mode
//TODO: impl mono_set_break_policy
/// Gets runtime version and build date as a string in format `VERSION (FULL_VERSION BUILD_DATE)`
pub fn get_runtime_build_info()->String{
    let cstr = unsafe{CString::from_raw(crate::binds::mono_get_runtime_build_info())};
    let s = cstr.to_str().expect("Could not create String").to_owned();
    drop(cstr);
    return s;
}
/// Enable/Disable signal chaing. If it is enabled, runtime saves original singal handlers and passes ceratin signals to them. 
/// # Constraints
/// Should be called before [`jit::init`] in order for singals to be propely chained.
/// # Signals
/// ## **SIGSEGV** and **SIGABRT**
/// Those singals will be called when recived while executing native code (code not run inside runtime) 
pub fn set_signal_chaining(chain_signals:bool){
    unsafe{crate::binds::mono_set_signal_chaining(chain_signals as i32)};
}
///Checks if currently loaded version of corelib will work with this runtime. Returns nothing if it will, and error message if it will not.
pub fn check_corelib()->Result<(),String>{
    let ptr = unsafe{crate::binds::mono_check_corlib_version()};
    if ptr == null_mut(){
        return Ok(());
    }
    else {
        let cstr = unsafe{CString::from_raw(ptr as *mut i8)};
        let res = cstr.to_str().expect("Could not create String.").to_owned();
        let _ptr = cstr.into_raw();
        return Err(res);
    }
}