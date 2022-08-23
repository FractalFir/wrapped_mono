#![cfg(test)]
use rusty_fork::rusty_fork_test;
use crate as wrapped_mono;
rusty_fork_test!{
    #[test]
    fn test_interface_iteration(){
        use wrapped_mono::*;
        let domain = jit::init("main",None);
        let asm = domain.assembly_open("test/dlls/Test.dll").expect("Could not load assembly");
        let img = asm.get_image();
        let test_class = Class::from_name(&img,"","TestFunctions").expect("Could not find class");
        let ifaces = test_class.get_interfaces();
        println!("{}",&test_class.get_name());
        assert!(ifaces.len() > 0);
        assert!("IInterfaceOne" == &ifaces[0].get_name());
    }
    #[test]
    fn class_get_namespace(){
        use crate as wrapped_mono;
        use wrapped_mono::jit;
        use wrapped_mono::object::*;
        use wrapped_mono::class::Class;
        let _main = jit::init("main",None);
        let class = Class::get_int_64();
        assert!("System" == &class.get_namespace());
    }
    #[test]
    fn class_get_namespace_no_namespace(){
        use wrapped_mono::*;
        let domain = jit::init("main",None);
        let asm = domain.assembly_open("test/dlls/Test.dll").expect("Could not load assembly");
        let img = asm.get_image();
        let test_class = Class::from_name(&img,"","TestFunctions").expect("Could not find class");
        assert!("" == &test_class.get_namespace());
    }
    #[test]
    fn class_get_array_element_class(){
        use wrapped_mono::*;
        let domain = jit::init("main",None);
        let arr:Array<i64> = Array::new(&domain,32);
        let arr_class = arr.get_class();
        assert!(Class::get_int_64() == arr_class.get_element_class());
    }
    #[test]
    fn class_get_rank(){
        use wrapped_mono::*;
        let domain = jit::init("main",None);
        let arr:Array<i64> = Array::new(&domain,32);
        let arr_class = arr.get_class();
        let rank = arr_class.get_rank();
        assert!(rank == 1);
    }
    #[test]
    fn class_get_rank_not_array(){
        use wrapped_mono::*;
        let _domain = jit::init("main",None);
        let rank = Class::get_int_64().get_rank();
        assert!(rank == 0);
    }
} 