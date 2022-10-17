use crate::class::Class;
use crate::binds::{MonoObject};
use crate::method::{Method,MethodTrait};
use crate::domain::Domain;
use crate::exception::except_managed;
use crate::gc::{GCHandle,gc_unsafe_enter,gc_unsafe_exit};
use crate::interop::{InteropRecive,InteropSend};
#[allow(unused_imports)] // for docs
use crate::delegate::Delegate;
///Safe representation of a refernece to a manged Object. Is **not nullable** when passed between managed and unmanged code(e.g when added as an argument to function exposed as an interna call). 
///It means that while it may represent a nullable type, wrapped-mono will automaticly panic when recived null value.
///For nullable support use `Option<Object>`.
pub struct Object{
    #[cfg(not(feature = "referneced_objects"))]
    obj_ptr:*mut crate::binds::MonoObject,
    #[cfg(feature = "referneced_objects")]
    handle:GCHandle,
}
use crate::mstring::MString;
///Trait contining functions common for all types of manged objects.
pub trait ObjectTrait{
    /// get hash of this object: This hash is **not** based on values of objects fields, and differs from result of calling object.GetHash()
    /// # Example 
    /// ```rust
    /// let object = Object::new(&domain,&class);
    /// let object_copy = object.clone_managed_object();
    /// assert!(object.hash() != object_copy.hash()); // Objects object and object_copy have exacly 
    /// // the same values of their fileds, but are diffrent instances, so their hash is diffrent.
    /// ```
    fn hash(&self)->i32;
    /// get [`Domain`] this object exists in.
    /// # Example
    ///```rust
    /// let domain = Domain.create(); //create Domain dom
    /// let object = Object::new(&dom,&class); //create object in Domain dom.
    /// let obj_domain = object.get_domain(); //get doamin object is in
    /// assert!(domain == obj_domain); 
    ///```
    fn get_domain(&self)->crate::domain::Domain;
    /// get size of managed object referenced by *self* in bytes. Does include builtin hidden data.
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
    /// get reflection token 
    //TODO:extend this description to make it more clear
    #[doc(hidden)]
    fn reflection_get_token(&self)->u32;
    /// returns [`Class`] of this object.
    /// # Example
    /// ```rust 
    /// let object = Object::new(&domain,&class);
    /// let object_class = object.get_class();
    /// assert!(class == object_class);
    /// ```
    fn get_class(&self)->Class;
    /// Returns result of calling ToString on this [`Object`]. Returns [`Exception`] if raised, and [`Option<MString>`] if not. Function returns [`Option<MString>`] to allow for null value to be returned. 
    fn to_mstring(&self)->Result<Option<MString>,Exception>;
    /// Casts a type implementing [`ObjectTrait`] to an object.
    fn cast_to_object(&self)->Object;
    /// Tries to cast an object to a sepcific object type, and returns None if canst impossible.
    /// # WARNING
    /// This cast does not work fully for [`Delegate`]-s with less than 2 arguments(casts that should fail will not fail).
    fn cast_from_object(obj:&Object)->Option<Self> where Self:Sized;
}
use crate::exception::Exception;
impl ObjectTrait for Object{
    fn hash(&self)->i32{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let hsh = unsafe{crate::binds::mono_object_hash(self.get_ptr())};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        hsh
    }
    fn get_domain(&self)->Domain{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let dom = unsafe{Domain::from_ptr(crate::binds::mono_object_get_domain(self.get_ptr()))};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        dom
    }
    fn get_size(&self)->u32{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let size = unsafe{crate::binds:: mono_object_get_size(self.get_ptr())};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        size
    }
    fn reflection_get_token(&self)->u32{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let tok = unsafe{crate::binds::mono_reflection_get_token(self.get_ptr())};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        tok
    }
    fn get_class(&self)->Class{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let class = unsafe{Class::from_ptr(
            crate::binds::mono_object_get_class(self.get_ptr())
        ).expect("Could not get class of an object")};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        class
    }
    fn to_mstring(&self)->Result<Option<MString>,Exception>{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let mut exc:*mut crate::binds::MonoException = core::ptr::null_mut();
        let res = unsafe{MString::from_ptr(
            crate::binds::mono_object_to_string(self.get_ptr(),&mut exc as *mut *mut crate::binds::MonoException as *mut *mut crate::binds::MonoObject)
        )};
        let exc = unsafe{Exception::from_ptr(exc)};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        match exc{
            Some(e)=>Err(e),
            None=>Ok(res),
        }
    }
    fn cast_to_object(&self)->Object{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let obj = unsafe{Self::from_ptr(self.get_ptr())}.unwrap();//Faliure impossible, object is always an object.
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        obj
    }
    fn cast_from_object(obj:&Object)->Option<Self>{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let obj = unsafe{Self::from_ptr(obj.get_ptr())};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        obj
    }
}
use crate::interop::InteropBox;
impl Object{ 
    ///returns [`Object`] *self* cast to *class* if *self* is derived from [`Class`] class. Does not affect original reference to object nor the object itself.
    pub fn is_inst(&self,class:&Class)->Option<Object>{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let inst = unsafe{Self::from_ptr(crate::binds::mono_object_isinst(self.get_ptr(),class.get_ptr()))};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        inst
    }
    ///Allocates new object of [`Class`] class. **Does not call the constructor**, to call constuctor call the `.ctor` method after creating the object. 
    /// # Examples
    /// ```rust
    /// let new_obj = Object::new(some_domain,new_objects_class);
    /// ```
    pub fn new(domain:&crate::domain::Domain,class:&Class)->Self{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let obj = unsafe{Self::from_ptr(crate::binds::mono_object_new(domain.get_ptr(),class.get_ptr()))}.expect("Could not create new type from class!");
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        obj
    }
    ///Creates new [`Object`] from pointer *obj_ptr*. Checks if it is null, and returns [`None`] if so.
    /// # Safety
    /// *obj_ptr* must be either a valid [`MonoObject`] pointer or null, otherwise resulting [`Object`] will not be valid and will **cause crashes**.
    pub unsafe fn from_ptr(obj_ptr:*mut crate::binds::MonoObject)->Option<Self>{
        #[cfg(not(feature = "referneced_objects"))]
        {
            if obj_ptr.is_null(){
                return None;
            }
            Some(Self{obj_ptr})
        }
        #[cfg(feature = "referneced_objects")]
        {
            if obj_ptr.is_null(){
                return None;
            }
            Some(Self{handle:GCHandle::create_default(obj_ptr)})
        }
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
        #[cfg(not(feature = "unsafe_boxing"))]
        {
            let self_class = self.get_class();
            let t_class = <T as InteropClass>::get_mono_class();
            if self_class != t_class{
                panic!("tried to unbox class of type `{}` as type `{}`",&self_class.get_name(),&t_class.get_name());
            }
        }
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let ptr = unsafe{crate::binds::mono_object_unbox(self.get_ptr()) as *mut <T as InteropRecive>::SourceType};
        let res = T::get_rust_rep(unsafe{*ptr});
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    unsafe fn box_val_unsafe(domain:&crate::domain::Domain,class:&Class,val:*mut std::ffi::c_void)->crate::object::Object{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res =crate::object::Object::from_ptr(crate::binds::mono_value_box(domain.get_ptr(),class.get_ptr(),val)).expect("Could not box value");
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
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
        unsafe{Self::box_val_unsafe(
            domain,&class,&mut data as *mut <T as InteropSend>::TargetType as *mut std::ffi::c_void
        )}
    }
    ///Gets internal [`MonoObject`] pointer.
    pub fn get_ptr(&self)->*mut MonoObject{
        #[cfg(not(feature = "referneced_objects"))]
        {
            self.obj_ptr
        }
        #[cfg(feature = "referneced_objects")]
        {
            self.handle.get_target()
        }
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
    pub fn get_virtual_method<T:InteropSend> (obj:Object,method:&Method<T>)->Option<Method<T>>{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Method::from_ptr(crate::binds::mono_object_get_virtual_method(
            obj.get_ptr(),method.get_ptr()
        ))};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
}
impl InteropRecive for Object{
    type SourceType = *mut  crate::binds::MonoObject;
    // unless this function is abused, this argument should come from the mono runtime, so it should be always valid.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn get_rust_rep(arg:Self::SourceType)->Self{
        let opt = unsafe{Self::from_ptr(arg)};
        except_managed(opt,"Rust function argument type is not nullable, but got null!For nullable types use Option<Object>!")
    }
}
impl InteropSend for Object{
    type TargetType = *mut  crate::binds::MonoObject;
    fn get_mono_rep(arg:Self)->Self::TargetType{
        arg.get_ptr()
    }
}
impl InteropRecive for Option<Object>{
    type SourceType = *mut  crate::binds::MonoObject;
    // unless this function is abused, this argument should come from the mono runtime, so it should be always valid.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn get_rust_rep(arg:Self::SourceType)->Self{
        unsafe{Object::from_ptr(arg)}
    }
}
impl InteropSend for Option<Object>{
    type TargetType = *mut  crate::binds::MonoObject;
    fn get_mono_rep(arg:Self)->Self::TargetType{
        match arg { Some(arg)=>arg.get_ptr(),None=>core::ptr::null_mut()}
    }
}
impl Object{
    ///Clones the underlying MonoObject *not* the reference to this object. (e.g when called on a reference to a managed object A will create second object B, not another reference to object A).
    pub fn clone_managed_object(&self)->Self{
        //if clone fails, it means that there is a much bigger problem somewhere down the line, so it can be just ignored.
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(crate::binds::mono_object_clone(self.get_ptr()))}.expect("MonoRuntime could not clone object!");
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
}
//for 0.2 TODO:extend functionalities relating to properites.
use crate::interop::InteropClass;
impl InteropClass for Object{
    fn get_mono_class()->Class{
        Class::get_object()
    }
}
impl InteropClass for Option<Object>{
    fn get_mono_class()->Class{
        Class::get_object()
    }
}
impl<O:ObjectTrait> PartialEq<O> for Object{
    fn eq(&self,other:&O)->bool{
        self.get_ptr() == other.cast_to_object().get_ptr()
    }
}
impl Clone for Object{
    fn clone(&self)->Self{
        unsafe{Self::from_ptr(self.get_ptr()).unwrap()}//If object exists then it can't be null
    }
}
