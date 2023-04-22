use core::ptr::null_mut;
use std::ffi::CString;
// Necesaty for docs to work
#[allow(unused_imports)]
use crate::jit;
#[warn(unused_imports)]
///Sets paths to directories containing manged assemblies and config files. If [`None`] passed for *`assembly_dir`*,
///default system location for assemblies will be used. If [`None`] passed for *`config_dir`* default system configs will be used.
pub fn set_dirs(assembly_dir: Option<&str>, config_dir: Option<&str>) {
    assembly_dir.map_or_else(
        || {
            config_dir.map_or_else(
                || {
                    unsafe { crate::binds::mono_set_dirs(null_mut(), null_mut()) };
                },
                |config_dir| {
                    let cfg_cstr = CString::new(config_dir).expect(crate::STR2CSTR_ERR);
                    unsafe { crate::binds::mono_set_dirs(null_mut(), cfg_cstr.as_ptr()) };
                    let _ = cfg_cstr;
                },
            )
        },
        |assembly_dir| {
            let asm_cstr = CString::new(assembly_dir).expect(crate::STR2CSTR_ERR);
            config_dir.map_or_else(
                || {
                    unsafe { crate::binds::mono_set_dirs(asm_cstr.as_ptr(), null_mut()) };
                },
                |config_dir| {
                    let cfg_cstr = CString::new(config_dir).expect(crate::STR2CSTR_ERR);
                    unsafe { crate::binds::mono_set_dirs(asm_cstr.as_ptr(), cfg_cstr.as_ptr()) };
                    let _ = cfg_cstr;
                },
            );
            let _ = asm_cstr;
        },
    )
}
///Load config from file *`fname`*, or defalut config if *`fname`* is none. Default config will be either the default system config or
///file in directory *`config_dir`* if set using [`set_dirs`] function.
pub fn config_parse(fname: Option<&str>) {
    fname.map_or_else(
        || unsafe { crate::binds::mono_config_parse(null_mut()) },
        |fname| {
            let cstr = CString::new(fname).expect(crate::STR2CSTR_ERR);
            unsafe { crate::binds::mono_config_parse(cstr.as_ptr()) };
            let _ = cstr;
        },
    )
}
///Load config from string in memory. *`config`* must be an string representing XML configuration.
pub fn config_parse_memory(config: &str) {
    let cstr = CString::new(config).expect(crate::STR2CSTR_ERR);
    unsafe { crate::binds::mono_config_parse_memory(config.as_ptr().cast::<i8>()) };
    let _ = cstr;
}
//TODO: impl mono_jit_set_aot_mode
//TODO: impl mono_set_break_policy
/// Gets runtime version and build date as a string in format `VERSION (FULL_VERSION BUILD_DATE)`
#[must_use]
pub fn get_runtime_build_info() -> String {
    let cstr = unsafe { CString::from_raw(crate::binds::mono_get_runtime_build_info()) };
    let build_info_msg = cstr.to_str().expect("Could not create String").to_owned();
    let _ = cstr;
    build_info_msg
}
/// Enable/Disable signal chaining. If it is enabled, runtime saves original signal handlers and passes certain signals to them.
/// # Constraints
/// Should be called before [`jit::init`] in order for signals to be properly chained.
/// # Signals
/// ## **SIGSEGV** and **SIGABRT**
/// Those signals will be called when received while executing native code (code not run inside runtime)
pub fn set_signal_chaining(chain_signals: bool) {
    unsafe { crate::binds::mono_set_signal_chaining(i32::from(chain_signals)) };
}
/// Checks if currently loaded version of corelib will work with this runtime. Returns nothing if it will, and error message if it will not.
/// # Errors
/// Returns an error message if corelib is mismatched.
pub fn check_corelib() -> Result<(), String> {
    let ptr = unsafe { crate::binds::mono_check_corlib_version() };
    if ptr.is_null() {
        Ok(())
    } else {
        let cstr = unsafe { CString::from_raw(ptr as *mut i8) };
        let res = cstr.to_str().expect("Could not create String.").to_owned();
        let _ptr = cstr.into_raw();
        Err(res)
    }
}
