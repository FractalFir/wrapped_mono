use rusty_fork::rusty_fork_test;
use crate as wrapped_mono;
rusty_fork_test! {
    #[test]
    fn getting_method(){
        use wrapped_mono::{jit,class::Class,method::Method};
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let _met = Method::get_method_from_name(&class,"GetArg",1).unwrap();
    }
    #[should_panic]
    #[test]
    fn getting_null_from_a_function(){
        use wrapped_mono::*;
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met = Method::get_method_from_name(&class,"GetObject",0).unwrap();
        let obj = method_invoke!(met,None).expect("Got exception").expect("Got null as expected!");
        let _res = obj.unbox::<i32>();
    }
    #[test]
    fn calling_method(){
        use wrapped_mono::{jit,class::Class,method::Method};
        use crate::interop::{get_mono_rep_val,ref_to_cvoid_ptr};
        use macros::*;
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met = Method::get_method_from_name(&class,"GetArg",1).unwrap();
        let mut arg1:i32 = 7;
        let obj = method_invoke!(met,None,arg1).expect("Exception").expect("Got null on a non-nullable!");
        let res = obj.unbox::<i32>();
        assert!(res == arg1);
    }
    #[test]
    fn getting_method_arg_count(){
        use wrapped_mono::*;
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met = Method::get_method_from_name(&class,"GetArg",1).unwrap();
        println!("method params:");
        assert!(met.get_param_count() == 1);
    }
    #[test]
    fn getting_method_arg_names(){
        use wrapped_mono::{jit,class::Class,method::Method};
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met = Method::get_method_from_name(&class,"GetArg",1).unwrap();
        println!("method params:");
        assert!(met.get_param_count() == 1);
        for param in met.get_param_names(){
            println!("|{}|",param);
        }
    }
    #[should_panic]
    #[test]
    fn getting_missing_method(){
        use wrapped_mono::{jit,class::Class,method::Method};
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let _met = Method::get_method_from_name(&class,"Missing",1).unwrap();
    }
    #[should_panic]
    #[test]
    fn gettig_missing_method_wrong_arg_count(){
        use wrapped_mono::{jit,class::Class,method::Method};
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let _met = Method::get_method_from_name(&class,"GetArg",3).unwrap();
    }
    #[test]
    fn passing_enum_method(){
        use wrapped_mono::{jit,class::Class,method::Method};
        use crate::interop::{get_mono_rep_val,ref_to_cvoid_ptr};
        use macros::*;
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met = Method::get_method_from_name(&class,"GetEnum",1).unwrap();
        let mut arg1:CLikeEnum = CLikeEnum::Val;
        let obj = method_invoke!(met,None,arg1).expect("Exception").expect("Got null on a non-nullable!");
        let res = obj.unbox::<CLikeEnum>();
        assert!(res == arg1);
    }
    /*
    #[test]
    fn testing_function_signature(){
        use wrapped_mono::{jit,class::Class,method::Method};
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let sig_check = img.check_fnc_sig("TestFunctions::GetArg",Class::get_int_32(),&vec![Class::get_int_32()]);
        assert!(sig_check);
    }
    */
}
use crate::{InteropRecive,InteropSend,InteropClass};
use crate::InteropBox;
#[derive(InteropRecive,InteropSend,Copy,Clone,PartialEq)]
#[repr(u64)]
enum CLikeEnum{
    Val = 1,
    Val2 = 2,
    Val3 = 612,
}
impl InteropBox for CLikeEnum{}
use crate::assembly::Assembly;
use crate::Class;
impl InteropClass for CLikeEnum{
    fn get_mono_class()-> Class{
        return Class::from_name(&Assembly::assembly_loaded("Test").expect("Could not find assembly").get_image(),"","CLikeEnum").expect("Could not get class!");
    }
}