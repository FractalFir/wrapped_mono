use crate::*;
use rusty_fork::rusty_fork_test;
rusty_fork_test! {
    #[test]#[cfg(feature = "referenced_objects")]
    fn test_gc_object(){
        use crate::gc::count_objects;
        let dom = jit::init("dom",None);
        let mut results:Vec<Object> = Vec::with_capacity(4000);
        for i in 0..800{
            let mut obj = Object::box_val::<i32>(&dom,i);
            for _j in 0..1000{
                obj = Object::box_val::<i32>(&dom,i);
                for _i in 0..10{
                    let _tmp = obj.clone();
                }
            }
            results.push(obj);
        }
        let prev = gc::count_objects();
        gc::collect();
        let next = count_objects();
        assert!(next<prev,"{} >= {}",next,prev);
        for (i,obj) in results.into_iter().enumerate(){
            let val = obj.unbox::<i32>();
            assert!(val == i as i32,"{} != {}",val,i);
        }
    }
    #[test]
    fn test_gc_object_multiref(){

        let dom = jit::init("dom",None);
        let obj = Object::box_val::<i32>(&dom,10);
        for _i in 0..100{
            let _sec_ref = obj.clone();
        }
    }
    #[test]#[cfg(feature = "referenced_objects")]
    fn test_gc_mstring(){
        use crate::gc::count_objects;
        let dom = jit::init("dom",None);
        let mut results:Vec<MString> = Vec::with_capacity(4000);
        for i in 0..800{
            let mut obj = MString::new(&dom,&format!("{}",i));
            for _j in 0..1000{
                obj = MString::new(&dom,&format!("{}",i));
                for _i in 0..10{
                    let _tmp = obj.clone();
                }
            }
            results.push(obj);
        }
        let prev = gc::count_objects();
        gc::collect();
        let next = count_objects();
        assert!(next<prev,"{} >= {}",next,prev);
        for (i,obj) in results.into_iter().enumerate(){
            let val = obj.to_string();
            let sec = format!("{}",i);
            assert!(val == sec,"{} != {}",val,sec);
        }
    }
    #[test]#[cfg(feature = "referenced_objects")]
    fn test_gc_array(){
        use crate::gc::count_objects;
        let dom = jit::init("dom",None);
        let mut results:Vec<Array<Dim1D,i32>> = Vec::with_capacity(4000);
        println!("Preparing to create test arrays!");
        // Having more temporary Arrays fills up the nursery, and causes problems with garbage collection(can't unlock a thread)
        for i in 0..17{
            let mut obj:Array<Dim1D,i32> = Array::new(&dom,&[i/50_usize]);
            for _j in 0..1000{
                obj = Array::new(&dom,&[i/50_usize]);
                for _i in 0..10{
                    let _tmp = obj.clone();
                }
            }
            if i % 100 == 0{
                println!("Created an array! {}",i);
                gc::collect();
            }
            results.push(obj);
        }
        println!("Created all test arrays!");
        let prev = gc::count_objects();
        gc::collect();
        let next = count_objects();
        assert!(next<prev,"{} >= {}",next,prev);
        println!("Ran GC!");
        for (i,obj) in results.into_iter().enumerate(){
            assert!(obj.len() == i/50);
        }
    }
    #[test]#[cfg(feature = "referenced_objects")]
    fn test_gc_exception(){
        use crate::gc::count_objects;
        let _dom = jit::init("dom",None);
        let mut results:Vec<Exception> = Vec::with_capacity(17);
        println!("Preparing to create test arrays!");
        // Having more temporary Arrays fills up the nursery, and causes problems with garbage collection(can't unlock a thread)
        for i in 0..17{
            let mut obj:Exception = Exception::arithmetic();
            for _j in 0..10000{
                obj = Exception::arithmetic();
                for _i in 0..10{
                    let _tmp = obj.clone();
                }
            }
            if i % 100 == 0{
                println!("Created an array! {}",i);
                gc::collect();
            }
            results.push(obj);
        }
        println!("Created all test arrays!");
        let prev = gc::count_objects();
        gc::collect();
        let next = count_objects();
        assert!(next<prev,"{} >= {}",next,prev);
        println!("Ran GC!");
        for obj in results{
            obj.get_size();
        }
    }
}
