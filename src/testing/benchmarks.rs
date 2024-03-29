/*
extern crate test;
use crate as wrapped_mono;
use lazy_static::*;
use test::Bencher;
use wrapped_mono::*;
lazy_static! {
    static ref DOM: Domain = jit::init("dom", None);
}
#[bench]
fn create_object(b: &mut Bencher) {
    let dom = &DOM;
    //enusure that used thread is attached to main domain
    dom.attach_thread();
    //ensure that nursery is empty for accurate measures!
    gc::collect(gc::max_generation());
    let class = Class::get_object();
    b.iter(|| {
        let obj = Object::new(dom, &class);
    });
}
#[bench]
fn clone_object(b: &mut Bencher) {
    let dom = &DOM;
    //enusure that used thread is attached to main domain
    dom.attach_thread();
    //ensure that nursery is empty for accurate measures!
    gc::collect(gc::max_generation());
    let obj = Object::box_val::<i32>(dom, 34);
    b.iter(|| {
        let obj2 = obj.clone();
    });
}
#[bench]
fn unbox_object(b: &mut Bencher) {
    let dom = &DOM;
    //enusure that used thread is attached to main domain
    dom.attach_thread();
    //ensure that nursery is empty for accurate measures!
    gc::collect(gc::max_generation());
    let obj = Object::box_val::<i32>(dom, 34);
    b.iter(|| {
        let obj2 = obj.unbox::<i32>();
    });
}
#[bench]
fn create_mstring(b: &mut Bencher) {
    let dom = &DOM;
    //enusure that used thread is attached to main domain
    dom.attach_thread();
    //ensure that nursery is empty for accurate measures!
    gc::collect(gc::max_generation());
    b.iter(|| {
        let mstr = MString::new(dom, "A");
    });
}
#[bench]
fn create_array(b: &mut Bencher) {
    let dom = &DOM;
    //enusure that used thread is attached to main domain
    dom.attach_thread();
    //ensure that nursery is empty for accurate measures!
    gc::collect(gc::max_generation());
    b.iter(|| {
        let arr: Array<1, i32> = Array::new(dom, &[2]);
    });
}
#[bench]
fn create_exception(b: &mut Bencher) {
    let dom = &DOM;
    //enusure that used thread is attached to main domain
    dom.attach_thread();
    //ensure that nursery is empty for accurate measures!
    gc::collect(gc::max_generation());
    b.iter(|| {
        let exc = Exception::arithmetic();
    });
}
#[bench]
fn clone_mstring(b: &mut Bencher) {
    let dom = &DOM;
    //enusure that used thread is attached to main domain
    dom.attach_thread();
    //ensure that nursery is empty for accurate measures!
    gc::collect(gc::max_generation());
    let mstr = MString::new(dom, "A");
    b.iter(|| {
        let mstr = mstr.clone();
    });
}
#[bench]
fn get_class(b: &mut Bencher) {
    let dom = &DOM;
    //enusure that used thread is attached to main domain
    dom.attach_thread();
    //ensure that nursery is empty for accurate measures!
    gc::collect(gc::max_generation());
    let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
    let img = asm.get_image();
    b.iter(|| {
        let class = Class::from_name(&img, "", "TestFunctions").unwrap();
    });
}
#[bench]
fn get_method(b: &mut Bencher) {
    let dom = &DOM;
    //enusure that used thread is attached to main domain
    dom.attach_thread();
    //ensure that nursery is empty for accurate measures!
    gc::collect(gc::max_generation());
    let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
    let img = asm.get_image();
    let class = Class::from_name(&img, "", "TestFunctions").unwrap();
    b.iter(|| {
        let met: Method<i32> = Method::get_from_name(&class, "GetArg", 1).unwrap();
    });
}
#[bench]
fn call_method(b: &mut Bencher) {
    let dom = &DOM;
    //enusure that used thread is attached to main domain
    dom.attach_thread();
    //ensure that nursery is empty for accurate measures!
    gc::collect(gc::max_generation());
    let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
    let img = asm.get_image();
    let class = Class::from_name(&img, "", "TestFunctions").unwrap();
    let met: Method<i32> = Method::get_from_name(&class, "GetArg", 1).unwrap();
    b.iter(|| {
        met.invoke(None, 8).unwrap().unwrap();
    });
}
#[bench]
fn preform_gc(b: &mut Bencher) {
    let dom = &DOM;
    //enusure that used thread is attached to main domain
    dom.attach_thread();
    //objects necessary to preform gc on.
    let mut objs = Vec::with_capacity(0xFFFFF);
    //Guarantee constant test conditions
    gc::collect(gc::max_generation());
    let class = Class::get_object();
    for i in 0..0xFFFF {
        objs.push(Object::new(dom, &class));
    }
    b.iter(|| {
        gc::collect(gc::max_generation());
    });
}*/
