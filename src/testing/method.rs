use rusty_fork::rusty_fork_test;
use crate as wrapped_mono;
use wrapped_mono::wrapped_mono_macros::*;
use wrapped_mono::{jit,class::Class,method::{Method,MethodTrait}};
rusty_fork_test! {
    #[test]
    fn getting_method(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let _met:Method<i32> = Method::get_method_from_name(&class,"GetArg",1).unwrap();
    }
    #[test]
    fn getting_method_no_gargs(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let _met:Method<()> = Method::get_method_from_name(&class,"GetSomeFiled",0).unwrap();
    }
    #[test]
    fn getting_method_return_no_return(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<()> = Method::get_method_from_name(&class,"SomeInterfaceFunction",0).unwrap();
        assert!(met.get_return() == Class::get_void());
    }
    #[test]
    fn getting_method_return(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<i32> = Method::get_method_from_name(&class,"GetArg",1).unwrap();
        assert!(met.get_return() == Class::get_int_32());
    }
    #[test]
    #[should_panic]
    fn getting_method_wrong_garg_count(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let _met:Method<(i32,i64)> = Method::get_method_from_name(&class,"GetArg",2).unwrap();
    }
    #[should_panic]
    #[test]
    fn getting_null_from_a_function(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<()> = Method::get_method_from_name(&class,"GetObject",0).unwrap();
        let obj = met.invoke(None,()).expect("Got exception").expect("Got null as expected!");
        let _res = obj.unbox::<i32>();
    }
    #[test]
    fn calling_method(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<i32> = Method::get_method_from_name(&class,"GetArg",1).unwrap();
        let obj = met.invoke(None,7).expect("Exception").expect("Got null on a non-nullable!");
        let res = obj.unbox::<i32>();
        assert!(res == 7);
    }
    #[test]
    fn calling_method_2_args(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<(i64,i64)> = Method::get_method_from_name(&class,"Mul",2).unwrap();
        let obj = met.invoke(None,(1,2)).expect("Exception").expect("Got null on a non-nullable!");
        let res = obj.unbox::<i64>();
        assert!(res == 1*2);
    }
    #[test]
    fn getting_method_2_args(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let _met:Method<(i64,i64)> = Method::get_method_from_name(&class,"Mul",2).unwrap();
    }
    #[should_panic]
    #[test]
    fn getting_method_2_wrong_args(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let _met:Method<(u32,i32)> = Method::get_method_from_name(&class,"Mul",2).unwrap();
    }
    #[test]
    fn calling_method_3_args(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<(i64,i64,i64)> = Method::get_method_from_name(&class,"Mul",3).unwrap();
        let obj = met.invoke(None,(1,2,3)).expect("Exception").expect("Got null on a non-nullable!");
        let res = obj.unbox::<i64>();
        assert!(res == 1*2*3);
    }
    #[test]
    fn calling_method_4_args(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<(i64,i64,i64,i64)> = Method::get_method_from_name(&class,"Mul",4).unwrap();
        let obj = met.invoke(None,(1,2,3,4)).expect("Exception").expect("Got null on a non-nullable!");
        let res = obj.unbox::<i64>();
        assert!(res == 1*2*3*4);
    }
    #[test]
    fn calling_method_5_args(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<(i64,i64,i64,i64,i64)> = Method::get_method_from_name(&class,"Mul",5).unwrap();
        let obj = met.invoke(None,(1,2,3,4,5)).expect("Exception").expect("Got null on a non-nullable!");
        let res = obj.unbox::<i64>();
        assert!(res == 1*2*3*4*5);
    }
    #[test]
    fn calling_method_6_args(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<(i64,i64,i64,i64,i64,i64)> = Method::get_method_from_name(&class,"Mul",6).unwrap();
        let obj = met.invoke(None,(1,2,3,4,5,6)).expect("Exception").expect("Got null on a non-nullable!");
        let res = obj.unbox::<i64>();
        assert!(res == 1*2*3*4*5*6);
    }
    #[test]
    fn calling_method_7_args(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<(i64,i64,i64,i64,i64,i64,i64)> = Method::get_method_from_name(&class,"Mul",7).unwrap();
        let obj = met.invoke(None,(1,2,3,4,5,6,7)).expect("Exception").expect("Got null on a non-nullable!");
        let res = obj.unbox::<i64>();
        assert!(res == 1*2*3*4*5*6*7);
    }
    #[test]
    fn calling_method_8_args(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<(i64,i64,i64,i64,i64,i64,i64,i64)> = Method::get_method_from_name(&class,"Mul",8).unwrap();
        let obj = met.invoke(None,(1,2,3,4,5,6,7,8)).expect("Exception").expect("Got null on a non-nullable!");
        let res = obj.unbox::<i64>();
        assert!(res == 1*2*3*4*5*6*7*8);
    }
    #[test]
    fn calling_method_9_args(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<(i64,i64,i64,i64,i64,i64,i64,i64,i64)> = Method::get_method_from_name(&class,"Mul",9).unwrap();
        let obj = met.invoke(None,(1,2,3,4,5,6,7,8,9)).expect("Exception").expect("Got null on a non-nullable!");
        let res = obj.unbox::<i64>();
        assert!(res == 1*2*3*4*5*6*7*8*9);
    }
    #[test]
    fn calling_method_10_args(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<(i64,i64,i64,i64,i64,i64,i64,i64,i64,i64)> = Method::get_method_from_name(&class,"Mul",10).unwrap();
        let obj = met.invoke(None,(1,2,3,4,5,6,7,8,9,10)).expect("Exception").expect("Got null on a non-nullable!");
        let res = obj.unbox::<i64>();
        assert!(res == 1*2*3*4*5*6*7*8*9*10);
    }
    #[test]
    fn calling_method_11_args(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<(i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64)> = Method::get_method_from_name(&class,"Mul",11).unwrap();
        let obj = met.invoke(None,(1,2,3,4,5,6,7,8,9,10,11)).expect("Exception").expect("Got null on a non-nullable!");
        let res = obj.unbox::<i64>();
        assert!(res == 1*2*3*4*5*6*7*8*9*10*11);
    }
    #[test]
    fn calling_method_12_args(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<(i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64)> = Method::get_method_from_name(&class,"Mul",12).unwrap();
        let obj = met.invoke(None,(1,2,3,4,5,6,7,8,9,10,11,12)).expect("Exception").expect("Got null on a non-nullable!");
        let res = obj.unbox::<i64>();
        assert!(res == 1*2*3*4*5*6*7*8*9*10*11*12);
    }
    #[test]
    fn calling_method_13_args(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<(i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64)> = Method::get_method_from_name(&class,"Mul",13).unwrap();
        let obj = met.invoke(None,(1,2,3,4,5,6,7,8,9,10,11,12,13)).expect("Exception").expect("Got null on a non-nullable!");
        let res = obj.unbox::<i64>();
        assert!(res == 1*2*3*4*5*6*7*8*9*10*11*12*13);
    }
    #[test]
    fn calling_method_14_args(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<(i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64)> = Method::get_method_from_name(&class,"Mul",14).unwrap();
        let obj = met.invoke(None,(1,2,3,4,5,6,7,8,9,10,11,12,13,14)).expect("Exception").expect("Got null on a non-nullable!");
        let res = obj.unbox::<i64>();
        assert!(res == 1*2*3*4*5*6*7*8*9*10*11*12*13*14);
    }
    #[test]
    fn calling_method_15_args(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<(i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64)> = Method::get_method_from_name(&class,"Mul",15).unwrap();
        let obj = met.invoke(None,(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15)).expect("Exception").expect("Got null on a non-nullable!");
        let res = obj.unbox::<i64>();
        assert!(res == 1*2*3*4*5*6*7*8*9*10*11*12*13*14*15);
    }
    #[test]
    fn calling_method_16_args(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<(i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64)> = Method::get_method_from_name(&class,"Mul",16).unwrap();
        let obj = met.invoke(None,(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16)).expect("Exception").expect("Got null on a non-nullable!");
        let res = obj.unbox::<i64>();
        assert!(res == 1*2*3*4*5*6*7*8*9*10*11*12*13*14*15*16);
    }
    #[test]
    fn getting_method_arg_count(){
        use wrapped_mono::*;
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<i32> = Method::get_method_from_name(&class,"GetArg",1).unwrap();
        println!("method params:");
        assert!(met.get_param_count() == 1);
    }
    #[test]
    fn getting_method_arg_names(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<i32> = Method::get_method_from_name(&class,"GetArg",1).unwrap();
        println!("method params:");
        assert!(met.get_param_count() == 1);
        for param in met.get_param_names(){
            println!("|{}|",param);
        }
    }
    #[should_panic]
    #[test]
    fn getting_missing_method(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let _met:Method<i32> = Method::get_method_from_name(&class,"Missing",1).unwrap();
    }
    #[should_panic]
    #[test]
    fn gettig_missing_method_wrong_arg_count(){
        use wrapped_mono::{jit,class::Class,method::Method};
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let _met:Method<i32> = Method::get_method_from_name(&class,"GetArg",3).unwrap();
    }
    #[test]
    fn passing_enum_method(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<CLikeEnum> = Method::get_method_from_name(&class,"GetEnum",1).unwrap();
        let arg1:CLikeEnum = CLikeEnum::Val;
        let obj = met.invoke(None,arg1).expect("Exception").expect("Got null on a non-nullable!");
        let res = obj.unbox::<CLikeEnum>();
        assert!(res == arg1);
    }
    /*
    #[test]
    fn calling_str_test_method(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<(String,String,String,String)> = Method::get_method_from_name(&class,"StrTest",4).expect("Can't find method StrTest");
        let obj = met.invoke(None,("one".to_owned(),"two".to_owned(),"three".to_owned(),"four".to_owned())).expect("Exception").expect("Got null on a non-nullable!");
        let res = obj.unbox::<i32>();
        assert!(res == 14);
    }
    */
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
impl InteropClass for CLikeEnum{
    fn get_mono_class()-> Class{
        return Class::from_name(&Assembly::assembly_loaded("Test").expect("Could not find assembly").get_image(),"","CLikeEnum").expect("Could not get class!");
    }
}