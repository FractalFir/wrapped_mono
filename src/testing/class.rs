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
    fn get_parrent(){
        use wrapped_mono::*;
        let domain = jit::init("main",None);
        let asm = domain.assembly_open("test/dlls/Test.dll").expect("Could not load assembly");
        let img = asm.get_image();
        let test_class = Class::from_name(&img,"","TestFunctions").expect("Could not find class");
        test_class.get_parent();
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
        let arr:Array<1,i64> = Array::new(&domain,&[32]);
        let arr_class = arr.get_class();
        assert!(Class::get_int_64() == arr_class.get_element_class());
    }
    #[test]
    fn class_get_rank(){
        use wrapped_mono::*;
        let domain = jit::init("main",None);
        let arr:Array<1,i64> = Array::new(&domain,&[32]);
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
    #[test]
    fn class_get_field(){
        use crate::binds::MonoObject;
        use wrapped_mono::{jit,class::Class,object::{Object,ObjectTrait}};
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let filed = Class::get_field_from_name(&class,"someField").expect("Could not get filed!");
    }
    #[test]
    fn ctors_get(){
        use crate::binds::MonoObject;
        use wrapped_mono::{jit,class::Class,object::{Object,ObjectTrait}};
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        unimplemented!("TODO:fix Class::get_ctors_recursive");
        //let ctors = class.get_ctros_recursive();
        //println!("{}",ctors.len());
        //assert!(ctors.len() == 2);//One of 'Object', one of 'TestFunctions'
    }
    #[test]
    fn ctors_recursive_get(){
        use crate::binds::MonoObject;
        use wrapped_mono::{jit,class::Class,object::{Object,ObjectTrait}};
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","CtorTestClass").expect("Could not get class");
        unimplemented!("TODO:fix Class::get_ctors_recursive");
        /*
        let ctors = class.get_ctros_recursive();
        println!("Found {} constructors!",ctors.len());
        for ctor in &ctors{
            println!("{}",ctor);
        }
        assert!(ctors.len() == 5);
        */
        //panic!();
    }
    #[test]
    fn get_delegate(){
        use wrapped_mono::{jit,class::Class,object::{Object,ObjectTrait}};
        let dom = jit::init("root",None);
        let del = Class::get_delegate_class();
    }
    #[test]
    fn construct_generic_class(){
        use crate::interop::InteropClass;
        use wrapped_mono::{jit,class::Class,object::{Object,ObjectTrait}};
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let t = Class::construct_generic_class("System","Tuple",&[i8::get_mono_class(),crate::Exception::get_mono_class()]);
        panic!("{}",t.unwrap().get_name_sig());
    }
    #[test]
    fn get_generic_class_string(){
        use crate::{Method,Class,MethodTrait};
        let dom = crate::jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<()> = Method::get_method_from_name(&class,"CreateTypeString",0).unwrap();
        let res = met.invoke(None,()).expect("Got an exception").unwrap();
    }
         
} 
