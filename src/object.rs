use crate::class::Class;
use crate::binds::MonoObject;
use crate::method::Method;
use crate::domain::Domain;
use crate::exception::ExceptManaged;
use core::ptr::null_mut;
///Safe representation of a refernece to a manged Object. Is **not nullable** when passed between managed and unmanged code(e.g when added as an argument to function exposed as an interna call). 
///It means that while it may represent a nullable type, wrapped-mono will automaticly panic when recived null value.
///For nullable support use `Option<Object>`.
pub struct Object{
    obj_ptr:*mut crate::binds::MonoObject,
}
use crate::mstring::MString;
///Trait contining functions common for all types of manged objects.
pub trait ObjectTrait{
    ///get hash of this object: This hash is **not** based on values of objects fields, and differs from result of calling object.GetHash()
    /// #Example 
    /// ```rust
    /// let object = Object::new(&domain,&class);
    /// let object_copy = object.clone_managed_object();
    /// assert!(object.hash() != object_copy.hash()); // Objects object and object_copy have exacly 
    /// // the same values of their fileds, but are diffrent instances, so their hash is diffrent.
    /// ```
    fn hash(&self)->i32;
    ///get [`Domain`] this object exists in.
    /// # Example
    ///```rust
    /// let domain = Domain.create(); //create Domain dom
    /// let object = Object::new(&dom,&class); //create object in Domain dom.
    /// let obj_domain = object.get_domain(); //get doamin object is in
    /// assert!(domain == obj_domain); 
    ///```
    fn get_domain(&self)->crate::domain::Domain;
    ///get size of managed object referenced by *self* in bytes. Does include builtin hidden data.
    /// # Example
    ///```csharp
    /// class SomeClass{};
    /// class OtherClass{int some_int;};
    ///```
    ///```rust
    /// let size = some_obj.get_size();  //Get size of some_obj(in this case an insatce of SomeClass)
    /// assert!(size == std::mem::size_of::<MonoObject>()); // 8 bytes on 32 bit systems, 16 on 64 bit ones (size of two pointers).
    /// let size_other = other_obj.get_size(); //Get size of other_obj(in this case an insatce of OtherClass)
    /// assert!(size_other == std::mem::size_of::<MonoObject>() + 8); //size of two hidden pointers + some_int filed.
    ///```
    fn get_size(&self)->u32;
    ///get reflection token 
    //TODO:extend this description to make it more clear
    fn reflection_get_token(&self)->u32;
    ///returns [`Class`] of this object.
    /// # Example
    /// ```rust 
    /// let object = Object::new(&domain,&class);
    /// let object_class = object.get_class();
    /// assert!(class == object_class);
    /// ```
    fn get_class(&self)->Class;
    ///returns [`Object`] *self* cast to *class* if *self* is derived from [`Class`]class. Does not affect original reference to object nor the object itself.
    fn is_inst(&self,class:&Class)->Option<Object>;
    ///Convert [`Object`] to [`MString`]. Returns [`Exception`] if raised, and [`Option<MString>`] if not. Function returns [`Option<MString>`] to allow for null value to be returned. 
    fn to_string(&self)->Result<Option<MString>,Exception>;
}
use crate::exception::Exception;
impl ObjectTrait for Object{
    fn hash(&self)->i32{
        return unsafe{crate::binds::mono_object_hash(self.obj_ptr)};
    }
    fn get_domain(&self)->Domain{
        return unsafe{Domain::from_ptr(crate::binds::mono_object_get_domain(self.obj_ptr))};
    }
    fn get_size(&self)->u32{
        return unsafe{crate::binds:: mono_object_get_size(self.obj_ptr)};
    }
    fn reflection_get_token(&self)->u32{
        return unsafe{crate::binds::mono_reflection_get_token(self.obj_ptr)};
    }
    fn get_class(&self)->Class{
        return unsafe{Class::from_ptr(
            crate::binds::mono_object_get_class(self.obj_ptr)
        ).expect("Could not get class of an object")};
    }
    fn is_inst(&self,class:&Class)->Option<Object>{
        return unsafe{Self::from_ptr(crate::binds::mono_object_isinst(self.get_ptr(),class.get_ptr()))};
    }
    fn to_string(&self)->Result<Option<MString>,Exception>{
        let mut exc:*mut crate::binds::MonoException = core::ptr::null_mut();
        let res = unsafe{MString::from_ptr(
            crate::binds::mono_object_to_string(self.obj_ptr,&mut exc as *mut *mut crate::binds::MonoException as *mut *mut crate::binds::MonoObject)
        )};
        let exc = unsafe{Exception::from_ptr(exc)};
        match exc{
            Some(e)=>return Err(e),
            None=>return Ok(res),
        }
    }
}
use crate::interop::InteropBox;
impl Object{ 
    ///Allocates new object of [`Class`] class. **Does not call the constructor**
    /// # Examples
    /// ```rust
    /// let new_obj = Object::new(some_domain,new_objects_class);
    /// ```
    pub fn new(domain:&crate::domain::Domain,class:&Class)->Self{
        return unsafe{Self::from_ptr(crate::binds::mono_object_new(domain.get_ptr(),class.get_ptr()))}.expect("Could not create new type from class!");
    }
    ///Creates new [`Object`] from pointer *obj_ptr*. Checks if it is null, and returns [`None`] if so.
    /// # Safety
    /// *obj_ptr* must be either a valid [`MonoObject`] pointer or null, otherwise resulting [`Object`] will not be valid and will **cause crashes**.
    pub unsafe fn from_ptr(obj_ptr:*mut crate::binds::MonoObject)->Option<Self>{
        if obj_ptr == core::ptr::null_mut(){
            return None;
        }
        return Some(Self{obj_ptr:obj_ptr});
    }
    ///Retrives and unboxed value. 
    /// # Safety 
    /// Calling it on a type which can't be unboxed **will lead to a crash**.
    /// Unboxing type 
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
    ///    let val = o.unbox::<i32>();
    ///}
    ///```
    pub fn unbox<T: InteropBox + Copy>(&self)->T{
        #[cfg(not(feature = "unsafe_unboxing"))]
        {
            let self_class = self.get_class();
            let t_class = <T as InteropBox>::get_mono_class();
            if self_class != t_class{
                panic!("tried to unbox class of type `{}` as type `{}`",&self_class.get_name(),&t_class.get_name());
            }
        }
        let ptr = unsafe{(crate::binds::mono_object_unbox(self.obj_ptr) as *mut <T as InteropRecive>::SourceType)};
        return T::get_rust_rep(unsafe{*ptr});
    }
    unsafe fn box_val_unsafe(domain:&crate::domain::Domain,class:&Class,val:*mut std::ffi::c_void)->crate::object::Object{
        return crate::object::Object::from_ptr(crate::binds::mono_value_box(domain.get_ptr(),class.get_ptr(),val)).expect("Could not box value");
    }
    ///Boxes value into an object.
    /// # Examples
    ///```
    /// let mut val:i32 = 0;
    /// let obj = Object::box_val(&domain,&int_class,val); //New object of type `Int32?`
    ///```
    pub fn box_val<T: InteropBox>(domain:&Domain,data:T)->crate::object::Object{
        let mut data = <T as InteropSend>::get_mono_rep(data); 
        let class = T::get_mono_class();
        return unsafe{Self::box_val_unsafe(
            domain,&class,&mut data as *mut <T as InteropSend>::TargetType as *mut std::ffi::c_void
        )};
    }
    ///Gets internal [`MonoObject`] pointer.
    pub fn get_ptr(&self)->*mut MonoObject{
        return self.obj_ptr;
    }
    ///Gets an implenentation virtual [`Method`] *method* for a specific [`Object`] *obj*.<br>
    /// # Explanation
    /// with given C# code
    ///```csharp
    /// class ParrentClass{
    ///     virtual void SomeMehod(){
    ///         //SomeFunction
    ///     } 
    /// }
    /// class ChildClass : ParrentClass{
    ///     override void SomeMehod(){
    ///         ///SomeOtherFunction
    ///     }
    /// }
    ///```
    /// When you call get_vitual_method on object that is instance of **ChildClass** 
    /// and method **ParrentClass::SomeMethod** you will get return value of **ChildClass::SomeMethod**.
    pub fn get_virtual_method(obj:Object,method:&Method)->Option<Method>{
        return unsafe{Method::from_ptr(crate::binds::mono_object_get_virtual_method(
            obj.get_ptr(),method.get_ptr()
        ))};
    }
}
use crate::interop::{InteropRecive,InteropSend};
impl InteropRecive for Object{
    type SourceType = *mut  crate::binds::MonoObject;
    fn get_rust_rep(arg:Self::SourceType)->Self{
        let opt = unsafe{Self::from_ptr(arg)};
        return <Object as ExceptManaged<Object>>::expect_managed_arg(opt,"Passed null reference to not nullable type! For nullable use Option<Object>!");
    }
}
impl InteropSend for Object{
    type TargetType = *mut  crate::binds::MonoObject;
    fn get_mono_rep(arg:Self)->Self::TargetType{
        return arg.get_ptr();
    }
}
impl InteropRecive for Option<Object>{
    type SourceType = *mut  crate::binds::MonoObject;
    fn get_rust_rep(arg:Self::SourceType)->Self{
        return unsafe{Object::from_ptr(arg)};
    }
}
impl InteropSend for Option<Object>{
    type TargetType = *mut  crate::binds::MonoObject;
    fn get_mono_rep(arg:Self)->Self::TargetType{
        return match arg { Some(arg)=>arg.get_ptr(),None=>core::ptr::null_mut()};
    }
}
impl Object{
    ///**Caution**: Clones MonoObject *not* reference to this object. (e.g when called on a referece to managed object A will create second object B, not another refernece to object A).
    pub fn clone_managed_object(&self)->Self{
        //if clone fails, it means that there is a much bigger problem somewhere down the line, so it can be just ignored.
        return unsafe{Self::from_ptr(crate::binds::mono_object_clone(self.obj_ptr))}.expect("MonoRuntime could not clone object!");
    }
}
//for 0.2 TODO:extend functionalities relating to properites.
