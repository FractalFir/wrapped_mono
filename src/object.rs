use crate::binds::MonoObject;
#[allow(unused_imports)] // for docs
// use crate::delegate::Delegate;
use crate::{domain::Domain,class::Class,method::Method};
use crate::gc::{gc_unsafe_enter, gc_unsafe_exit, GCHandle};
use crate::interop::{InteropRecive, InteropSend};
use crate::tupleutilis::{CompareClasses, TupleToPtrs};
///Safe representation of a refernece to a manged Object. Is **not nullable** when passed between managed and unmanged code(e.g when added as an argument to function exposed as an interna call).
///It means that while it may represent a nullable type, wrapped-mono will automaticly panic when recived null value.
///For nullable support use `Option<Object>`.
pub struct Object {
    #[cfg(not(feature = "referneced_objects"))]
    obj_ptr: *mut MonoObject,
    #[cfg(feature = "referneced_objects")]
    handle: GCHandle,
}
use crate::mstring::MString;
///Trait contining functions common for all types of manged objects.
pub trait ObjectTrait: Sized + InteropClass {
    fn cast<Target: ObjectTrait>(&self) -> Option<Target> {
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe { Target::from_ptr(self.get_ptr()) };
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Gets the internal [`MonoObject`] pointer.
    #[must_use]
    fn get_ptr(&self) -> *mut MonoObject;
    /// Creates new instance of [`Self`] from *mut [`MonoObject`]. Returns `None` if either `obj_ptr` is null OR object `obj_ptr` points to is of a type which does not derive from the managed type [`Self`] represents.
    /// # Safety
    /// Pointer must either be null, or point to a managed object.
    #[must_use]
    unsafe fn from_ptr(obj_ptr: *mut MonoObject) -> Option<Self> {
        let class = Self::get_mono_class();
        let obj_ptr = crate::binds::mono_object_isinst(obj_ptr, class.get_ptr());
        if obj_ptr.is_null() {
            None
        } else {
            Some(Self::from_ptr_unchecked(obj_ptr))
        }
    }
    /// Creates new instance of [`Self`] from *mut [`MonoObject`]. Pointer is guaranteed to be not null, and of type which can be assigned to managed type represented by [`Self`].
    /// # Safety
    /// The pointer must not be null, and point to a managed Object of either type represented by [`Self`] or a type derived from it.
    #[must_use]
    unsafe fn from_ptr_unchecked(obj: *mut MonoObject) -> Self;
    /// get hash of this object: This hash is **not** based on values of objects fields, and differs from result of calling object.GetHash()
    /// # Example
    /// ```no_run
    /// # use wrapped_mono::*;
    /// # let class = Class::get_int_32();
    /// # let domain = Domain::get_current().unwrap();
    /// let object = Object::new(&domain,&class);
    /// let object_copy = object.clone_managed_object();
    /// assert!(object.hash() != object_copy.hash()); // Objects object and object_copy have exacly
    /// // the same values of their fileds, but are diffrent instances, so their hash is diffrent.
    /// ```
    #[must_use]
    fn hash(&self) -> i32 {
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let hsh = unsafe { crate::binds::mono_object_hash(self.get_ptr()) };
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        hsh
    }
    /// get [`Domain`] this object exists in.
    /// # Example
    ///```no_run
    /// # use wrapped_mono::*;
    /// # let class = Class::get_int_32();
    /// let domain = Domain::create(); //create Domain dom
    /// let object = Object::new(&domain,&class); //create object in Domain dom.
    /// let obj_domain = object.get_domain(); //get doamin object is in
    /// assert!(domain == obj_domain);
    ///```
    #[must_use]
    fn get_domain(&self) -> Domain {
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let dom = unsafe { Domain::from_ptr(crate::binds::mono_object_get_domain(self.get_ptr())) };
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        dom
    }
    /// get size of managed object referenced by *self* in bytes. Does include builtin hidden data.
    /// # Example
    ///```ignore
    /// class SomeClass{};
    /// class OtherClass{int some_int;};
    ///```
    ///```no_run
    /// # use wrapped_mono::*;
    /// # use wrapped_mono::binds::MonoObject;
    /// # let domain = Domain::get_current().unwrap();
    /// # let some_obj = Object::new(&domain,&Class::get_void());
    /// # let other_obj = Object::box_val::<i32>(&domain,77);
    /// let size = some_obj.get_size();  //Get size of some_obj(in this case an instance of SomeClass)
    /// assert!(size == std::mem::size_of::<MonoObject>() as u32); // 8 bytes on 32 bit systems, 16 on 64 bit ones (size of two pointers).
    /// let size_other = other_obj.get_size(); //Get size of other_obj(in this case an instance of OtherClass)
    /// assert!(size_other == (std::mem::size_of::<MonoObject>() + std::mem::size_of::<i32>()) as u32); //size of two hidden pointers + some_int filed.
    ///```
    #[must_use]
    fn get_size(&self) -> u32 {
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let size = unsafe { crate::binds::mono_object_get_size(self.get_ptr()) };
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        size
    }
    /// get reflection token
    //TODO:extend this description to make it more clear
    #[doc(hidden)]
    fn reflection_get_token(&self) -> u32 {
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let tok = unsafe { crate::binds::mono_reflection_get_token(self.get_ptr()) };
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        tok
    }
    /// Returns [`Class`] of this object. NOTE: This is function returns the class of the underlying object, not class represented by [`Self`]. This means that class returned from `get_class` may be a class derived from class [`Self`] represents.
    /// # Example
    /// ```no_run
    /// # use wrapped_mono::*;
    /// # let domain = Domain::get_current().unwrap();
    /// # let class = Class::get_void();
    /// let object = Object::new(&domain,&class);
    /// let object_class = object.get_class();
    /// assert!(class == object_class);
    /// ```
    #[must_use]
    fn get_class(&self) -> Class {
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let class = unsafe {
            Class::from_ptr(crate::binds::mono_object_get_class(self.get_ptr()))
                .expect("Could not get class of an object")
        };
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        class
    }
    /// Returns result of calling `ToString` on this [`Object`].
    /// # Errors
    /// Returns [`Exception`] if raised, and [`Option<MString>`] if not. Function returns [`Option<MString>`] to allow for null value to be returned.
    fn to_mstring(&self) -> Result<Option<MString>, Exception> {
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let mut exc: *mut crate::binds::MonoException = core::ptr::null_mut();
        let res = unsafe {
            MString::from_ptr(
                crate::binds::mono_object_to_string(
                    self.get_ptr(),
                    std::ptr::addr_of_mut!(exc).cast::<*mut MonoObject>(),
                )
                .cast::<MonoObject>(),
            )
        };
        let exc = unsafe { Exception::from_ptr(exc.cast()) };
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        exc.map_or_else(|| Ok(res), Err)
    }
}
use crate::exception::Exception;
impl ObjectTrait for Object {
    ///Gets internal [`MonoObject`] pointer.
    fn get_ptr(&self) -> *mut MonoObject {
        #[cfg(not(feature = "referneced_objects"))]
        {
            self.obj_ptr
        }
        #[cfg(feature = "referneced_objects")]
        {
            self.handle.get_target()
        }
    }
    unsafe fn from_ptr_unchecked(obj_ptr: *mut MonoObject) -> Self {
        debug_assert!(
            !obj_ptr.is_null(),
            "Error: Violated function contract. *obj_ptr* must never be null, but was null."
        );
        #[cfg(not(feature = "referneced_objects"))]
        {
            Self { obj_ptr }
        }
        #[cfg(feature = "referneced_objects")]
        {
            Self {
                handle: GCHandle::create_default(obj_ptr),
            }
        }
    }
}
use crate::interop::InteropBox;
impl Object {
    ///Allocates new object of [`Class`] class. **Does not call the constructor**, to call constuctor call the `.ctor` method after creating the object.
    /// # Examples
    /// ```no_run
    /// # use wrapped_mono::*;
    /// # let domain = Domain::get_current().unwrap();
    /// # let class = Class::get_void();
    /// let new_obj = Object::new(&domain,&class);
    /// ```
    #[must_use]
    pub fn new(domain: &crate::domain::Domain, class: &Class) -> Self {
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let obj = unsafe {
            Self::from_ptr(crate::binds::mono_object_new(
                domain.get_ptr(),
                class.get_ptr(),
            ))
        }
        .expect("Could not create new type from class!");
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        obj
    }
    /// Creates new [`Object`] from pointer *`obj_ptr`*. Checks if it is null, and returns [`None`] if so.
    /// # Safety
    /// *`obj_ptr`* must be either a valid [`MonoObject`] pointer or null, otherwise resulting [`Object`] will not be valid and will **cause crashes**.
    #[must_use]
    pub unsafe fn from_ptr(obj_ptr: *mut MonoObject) -> Option<Self> {
        #[cfg(not(feature = "referneced_objects"))]
        {
            if obj_ptr.is_null() {
                return None;
            }
            Some(Self { obj_ptr })
        }
        #[cfg(feature = "referneced_objects")]
        {
            if obj_ptr.is_null() {
                return None;
            }
            Some(Self {
                handle: GCHandle::create_default(obj_ptr),
            })
        }
    }
    /// Unboxes the value in [`Object`] `self`.
    /// # Safety
    /// Calling it on a type which can't be unboxed **will lead to a crash**.
    /// # Panics
    /// Type T must match the unboxed managed type.
    /// Unboxing type
    ///C#<br>
    ///```ignore
    ///int num = 123;
    ///Object boxed = num;
    ///RustFunction(boxed);
    ///```
    ///Rust
    ///```no_run
    /// # use wrapped_mono::*;
    ///#[invokable]
    ///fn rust_function(o:Object){
    ///    let val = o.unbox::<i32>();
    ///}
    ///```
    #[must_use]
    pub fn unbox<T: InteropBox + Copy>(&self) -> T {
        #[cfg(not(feature = "unsafe_boxing"))]
        {
            let self_class = self.get_class();
            let t_class = <T as InteropClass>::get_mono_class();
            assert!(
                self_class == t_class,
                "tried to unbox class of type `{}` as type `{}`",
                &self_class.get_name(),
                &t_class.get_name()
            );
        }
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let ptr = unsafe {
            crate::binds::mono_object_unbox(self.get_ptr())
                .cast::<<T as InteropRecive>::SourceType>()
        };
        let res = T::get_rust_rep(unsafe { *ptr });
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    unsafe fn box_val_unsafe(
        domain: &crate::domain::Domain,
        class: &Class,
        val: *mut std::ffi::c_void,
    ) -> Self {
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = Self::from_ptr(crate::binds::mono_value_box(
            domain.get_ptr(),
            class.get_ptr(),
            val,
        ))
        .expect("Could not box value");
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Boxes value into an object.
    /// # Examples
    ///```no_run
    /// # use wrapped_mono::*;
    /// # let domain = Domain::get_current().unwrap();
    /// let mut val:i32 = 0;
    /// let obj = Object::box_val::<i32>(&domain,val); //New object of type `Int32?`
    ///```
    pub fn box_val<T: InteropBox>(domain: &Domain, data: T) -> Self {
        let mut data = <T as InteropSend>::get_mono_rep(data);
        let class = T::get_mono_class();
        unsafe {
            Self::box_val_unsafe(
                domain,
                &class,
                std::ptr::addr_of_mut!(data).cast::<std::ffi::c_void>(),
            )
        }
    }
    ///Gets an implementation virtual [`Method`] *`method`* for a specific [`Object`] *`obj`*.<br>
    /// # Explanation
    /// with given C# code
    ///```ignore
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
    /// When you call`get_vitual_method` on object that is instance of **`ChildClass`**
    /// and method **`ParrentClass::SomeMethod`** you will get return value of **`ChildClass::SomeMethod`**.
    #[must_use]
    pub fn get_virtual_method<T: TupleToPtrs + CompareClasses + InteropSend>(
        obj: &Self,
        method: &Method<T>,
    ) -> Option<Method<T>>
    where
        <T as InteropSend>::TargetType: TupleToPtrs,
    {
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Method::from_ptr(crate::binds::mono_object_get_virtual_method(
                obj.get_ptr(),
                method.get_ptr(),
            ))
        };
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
}
impl Object {
    ///Clones the underlying [`MonoObject`] *not* the reference to this object. (e.g when called on a reference to a managed object A will create second object B, not another reference to object A).
    #[must_use]
    pub fn clone_managed_object(&self) -> Self {
        //if clone fails, it means that there is a much bigger problem somewhere down the line, so it can be just ignored.
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe { Self::from_ptr(crate::binds::mono_object_clone(self.get_ptr())) }
            .expect("MonoRuntime could not clone object!");
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
}
//for 0.2 TODO:extend functionalities relating to properties.
use crate::interop::InteropClass;
impl InteropClass for Object {
    fn get_mono_class() -> Class {
        Class::get_object()
    }
}
impl InteropClass for Option<Object> {
    fn get_mono_class() -> Class {
        Class::get_object()
    }
}
impl<O: ObjectTrait> PartialEq<O> for Object {
    fn eq(&self, other: &O) -> bool {
        self.get_ptr() == other.get_ptr()
    }
}
impl Clone for Object {
    fn clone(&self) -> Self {
        unsafe { Self::from_ptr(self.get_ptr()).unwrap() } //If object exists then it can't be null
    }
}
