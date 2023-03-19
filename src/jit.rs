use crate::binds::{mono_jit_cleanup, mono_jit_exec, mono_jit_init, mono_jit_init_version};
use crate::domain::Domain;
use std::ffi::CString;
static mut HAS_BEEN_INITIALIZED: bool = false;
/// This function starts up MonoRuntime,and returns main domain. It should be called before any other mono function is called. **Can be only called once per process.**
/// Version argument specifies runtime version, if **None** passed, default version will be selected.
/// ```rust
/// let main_domain = jit::init("domain_name",None);
/// ```
/// ```rust
/// let main_domain_with_version = jit::init("domain_name","v4.0.30319");
/// ```
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
        Domain::from_ptr(match version {
            Some(s) => {
                let v_cstr = CString::new(s).expect(crate::STR2CSTR_ERR);
                let ptr = mono_jit_init_version(n_cstr.as_ptr(), v_cstr.as_ptr());
                crate::hold(&v_cstr);
                ptr
            }
            None => mono_jit_init(n_cstr.as_ptr()),
        })
    };
    unsafe { crate::binds::mono_jit_thread_attach(res.get_ptr()) };
    crate::hold(&n_cstr);
    res
}
/// This function shuts down MonoRuntime.
/// **WARNING!** after it is called, MonoRuntime **will not be** able to be used again in the same process, since it can be only started up once.
/// ```rust
/// let main_domain = jit::init("main",None);
/// // All code using MonoRuntime goes here
/// jit::cleanup(main_domain);
/// ```
pub fn cleanup(domain: Domain) {
    unsafe { mono_jit_cleanup(domain.get_ptr()) };
}
use crate::assembly::Assembly;
/// Function used to call main function from assembly in domain with arguments.
/// ```csharp
/// //C# code in file "SomeAssembly.dll"
/// class Apllication{
/// public static void Main(string args[]){
///     /*Some C# code*/   
///     }
/// }
/// ```
/// ```rust
/// let main_doamin = jit::init("main",None);
/// let asm = main_domain.assembly_open("SomeAssembly.dll");
/// let args = vec!["arg1","arg2","arg3"];
/// let res = jit::exec(main_domain,asm,args);
/// ```
pub fn exec(domain: &Domain, assembly: &Assembly, args: Vec<&str>) -> i32 {
    let argc: i32 = args.len() as i32 + 1;
    let mut cstr_args: Vec<CString> = Vec::new();
    let mut argv: Vec<*mut i8> = Vec::with_capacity(args.len() + 1);
    // 1-st argument is expected to be assembly name
    unsafe {
        argv.push(crate::binds::mono_stringify_assembly_name(
            crate::binds::mono_assembly_get_name(assembly.get_ptr()),
        ))
    };
    for arg in args {
        let cstr_arg = CString::new(arg).unwrap();
        argv.push(cstr_arg.as_ptr() as *mut i8);
        cstr_args.push(cstr_arg);
    }
    let res = unsafe {
        mono_jit_exec(
            domain.get_ptr(),
            assembly.get_ptr(),
            argc,
            argv.as_mut_ptr(),
        )
    };
    crate::hold(&cstr_args);
    res
}
