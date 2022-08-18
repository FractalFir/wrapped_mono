use rusty_fork::*;
use crate as wrapped_mono;
rusty_fork_test!{
    #[test]
    fn create_not_implemented_exception(){
        use wrapped_mono::*;
        let domain = jit::init("main",None);
        let execepion = Exception::not_implemented("exception!");
    }
    #[test]
    fn create_not_argument_exception(){
        use wrapped_mono::*;
        let domain = jit::init("main",None);
        let execepion = Exception::argument_exception("arg1","exception!");
    }
}
