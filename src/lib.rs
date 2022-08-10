//! Rust wrapper around mono runtime
/// Autognerated, unsafe binds to mono library
pub mod binds;
/// Functions realted to Mono JIT Runtime
pub mod jit;
/// Functions and types related to MonoDomain type.
pub mod domain;
/// Functions and types related to MonoAssemblt type.
pub mod assembly;
/// Trait related to converting Rust's types and MonoRuntime's types when exposing rust functios to managed code
pub mod invokable;
/// Utilities related to arrays.
pub mod array;
mod testing;
use macros::{invokable,add_internal_call};
use rusty_fork::rusty_fork_test;
use core::ptr::null_mut;
use invokable::InvokePass;
rusty_fork_test! {
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
    
}