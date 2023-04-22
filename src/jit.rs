use crate::binds::{mono_jit_cleanup, mono_jit_exec, mono_jit_init, mono_jit_init_version};
use crate::domain::Domain;
use std::ffi::CString;
static mut HAS_BEEN_INITIALIZED: bool = false;
/// This function starts up MonoRuntime,and returns main domain. It should be called before any other mono function is called. **Can be only called once per process.**
/// Version argument specifies runtime version, if **None** passed, default version will be selected.
/// ```no_run
/// # use wrapped_mono::*;
/// let main_domain = jit::init("domain_name",None);
/// ```
/// ```no_run
/// # use wrapped_mono::*;
/// let main_domain_with_version = jit::init("domain_name",Some("v4.0.30319"));
/// ```
/// # Panics
/// Panics if the runtime is initialised second time. `MonoRuntime` can be only initialised once.
pub fn init(name: &str, version: Option<&str>) -> Domain {
    unsafe {
        assert!(
            !HAS_BEEN_INITIALIZED,
            "Mono runtime can't be initialized twice in the same process."
        );
        HAS_BEEN_INITIALIZED = true;
    }
    let n_cstr = CString::new(name).expect(crate::STR2CSTR_ERR);
    let res = unsafe {
        Domain::from_ptr(version.map_or_else(
            || mono_jit_init(n_cstr.as_ptr()),
            |s| {
                let v_cstr = CString::new(s).expect(crate::STR2CSTR_ERR);
                let ptr = mono_jit_init_version(n_cstr.as_ptr(), v_cstr.as_ptr());
                let _ = &v_cstr;
                ptr
            },
        ))
    };
    unsafe { crate::binds::mono_jit_thread_attach(res.get_ptr()) };
    let _ = &n_cstr;
    res
}
/// This function shuts down the `MonoRuntime`.
/// **WARNING!** after it is called, `MonoRuntime` **will not be** able to be used again in the same process, since it can be only started up once.
/// ```no_run
/// # use wrapped_mono::*;
/// let main_domain = jit::init("main",None);
/// // All code using MonoRuntime goes here
/// jit::cleanup(main_domain);
/// ```
pub fn cleanup(domain: Domain) {
    unsafe { mono_jit_cleanup(domain.get_ptr()) };
}
use crate::assembly::Assembly;
/// Function used to call main function from assembly in domain with arguments.
/// ```ignore
/// //C# code in file "SomeAssembly.dll"
/// class Apllication{
/// public static void Main(string args[]){
///     /*Some C# code*/   
///     }
/// }
/// ```
/// ```no_run
/// # use wrapped_mono::*;
/// let main_domain = jit::init("main",None);
/// let asm = main_domain.assembly_open("SomeAssembly.dll").expect("Could not open the assembly!");
/// let args = vec!["arg1","arg2","arg3"];
/// let res = jit::exec(&main_domain,&asm,args);
/// ```
/// # Errors
/// Returns err if could not convert an arg to a `CString`. The `err` will contain the index of the invalid argument.
pub fn exec(domain: &Domain, assembly: &Assembly, args: Vec<&str>) -> Result<i32, usize> {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_possible_wrap)]
    let argument_count: i32 = (args.len() + 1) as i32;
    let mut cstr_args: Vec<CString> = Vec::new();
    let mut argument_vector: Vec<*mut i8> = Vec::with_capacity(args.len() + 1);
    // 1-st argument is expected to be assembly name
    unsafe {
        argument_vector.push(crate::binds::mono_stringify_assembly_name(
            crate::binds::mono_assembly_get_name(assembly.get_ptr()),
        ));
    }
    for (index, arg) in args.into_iter().enumerate() {
        let Ok(cstr_arg) = CString::new(arg) else { return Err(index) };
        argument_vector.push(cstr_arg.as_ptr() as *mut i8);
        cstr_args.push(cstr_arg);
    }
    let res = unsafe {
        mono_jit_exec(
            domain.get_ptr(),
            assembly.get_ptr(),
            argument_count,
            argument_vector.as_mut_ptr(),
        )
    };
    let _ = &cstr_args;
    Ok(res)
}
