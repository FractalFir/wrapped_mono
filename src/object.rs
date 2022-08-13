///Safe representation of MonoObject.
pub struct Object{
    obj_ptr:*mut crate::binds::MonoObject,
}
///Trait contining functions common for all types of objects.
pub trait ObjectTrait{
    ///get hash of this object: This hash is **not** the same for diffrent objects with the identical values of their fields.
    fn hash(&self)->i32;
    ///get domain this object exists in.
    fn get_domain(&self)->crate::domain::Domain;
    ///get size of this object in bytes
    fn get_size(&self)->u32;
    ///get reflection token TODO:extend this description to make it more clear
    fn reflection_get_token(&self)->u32;
    ///returns class of this object
    fn get_class(&self)->crate::class::Class;
    ///returns Object *self* cast to *class* if Object *self* is derived from class.
    fn is_inst(&self,class:crate::class::Class)->Option<Object>;
}
impl ObjectTrait for Object{
    fn hash(&self)->i32{
        return unsafe{crate::binds::mono_object_hash(self.obj_ptr)};
    }
    fn get_domain(&self)->crate::domain::Domain{
        return unsafe{crate::domain::Domain::from_ptr(crate::binds::mono_object_get_domain(self.obj_ptr))};
    }
    fn get_size(&self)->u32{
        return unsafe{crate::binds:: mono_object_get_size(self.obj_ptr)};
    }
    fn reflection_get_token(&self)->u32{
        return unsafe{crate::binds::mono_reflection_get_token(self.obj_ptr)};
    }
    fn get_class(&self)->crate::class::Class{
        return unsafe{crate::class::Class::from_ptr(
            crate::binds::mono_object_get_class(self.obj_ptr)
        ).expect("Could not get class of an object")};
    }
    fn is_inst(&self,class:crate::class::Class)->Option<Object>{
        return unsafe{Self::from_ptr(crate::binds::mono_object_isinst(self.get_ptr(),class.get_ptr()))};
    }
}
impl Object{
    ///Allocates new object of Class class. **Does not call the constructor**
    pub fn new(domain:crate::domain::Domain,class:crate::class::Class)->Self{
        return unsafe{Self::from_ptr(crate::binds::mono_object_new(domain.get_ptr(),class.get_ptr()))}.expect("Could not create new type from class!");
    }
    pub unsafe fn from_ptr(obj_ptr:*mut crate::binds::MonoObject)->Option<Self>{
        if obj_ptr == core::ptr::null_mut(){
            return None;
        }
        return Some(Self{obj_ptr:obj_ptr});
    }
    ///Retrives pointer to unboxed value. 
    ///**Example:**<br>
    ///C#<br>
    ///```cs
    ///int num = 123;
    ///Object boxed = num;
    ///RustFunction(boxed);
    ///```
    ///Rust
    ///```rust 
    ///#[invokable]
    ///fn rust_function(o:Object){
    ///    unsafe{let val = *(o.unbox() as *mut i32)};
    ///}
    ///```
    pub unsafe fn unbox(&self)->*mut std::ffi::c_void{
        return unsafe{crate::binds::mono_object_unbox(self.obj_ptr)};
    }
    pub unsafe fn box_val(domain:crate::domain::Domain,class:crate::class::Class,val:*mut std::ffi::c_void)->crate::object::Object{
        return unsafe{crate::object::Object::from_ptr(crate::binds::mono_value_box(domain.get_ptr(),class.get_ptr(),val)).expect("Could not box value")};
    }
    pub unsafe fn get_ptr(&self)->*mut crate::binds::MonoObject{
        return self.obj_ptr;
    }
}
impl crate::invokable::InvokePass for Object{
    type SourceType = *mut  crate::binds::MonoObject;
    fn get_rust_rep(arg:Self::SourceType)->Self{
        return unsafe{Self::from_ptr(arg)}.expect("passed MonoObject argument is invalid");
    }
}
impl Clone for Object{
    fn clone(&self)->Self{
        //if clone fails, it means that there is a much bigger problem with mono runtime, so we jus
        return unsafe{Self::from_ptr(crate::binds::mono_object_clone(self.obj_ptr))}.expect("MonoRuntime could not clone object!");
    }
}