#![cfg(test)]
use crate as wrapped_mono;
use rusty_fork::rusty_fork_test;
rusty_fork_test! {
    #[test]
    fn test_interface_iteration(){
        use wrapped_mono::*;
        let domain = jit::init("main",None);
        let asm = domain.assembly_open("test/dlls/Test.dll").expect("Could not load assembly");
        let img = asm.get_image();
        let test_class = Class::from_name(&img,"","TestFunctions").expect("Could not find class");
        let ifaces = test_class.get_interfaces();
        println!("{}",&test_class.get_name());
        assert!(!ifaces.is_empty());
        assert!("IInterfaceOne" == &ifaces[0].get_name());
    }
    #[test]
    fn class_get_namespace(){
        use crate as wrapped_mono;
        use wrapped_mono::jit;

        use wrapped_mono::class::Class;
        let _main = jit::init("main",None);
        let class = Class::get_int_64();
        assert!("System" == &class.get_namespace());
    }
    #[test]
    fn fmt(){
        use wrapped_mono::*;
        let domain = jit::init("main",None);
        let asm = domain.assembly_open("test/dlls/Test.dll").expect("Could not load assembly");
        let img = asm.get_image();
        let test_class = Class::from_name(&img,"","TestFunctions").expect("Could not find class");
        let dbg_fmt = format!("{test_class:?}");
        assert_eq!(dbg_fmt,"Class{namespace:\"\",name:\"TestFunctions\"}");
    }
    #[test]
    fn get_parrent(){
        use wrapped_mono::*;
        let domain = jit::init("main",None);
        let asm = domain.assembly_open("test/dlls/Test.dll").expect("Could not load assembly");
        let img = asm.get_image();
        let test_class = Class::from_name(&img,"","TestFunctions").expect("Could not find class");
        let parrent = test_class.get_parent().unwrap();
        assert_eq!(parrent,Class::get_object());
    }
    #[test]
    fn class_get_namespace_no_namespace(){
        use wrapped_mono::*;
        let domain = jit::init("main",None);
        let asm = domain.assembly_open("test/dlls/Test.dll").expect("Could not load assembly");
        let img = asm.get_image();
        let test_class = Class::from_name(&img,"","TestFunctions").expect("Could not find class");
        assert!(test_class.get_namespace().is_empty());
    }
    #[test]
    fn class_get_array_element_class(){
        use wrapped_mono::*;
        let domain = jit::init("main",None);
        let arr:Array<Dim1D,i64> = Array::new(&domain,&[32]);
        let arr_class = arr.get_class();
        assert!(Class::get_int_64() == arr_class.get_element_class());
    }
    #[test]
    fn class_get_rank(){
        use wrapped_mono::*;
        let domain = jit::init("main",None);
        let arr:Array<Dim1D,i64> = Array::new(&domain,&[32]);
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

        use wrapped_mono::{jit,class::Class};
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let _filed = Class::get_field_from_name(&class,"someField").expect("Could not get filed!");
    }
    // TODO:rethink removal of those functionalities, maybe re-add themin 0.3
    /*
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
    }*/
    #[test]
    fn get_delegate(){
        use wrapped_mono::{jit,class::Class};
        let _dom = jit::init("root",None);
        let _del = Class::get_delegate_class();
    }
    #[test]
    fn get_generic_class_string(){
        use crate::{Method,Class};
        let dom = crate::jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<()> = Method::get_from_name(&class,"CreateTypeString",0).unwrap();
        let _res = met.invoke(None,()).expect("Got an exception").unwrap();
    }

}
