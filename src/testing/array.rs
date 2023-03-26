use crate as wrapped_mono;
use rusty_fork::rusty_fork_test;
use wrapped_mono::{
    array::Array,
    class::Class,
    jit,
    method::{Method, MethodTrait},
    object::{Object, ObjectTrait},
    *
};
rusty_fork_test! {
    #[test]
    fn get_2D_array_from_method(){
        use crate::binds::MonoObject;
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let mthd:Method<()> = Method::get_from_name(&class,"Get2DIntArray",0).expect("Could not load function");
        let arr:Array<Dim2D,i32> = unsafe{Array::from_ptr((
            mthd.invoke(None,()).expect("Exception").expect("got null").get_ptr() as *mut crate::binds::MonoArray
        ))}.expect("got null again");
        assert!(arr.len() == 8*16);
    }
    #[test]
    fn create_4D_array(){
        let dom = jit::init("root",None);
        let arr:Array<Dim4D,i32> = Array::new(&dom,&[1,2,3,4]);
        assert!(arr.get_lenghts() == [1,2,3,4]);
        assert!(arr.len() == 1*2*3*4);
        assert!(arr.get_class().get_rank() == 4);
    }
    #[test]
    fn create_1D_array(){
        let dom = jit::init("root",None);
        let arr:Array<Dim1D,i32> = Array::new(&dom,&[89]);
        assert!(arr.get_lenghts() == [89]);
        assert!(arr.len() == 89);
        assert!(arr.get_class().get_rank() == 1);
    }
    #[test]
    fn acces_1D_array(){
        let dom = jit::init("root",None);
        let mut arr:Array<Dim1D,i32> = Array::new(&dom,&[89]);
        assert!(arr.get_lenghts() == [89]);
        assert!(arr.len() == 89);
        assert!(arr.get_class().get_rank() == 1);
        for i in 0..89{
            arr.set([i],i as i32);
        }
        for i in 0..89{
            assert!(arr.get([i]) == i as i32);
        }
    }
    #[test]
    fn acces_2D_array(){
        let dom = jit::init("root",None);
        let mut arr:Array<Dim2D,usize> = Array::new(&dom,&[89,13]);
        assert!(arr.get_lenghts() == [89,13]);
        assert!(arr.len() == 89 * 13);
        assert!(arr.get_class().get_rank() == 2);
        for i in 0..89{
            for j in 0..13{
                arr.set([i,j],i^j);
            }
        }
        for i in 0..89{
            for j in 0..13{
                let a = arr.get([i,j]);
                let b =  i^j;
                assert!(a == b,"{} != {}",a,b);
            }
        }
    }
    #[cfg(not(feature = "unsafe_arrays"))]
    #[test]
    #[should_panic]
    fn outsiede_bound_acces_2D_array(){
        let dom = jit::init("root",None);
        let mut arr:Array<Dim2D,usize> = Array::new(&dom,&[89,13]);
        assert!(arr.get_lenghts() == [89,13]);
        assert!(arr.len() == 89 * 13);
        assert!(arr.get_class().get_rank() == 2);
        for i in 0..89{
            for j in 0..14{
                arr.set([i,j],i^j);
            }
        }
        for i in 0..89{
            for j in 0..13{
                let a = arr.get([i,j]);
                let b =  i^j;
                assert!(a == b,"{} != {}",a,b);
            }
        }
    }
}
