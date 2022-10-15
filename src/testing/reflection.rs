use crate as wrapped_mono;
use rusty_fork::rusty_fork_test;
use wrapped_mono::*;
rusty_fork_test! {
    #[test]
    fn reflection_type_from_name(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let rftype = ReflectionType::from_name("TestFunctions",&img).expect("Could not get reflection type");
    }
    #[test]
    fn reflection_type_from_class(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let rftype = ReflectionType::from_class(&class);
    }
    #[test]
    fn get_tuple_class(){
        let dom = jit::init("root",None);
        let asm = Assembly::assembly_loaded("mscorlib").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"System","Tuple`2").expect("Could not get class");
        let rftype = ReflectionType::from_class(&class);
    }
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
    }
}
