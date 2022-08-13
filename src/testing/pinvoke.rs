use rusty_fork::rusty_fork_test;
use macros::*;
use crate as wrapped_mono;
rusty_fork_test! {
    #[test]
    fn p_invoke(){
        #[invokable]
        fn string_test(s:String){
            assert!(s == "|one,two,three,four,");
        }
        #[invokable]
        fn pass_arg_count(input:i32){
            assert!(input == 4);
        }
        use wrapped_mono::array::*;
        use wrapped_mono::object::ObjectTrait;
        #[invokable]
        fn pass_data_array(input:Array<i32>){
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
        use wrapped_mono::*;
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/local/Pinvoke.dll").unwrap();
        let mut args:Vec<&str> = Vec::new();

        args.push("one");
        args.push("two");
        args.push("three");
        args.push("four");

        add_internal_call!("Test::SendTestString",string_test);
        add_internal_call!("Test::PassArgCount", pass_arg_count);
        add_internal_call!("Test::PassDataArray",pass_data_array);
        
        let _res = jit::exec(dom,asm,args);

    } 
}