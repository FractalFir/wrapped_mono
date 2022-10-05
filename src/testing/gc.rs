use crate::*;
use rusty_fork::rusty_fork_test;
rusty_fork_test! { 
    #[test]
    fn test_gc_object(){
        use crate::gc::count_objects;
        let dom = jit::init("dom",None);
        let mut results:Vec<Object> = Vec::with_capacity(4000);
        for i in 0..8000{
            let mut obj = Object::box_val::<i32>(&dom,i);
            for j in 0..100{
                obj = Object::box_val::<i32>(&dom,i); 
                for i in 0..10{
                    let tmp = obj.clone();
                } 
            }
            results.push(obj);
        }
        let prev = gc::count_objects();
        gc::collect(gc::max_generation());
        let next = count_objects();
        assert!(next<prev,"{} >= {}",next,prev);
        for (i,obj) in results.into_iter().enumerate(){
            let val = obj.unbox::<i32>();
            assert!(val == i as i32,"{} != {}",val,i);
        }
    }
    #[test]
    fn test_gc_object_multiref(){
        use crate::gc::count_objects;
        let dom = jit::init("dom",None);
        let obj = Object::box_val::<i32>(&dom,10);
        for i in 0..100{
            let sec_ref = obj.clone();
        }
    }
    #[test]
    fn test_gc_mstring(){
        use crate::gc::count_objects;
        let dom = jit::init("dom",None);
        let mut results:Vec<MString> = Vec::with_capacity(4000);
        for i in 0..8000{
            let mut obj = MString::new(&dom,&format!("{}",i));
            for j in 0..100{
                obj = MString::new(&dom,&format!("{}",i));
                for i in 0..10{
                    //let tmp = obj.clone();
                } 
            }
            results.push(obj);
        }
        let prev = gc::count_objects();
        gc::collect(gc::max_generation());
        let next = count_objects();
        assert!(next<prev,"{} >= {}",next,prev);
        for (i,obj) in results.into_iter().enumerate(){
            let val = obj.to_string();
            let sec = format!("{}",i);
            assert!(val == sec,"{} != {}",val,sec);
        }
    }
}
