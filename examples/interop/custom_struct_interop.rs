use wrapped_mono::*;
/*
C# code in AsmWithVec3
namespace Vec3Namespace{
    struct Vec3{
        float x;
        float y;
        float z;
    }
}
*/
//this types layout does not differ on managed and unmanged side.
#[derive(InteropSend,InteropReceive)]
struct Vec3{
    x:f32,
    y:f32,
    z:f32,
} 
use lazy_static::*;
lazy_static!{
    static ref vec3_class:Class = {
        let img = Assembly::assembly_loaded("AsmWithVec3").expect("Could not find assembly").get_image();
        let vec3_class = Class::from_name(&img,"Vec3Namespace","Vec3").expect("Could not find vec3!");
        vec3_class
    }
}
impl InteropClass for Vec3{
    fn get_mono_class()->Class{
        return vec3_class;
    }
}
//Because Vec3 is a struct on the managed side, it can be unboxed, so it is marked as unboxable by implementing InteropBox
impl InteropBox for Vec3{}
//Vec3 can now be send between managed and unmaanged code, have arrays created, and be boxed/unboxed.
//examples:
#[invokable]
fn do_vec3_magic(Vec3:input)->Vec3{
    println!("Doing magic with vec3!");
    //Some magic operaion is made on input.
    return input;
}
//Then it can be exposed to managed code
fn vec3_expose_magic(){
    add_internal_call!("Vec3Namespace.Vec3::DoMagic",do_vec3_magic);
}
//arrays can be made from it
fn make_vec3_array()->Array<Vec3>{
    return Array::new(doamin.get_curr(),64);
}
//and it can be boxed/unboxed
fn box_n_unbox_vec3(input:Vec3)->Vec3{
    let boxed = Object::box_val::<Vec3>(input);
    let unbox = boxed.unbox();
    return unboxed;
}
// #############################
struct SomeObjectClass{
    obj:Object,
}

// Receiving `SomeObjectClass` as a non-nullable!
impl InteropReceive for SomeObjectClass{
    type SourceType = *mut MonoObject;
    fn get_rust_rep(src:Self::SourceType)->Self{
        return unsafe{Object::from_ptr(src)}.expect("Got null on a non nullable type!");
    }
}
// Receiving `SomeObjectClass` as a nullable!
impl InteropReceive for Option<SomeObjectClass>{
    type SourceType = Option<Object>;
    fn get_rust_rep(src:Self::SourceType)->Self{
        return src;
    }
}
// Sending `SomeObjectClass` as a non-nullable!
impl InteropSend for SomeObjectClass{
    type TargetType = *mut MonoObject;
    fn get_mono_rep(src:Self)->Self::TargetType{
        return src.get_ptr();
    }
}
use core::ptr::null_mut;
// Sending `SomeObjectClass` as a nullable!
impl InteropSend for Option<SomeObjectClass>{
    type TargetType = *mut MonoObject;
    fn get_mono_rep(src:Self)->Self::TargetType{
        match src{
            Some(src)=>return src.get_ptr(),
            None=>return null_mut(),
        }
    }
}
