pub mod binds;
pub mod jit;
pub mod domain;
pub mod assembly;
pub mod invokable;
use macros::invokable;
use rusty_fork::rusty_fork_test;
use core::ptr::null_mut;
#[invokable]
fn test_fnc(test_string:&str){
    return str.len();
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
        let asm = dom.assembly_open("test/local/Exec.dll").unwrap();
        let mut args:Vec<&str> = Vec::new();
        args.push("1");
        args.push("2");
        assert!(2 == jit::exec(dom,asm,args));
    }
    #[test]
    fn p_invoke(){
        //testing makro function
        invokable_test_fnc();
    }
}