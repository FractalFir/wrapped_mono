use rusty_fork::rusty_fork_test;
use macros::*;
use crate as wrapped_mono;
mod some_mod{
    use macros::*;
    #[invokable]
    pub fn some_fn(){}
}
rusty_fork_test! {
    #[test]
    fn internal_call(){
        #[invokable]
        pub fn string_test(s:String) -> i32{
            assert!(s == "|one,two,three,four,");
            return 5;
        }
        #[invokable]
        fn pass_arg_count(input:i32){
            assert!(input == 4);
        }
        #[invokable]
        fn pass_test_char(input:char){
            assert!(input == 'ó');
        }
        use wrapped_mono::array::*;
        use wrapped_mono::object::ObjectTrait;
        use wrapped_mono::object::Object;
        #[invokable]
        fn pass_data_array(input:Array<i32,1>){
            let len = input.len();
            let size = input.get_size();
            println!("size:{}",size);
            assert!(size == 56);
            assert!(len == 6);
            for i in 0..len{
                println!("i:{}",i);
                assert!(input.get(i) == i as i32);
            }
        }
        ///invokable macro does not work with <>
        #[invokable]
        fn get_object()->Option<Object>{
            return None;
        }

        use wrapped_mono::*;
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Pinvoke.dll").unwrap();
        let mut args:Vec<&str> = Vec::new();

        args.push("one");
        args.push("two");
        args.push("three");
        args.push("four");
        add_internal_call!("Test::SendTestString",string_test);
        add_internal_call!("Test::PassArgCount", pass_arg_count);
        add_internal_call!("Test::PassDataArray",pass_data_array);
        add_internal_call!("Test::GetObject",get_object);
        add_internal_call!("Test::PassTestChar",pass_test_char);
        add_internal_call!("Test::SomeFN",some_mod::some_fn);

        let _res = jit::exec(&dom,&asm,args);
    } 
}