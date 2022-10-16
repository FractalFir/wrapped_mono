use rusty_fork::rusty_fork_test;
use crate as wrapped_mono;
use wrapped_mono::*;
rusty_fork_test!{
    #[test]
    fn getting_delegate_from_method(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<()> = Method::get_from_name(&class,"GetDelegate",0).unwrap();
        let obj = met.invoke(None,()).expect("Got an Exception").expect("Got null on a non-nullable!");
        assert!(obj.get_class().is_delegate());
    }
    #[test]
    fn calling_delegate_from_method(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met:Method<()> = Method::get_from_name(&class,"GetDelegate",0).unwrap();
        let obj = met.invoke(None,()).expect("Got an Exception").expect("Got null on a non-nullable!");
        let del:Delegate<(i32,i32)> = Delegate::cast_from_object(&obj).expect("Expected delegate, got something else");
        let _res = del.invoke((10,10)).expect("Exception").expect("Got null");
    }
}
