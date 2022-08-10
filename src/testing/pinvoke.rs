use rusty_fork::rusty_fork_test;
rusty_fork_test! {
    #[test]
    fn p_invoke(){
        #[invokable]
        fn string_test(s:String){
            println!("{}",s);
            assert!(s == "Hello From Mono!");
        }
        #[invokable]
        fn pass_arg_count(input:i32){
            let a:u64 = 0xFAFAFAFAFAFAFAFA;
            println!("Hello from arg_count! input:{}",input);
        }
        use crate::*;
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/local/Pinvoke.dll").unwrap();
        let mut args:Vec<&str> = Vec::new();

        args.push("1");
        args.push("2");

        add_internal_call!("Test::SendTestString",string_test);
        add_internal_call!("Test::PassArgCount", pass_arg_count);
        
        let res = jit::exec(dom,asm,args);

    } 
}
