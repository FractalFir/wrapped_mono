use crate as wrapped_mono;
use rusty_fork::rusty_fork_test;
use wrapped_mono::*;
rusty_fork_test! {
    #[test]
    fn reflection_type_from_name(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let _rftype = ReflectionType::from_name("TestFunctions",img).expect("Could not get reflection type");
    }
    #[test]
    fn reflection_type_from_class(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let _rftype = ReflectionType::from_class(&class);
    }
    #[test]
    fn get_tuple_class(){
        let _dom = jit::init("root",None);
        let asm = Assembly::assembly_loaded("mscorlib").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"System","Tuple`2").expect("Could not get class");
        let _rftype = ReflectionType::from_class(&class);
    }
    // TODO:re-enable this test on the dev branch when working on 0.3(it is not planed to be in 0.2) and try to fix the bug that prevents it from working: Check if the type we get is realy generic (maybe types such as "System.Tuple`3" are cast to "System.Tuple" behind the scenes?
    /*
    #[test]
    fn create_generic_type(){
        let dom = jit::init("root",None);
        let asm = Assembly::assembly_loaded("mscorlib").unwrap();
        let img = asm.get_image();
        let target_type = ReflectionType::create_generic(
            &img,
            "System.Tuple",
            &[Class::get_int_32().into(),Class::get_byte().into(),Class::get_sbyte().into()]
        ).unwrap();
    }*/
}
