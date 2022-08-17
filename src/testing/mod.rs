#![cfg(test)]
use rusty_fork::rusty_fork_test;
mod pinvoke;
mod object;
mod method;
mod exception;
use crate as wrapped_mono;
use wrapped_mono::*;
use macros::{invokable,add_internal_call};
use invokable::InvokePass;
rusty_fork_test! {
    #[test]
    fn jit_execution(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/local/Jit.dll").unwrap();
        let mut args:Vec<&str> = Vec::new();
        args.push("1");
        args.push("2");
        let _res = jit::exec(&dom,&asm,args);
    }
    #[test]
    fn jit_init(){
        use wrapped_mono::jit;
        let _dom = jit::init("root",None);
    }
    #[test]
    fn jit_init_version(){
        use wrapped_mono::jit;
        let _dom = jit::init("root",Some("v4.0.30319"));
    }
    #[test]
    fn multiple_domains(){
        use wrapped_mono::jit;
        use crate::domain::Domain;
        let _dom = jit::init("root",None);
        let _dom2 = Domain::create();
    }
    #[test]
    fn assembly_loading(){
        use wrapped_mono::jit;
        let dom = jit::init("root",None);
        dom.assembly_open("test/local/Test.dll").unwrap();
    }
    #[should_panic]
    #[test]
    fn missing_assembly_loading(){
        use wrapped_mono::jit;
        let dom = jit::init("root",None);
        dom.assembly_open("test/local/Missing.dll").unwrap();
    }
    #[test]
    fn stop_jit(){
        use wrapped_mono::jit;
        let dom = jit::init("root",None);
        jit::cleanup(dom);
    }
    #[test]
    fn getting_image_from_assembly(){
        use wrapped_mono::jit;
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/local/Test.dll").unwrap();
        let _img = asm.get_image();
    }
    #[test]
    fn gettig_class_from_image(){
        use wrapped_mono::{jit,class::Class};
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/local/Test.dll").unwrap();
        let img = asm.get_image();
        let _class = Class::from_name(&img,"","TestFunctions");
    }
    #[test]
    fn test_object_size(){
        use crate::binds::MonoObject;
        use wrapped_mono::{jit,class::Class,method::Method,object::{Object,ObjectTrait}};
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/local/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let obj = Object::new(&dom,&class);
        let size = obj.get_size();
        //println!("{}",size);
        assert!(size == std::mem::size_of::<MonoObject>() as u32);
    }
} 

