use rusty_fork::*;
use crate as wrapped_mono;
rusty_fork_test!{
    #[test]
    fn create_not_implemented_exception(){
        use wrapped_mono::*;
        let _domain = jit::init("main",None);
        let _execepion = Exception::not_implemented("exception!");
    }
    #[test]
    fn create_not_argument_exception(){
        use wrapped_mono::*;
        let _domain = jit::init("main",None);
        let _execepion = Exception::argument_exception("arg1","exception!");
    }
}
