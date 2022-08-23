use rusty_fork::rusty_fork_test;
rusty_fork_test! { 
    #[test]
    fn object_creation(){
        use crate as wrapped_mono;
        use wrapped_mono::jit;
        use wrapped_mono::object::*;
        use wrapped_mono::class::Class;
        let main = jit::init("main",None);
        let asm = main.assembly_open("test/dlls/Pinvoke.dll").unwrap();
        let img = asm.get_image();
        let test_class = Class::from_name(&img,"","Secondary").expect("Could not find class!");

        let obj = Object::new(&main,&test_class);
        let _hsh = obj.hash();
    }
    #[test]
    fn object_box(){
        use crate as wrapped_mono;
        use wrapped_mono::jit;
        use wrapped_mono::object::*;
        let main = jit::init("main",None);

        let _obj = Object::box_val(&main,128);
    }
    #[test]
    fn object_unbox(){
        use crate as wrapped_mono;
        use wrapped_mono::jit;
        use wrapped_mono::object::*;
        let main = jit::init("main",None);

        let val:i32 = 128; 
        let obj = Object::box_val(&main,128);
        
        let unboxed = Object::unbox::<i32>(&obj);

        assert!(unboxed == val);
    }
    #[cfg(not(feature = "unsafe_unboxing"))]
    #[should_panic]
    #[test]
    fn object_unbox_wrong_type(){
        use crate as wrapped_mono;
        use wrapped_mono::jit;
        use wrapped_mono::object::*;
        let main = jit::init("main",None);

        let val:i32 = 128; 
        let obj = Object::box_val(&main,val);
        let _unboxed = Object::unbox::<i64>(&obj);
    }
}
