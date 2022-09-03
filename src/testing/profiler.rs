use crate as wrapped_mono;
use wrapped_mono::*;
use crate::profiler::Profiler;
use rusty_fork::rusty_fork_test;
use std::sync::Arc;
#[derive(Clone)]
struct TestData{
    pub a:u64,
    pub b:u64,
    pub c:String,
    pub d:Vec<u64>,
}
rusty_fork_test! {
    #[test]
    fn jit_init_cb(){
        let mut i:u32 = 0;
        let mut init_lisener = Profiler::create(i);
        fn profiler_runtime_init_callback(prof:&mut Profiler<u32>){
            let ls:&mut u32 = (prof.get_internal_data());
            (*ls) += 1;
        }
        init_lisener.add_runtime_initialized_callback(profiler_runtime_init_callback);
        let dom = jit::init("root",None);
        assert!(*init_lisener.get_internal_data() == 1);
    }
    #[test]
    fn jit_shutdown_beg(){
        let mut i:u32 = 0;
        let mut init_lisener = Profiler::create(i);
        fn profiler_runtime_shutdown_beg(prof:&mut Profiler<u32>){
            let ls:&mut u32 = (prof.get_internal_data());
            (*ls) += 1;
        }
        init_lisener.add_runtime_shutown_begin_callback(profiler_runtime_shutdown_beg);
        let dom = jit::init("root",None);
        jit::cleanup(dom);
        assert!(*init_lisener.get_internal_data() == 1);
    }
    #[test]
    fn jit_shutdown_end(){
        let mut i:u32 = 0;
        let mut init_lisener = Profiler::create(i);
        fn profiler_runtime_shutdown_end(prof:&mut Profiler<u32>){
            let ls:&mut u32 = (prof.get_internal_data());
            (*ls) += 1;
        }
        init_lisener.add_runtime_shutown_end_callback(profiler_runtime_shutdown_end);
        let dom = jit::init("root",None);
        jit::cleanup(dom);
        assert!(*init_lisener.get_internal_data() == 1);
    }
    #[test]
    fn profiler_test(){
    } 
}