use crate as wrapped_mono;
use crate::profiler::Profiler;
use rusty_fork::rusty_fork_test;
use std::sync::Arc;
use wrapped_mono::*;
///Local macro used to simplify tests
macro_rules! profiler_test {
    ($tname:ident) => {
        rusty_fork_test! {
            #[test]
            fn $tname (){
                let mut i:u32 = 0;
                let mut init_lisener = Profiler::create(i);
                fn callback(prof:&mut Profiler<u32>){
                    let ls:&mut u32 = (prof.get_internal_data());
                    (*ls) += 1;
                }
                init_lisener.$tname(callback);
                let dom = jit::init("root",None);
                jit::cleanup(dom);
                assert!(*init_lisener.get_internal_data() == 1);
            }
        }
    };
    ($tname:ident,$rtime_code:block,$tpe:tt) => {
        rusty_fork_test! {
            #[test]
            fn $tname (){
                let mut i:u32 = 0;
                let mut init_lisener = Profiler::create(i);
                fn callback(prof:&mut Profiler<u32>,_:$tpe){
                    let ls:&mut u32 = (prof.get_internal_data());
                    (*ls) += 1;
                }
                init_lisener.$tname(callback);
                let dom = jit::init("root",None);
                $rtime_code
                jit::cleanup(dom);
                assert!(*init_lisener.get_internal_data() == 1);
            }
        }
    };
}
// Some tests do not pass because macro implementing them does not support code injection needed to test them propely.
profiler_test! {add_runtime_initialized_callback}
profiler_test! {add_runtime_shutdown_begin_callback}
profiler_test! {add_runtime_shutdown_end_callback}
profiler_test! {add_context_loaded}
profiler_test! {add_context_unloaded}
profiler_test! {add_domain_loading,{},(&mut Domain)}
profiler_test! {add_domain_loaded,{},(&mut Domain)}
//Do not work.
//profiler_test!{add_domain_unloading,{},(&mut Domain)}
//profiler_test!{add_domain_unloaded,{},(&mut Domain)}
//profiler_test!{add_domain_name,{},(&mut Domain)}

// TODO:re-enable this test on the dev branch when working on 0.3(it is not planed to be in 0.2)
/*
profiler_test!{add_jit_begin,{},
    (&Method<String>)}
rusty_fork_test! {
    #[test]
    fn profiler_arc(){
        let dom = jit::init("root",None);
        let data:Arc<u32> = Arc::new(0);
        let prof = Profiler::create(data);
        prof.destroy();
    }
}
*/
