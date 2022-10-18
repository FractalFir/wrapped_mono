use rusty_fork::rusty_fork_test;
use crate as wrapped_mono;
use wrapped_mono::jit;
use wrapped_mono::object::ObjectTrait;
use wrapped_mono::object::*;
rusty_fork_test!{ 
    #[test]
    fn object_creation(){
        use wrapped_mono::class::Class;
        let main = jit::init("main",None);
        let asm = main.assembly_open("test/dlls/Pinvoke.dll").unwrap();
        let img = asm.get_image();
        let test_class = Class::from_name(&img,"","Secondary").expect("Could not find class!");
        let obj = Object::new(&main,&test_class);
        let _hsh = hash(&obj);
    }
    #[test]
    fn object_box(){
        let main = jit::init("main",None);
        let _obj = Object::box_val(&main,128);
    }
    #[test]
    fn object_unbox(){
        let main = jit::init("main",None);
        let val:i32 = 128; 
        let obj = Object::box_val(&main,128);
        let unboxed = Object::unbox::<i32>(&obj);
        assert!(unboxed == val);
    }
    #[cfg(not(feature = "unsafe_boxing"))]
    #[should_panic]
    #[test]
    fn object_unbox_wrong_type(){   
        let main = jit::init("main",None);
        let val:i32 = 128; 
        let obj = Object::box_val(&main,val);
        let _unboxed = Object::unbox::<i64>(&obj);
    }
    #[test]
    fn test_object_size(){
        use crate::binds::MonoObject;
        use std::mem::size_of;
        use wrapped_mono::{jit,class::Class,object::{Object,ObjectTrait}};
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let obj = Object::new(&dom,&class);
        let size = obj.get_size();
        //println!("{}",size);
        assert!(size as usize == size_of::<MonoObject>()  + size_of::<i32>());
    }
    #[test]
    fn test_object_field_get_value(){
        use crate::binds::MonoObject;
        use wrapped_mono::{jit,class::Class,object::{Object,ObjectTrait}};
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let obj = Object::new(&dom,&class);
        let field = Class::get_field_from_name(&class,"someField").expect("Could not get field!");
        let val = field.get_value_object(&obj).expect("Could not get object field!");
        let unboxed = val.unbox::<i32>();
        //Gets 0 because constructor not called!
        assert!(unboxed == 0);
    }
}
