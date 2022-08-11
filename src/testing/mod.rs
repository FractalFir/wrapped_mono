use rusty_fork::rusty_fork_test;
mod pinvoke;
rusty_fork_test! {
    #[test]
    fn jit_init(){
        use crate::jit;
        let dom = jit::init("root",None);
    }
    #[test]
    fn jit_init_version(){
        use crate::jit;
        let dom = jit::init("root",Some("v4.0.30319"));
    }
    #[test]
    fn multiple_domains(){
        use crate::{domain::Domain,jit};
        let dom = jit::init("root",None);
        let dom2 = Domain::create();
    }
    #[test]
    fn assembly_loading(){
        use crate::{domain::Domain,jit};
        let dom = jit::init("root",None);
        dom.assembly_open("test/local/Test.dll").unwrap();
    }
    #[should_panic]
    #[test]
    fn missing_assembly_loading(){
        use crate::{domain::Domain,jit};
        let dom = jit::init("root",None);
        dom.assembly_open("test/local/Missing.dll").unwrap();
    }
    #[test]
    fn stop_jit(){
        use crate::{jit};
        let dom = jit::init("root",None);
        jit::cleanup(dom);
    }
    #[test]
    fn getting_image_from_assembly(){
        use crate::{domain::Domain,jit};
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/local/Test.dll").unwrap();
        let img = asm.get_image();
    }
    #[test]
    fn gettig_class_from_image(){
        use crate::{domain::Domain,jit,class::Class};
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/local/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(img,"","TestFunctions");
    }
} 
