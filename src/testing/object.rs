use rusty_fork::rusty_fork_test;
rusty_fork_test! { 
    #[test]
    fn object_creation(){
        use crate as wrapped_mono;
        use wrapped_mono::jit;
        use wrapped_mono::object::*;
        use wrapped_mono::class::Class;
        let main = jit::init("main",None);
        let asm = main.assembly_open("test/local/Pinvoke.dll").unwrap();
        let img = asm.get_image();
        let test_class = Class::from_name(&img,"","Secondary").expect("Could not find class!");

        let obj = Object::new(&main,&test_class);
        let _hsh = obj.hash();
    }
}
