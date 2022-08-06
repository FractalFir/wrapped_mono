pub mod binds;
pub mod jit;
pub mod domain;
pub mod assembly;
pub mod invokable_arg;
use macros::invokable;
use rusty_fork::rusty_fork_test;
use core::ptr::null_mut;
use invokable_arg::InvokableArg;
           
#[invokable]
fn pass_arg_count(test_val:i32){
    println!("args:{}",test_val);
    assert!(test_val == 2);
    panic!();
}
rusty_fork_test! {
    #[test]
    fn jit_init(){
        let dom = jit::init("root",None);
    }
    #[test]
    fn jit_init_version(){
        let dom = jit::init("root",Some("v4.0.30319"));
    }
    #[test]
    fn multiple_domains(){
        use crate::domain::Domain;
        let dom = jit::init("root",None);
        let dom2 = Domain::create();
    }
    #[test]
    fn assembly_loading(){
        use crate::domain::Domain;
        let dom = jit::init("root",None);
        dom.assembly_open("test/local/Test.dll").unwrap();
    }
    #[should_panic]
    #[test]
    fn missing_assembly_loading(){
        use crate::domain::Domain;
        let dom = jit::init("root",None);
        dom.assembly_open("test/local/Missing.dll").unwrap();
    }
    #[test]
    fn stop_jit(){
        let dom = jit::init("root",None);
        jit::cleanup(dom);
    }
    #[test]
    fn jit_execution(){
        use crate::domain::Domain;
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/local/Jit.dll").unwrap();
        let mut args:Vec<&str> = Vec::new();
        args.push("1");
        args.push("2");
        let res = jit::exec(dom,asm,args);
    }
    #[test]
    fn p_invoke(){
        use std::ffi::{CString,c_void};
        use crate::domain::Domain;
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/local/Pinvoke.dll").unwrap();
        let mut args:Vec<&str> = Vec::new();
        args.push("1");
        args.push("2");
        //passing function bind
        let cstr = CString::new("Test::PassArgCount").expect("Could not create cstring");
        let fnc_ptr:*const c_void = unsafe{std::mem::transmute(&pass_arg_count_invokable)};
        println!("ptr:{:#x}",fnc_ptr as usize);
        unsafe{binds::mono_add_internal_call(cstr.as_ptr(),fnc_ptr)};
        drop(cstr);
        let res = jit::exec(dom,asm,args);
        panic!();
    }
}