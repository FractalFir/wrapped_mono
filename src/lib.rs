pub mod binds;
pub mod jit;
pub mod domain;
pub mod assembly;
use rusty_fork::rusty_fork_test;
use core::ptr::null_mut;
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
}