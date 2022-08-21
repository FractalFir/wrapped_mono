use crate::interop::{InteropRecive,InteropSend};
/// Safe representation of MonoArray(a reference to a managed array). Reqiures it's generic argument to implement InvokePass in order to automaticaly convert value from managed type to rust type.
/// # Safety
/// It is possible to use wrong type Array (e.g. casting float[] to Array<String>) and either cause a crash or read a garbage value.
/// # Nullable support
/// [`Array<T>`] is non-nullable on defult and will panic when null passed as argument form managed code. For nullable support use [`Option<Array<T>>`].
pub struct Array<T:InteropRecive + InteropSend>{
    arr_ptr:*mut crate::binds::MonoArray,
    pd:std::marker::PhantomData<T>,
} 
impl<T:InteropRecive + InteropSend> Array<T>{
    ///Function returning element at *index*
    /// # Example
    ///```rust
    /// fn some_fn(input:&Array<f32>)->f32{
    ///     let a = input.get(0);  
    ///     let b = input.get(1);
    ///     return a + b;  
    ///}
    ///```
    pub fn get(&self,index:usize)->T{
        let src:T::SourceType = unsafe{*(crate::binds::mono_array_addr_with_size(self.arr_ptr,std::mem::size_of::<T::SourceType>() as i32,index) as *const T::SourceType)};
        return T::get_rust_rep(src);
    }
    ///Function seting element at *index* to *value*
    /// # Example
    ///```rust
    /// fn set_fn(input:&mut Array<i32>){
    ///     input.set(0,0);
    ///     input.set(1,1);
    /// }
    ///```
    pub fn set(&mut self,index:usize,value:T){
        let tmp = T::get_mono_rep(value);
        let ptr =  unsafe{crate::binds::mono_array_addr_with_size(
            self.arr_ptr,std::mem::size_of::<T::TargetType>() as i32,index)
            as *mut T::TargetType};
        unsafe{(*ptr) = tmp};
    }
    ///Function returning length of the array.
    /// # Example
    ///```rust
    /// fn get_avg(input:&Array<f32>)->f32{
    ///     let mut sum = 0.0;
    ///     for i in 0..input.len{
    ///         sum+=input.get(i);
    ///     }
    ///     return sum/(input.len() as f32);
    /// }
    ///```
    pub fn len(&self)->usize{
        return unsafe{crate::binds::mono_array_length(self.arr_ptr) as usize};
    }
    ///Function creating Array<T> from a pointer to MonoArray
    /// # Safety
    /// Pointer must be either a pointer to valid MonoAray of the same type, or a null pointer. Invalid values may lead to undefined behaviour and crashes.
    pub unsafe fn from_ptr(ptr:*mut crate::binds::MonoArray)->Option<Self>{
        if ptr == null_mut(){
            return None;
        }
        return Some(Array{arr_ptr:ptr,pd:std::marker::PhantomData});
    }
    ///Alocate new array in *domain* holding *n* elements of type *class*. 
    /// # Example
    ///```rust
    /// let arr_len = 8;
    /// let arr = Array<i32>::new(&domain,&int_managed_class,arr_len);
    /// assert!(arr.len() == arr_len);
    ///```
    pub fn new(domain:&crate::domain::Domain,class:&crate::class::Class,n:usize)->Self{
        return unsafe{Self::from_ptr(
            crate::binds::mono_array_new(domain.get_ptr(),class.get_ptr(),n)
        )}.expect("could not create a new array!");
    }
    ///Function returning a copy of internal pointer to MonoArray
    pub fn get_ptr(&self)->*mut crate::binds::MonoArray{
        return self.arr_ptr;
    }
    ///Clones managed array, **not** the refernece to it.
    pub fn clone_managed_array(&self)->Self{
        return unsafe{Self::from_ptr(crate::binds::mono_array_clone(self.arr_ptr))}.expect("coud not create copy of an array!");
    }
}
impl<T:InteropRecive + InteropSend> InteropRecive for Array<T>{
    type SourceType = *mut crate::binds::MonoArray;
    fn get_rust_rep(arg:Self::SourceType)->Self{
        use crate::exception::ExceptManaged;
        let opt = unsafe{Self::from_ptr(arg)};
        return <Array<T> as ExceptManaged<Array<T>>>::expect_managed_arg(opt,"Got null in an not nullable type. For nullable support use Option<Array>");
    }
}
impl<T:InteropRecive + InteropSend> InteropSend for Array<T>{
    type TargetType = *mut crate::binds::MonoArray;
    fn get_mono_rep(arg:Self)->Self::TargetType{
        return arg.get_ptr();
    }
}
use core::ptr::null_mut;
use crate::binds::MonoObject;
use crate::mstring::MString;
use crate::Exception;
impl<T:InteropRecive + InteropSend> crate::object::ObjectTrait for Array<T>{
    fn hash(&self)->i32{
        return unsafe{crate::binds::mono_object_hash(self.arr_ptr as *mut MonoObject)};
    }
    fn get_domain(&self)->crate::domain::Domain{
        return unsafe{crate::domain::Domain::from_ptr(crate::binds::mono_object_get_domain(self.arr_ptr as *mut MonoObject))};
    }
    fn get_size(&self)->u32{
        return unsafe{crate::binds:: mono_object_get_size(self.arr_ptr as *mut MonoObject)};
    }
    fn reflection_get_token(&self)->u32{
        return unsafe{crate::binds::mono_reflection_get_token(self.arr_ptr as *mut MonoObject)};
    }
    fn get_class(&self)->crate::class::Class{
        return unsafe{crate::class::Class::from_ptr(
            crate::binds::mono_object_get_class(self.arr_ptr as *mut MonoObject)
        ).expect("Could not get class of an object")};
    }
    fn is_inst(&self,class:&crate::class::Class)->Option<crate::object::Object>{
        return unsafe{crate::object::Object::from_ptr(
            crate::binds::mono_object_isinst(self.get_ptr() as *mut crate::binds::MonoObject,class.get_ptr())
        )};
    }
    fn to_string(&self)->Result<Option<MString>,Exception>{
        let mut exc:*mut crate::binds::MonoException = core::ptr::null_mut();
        let res = unsafe{MString::from_ptr(
            crate::binds::mono_object_to_string(self.arr_ptr as *mut crate::binds::MonoObject,&mut exc as *mut *mut crate::binds::MonoException as *mut *mut crate::binds::MonoObject)
        )};
        let exc = unsafe{Exception::from_ptr(exc)};
        match exc{
            Some(e)=>return Err(e),
            None=>return Ok(res),
        }
    }
}
impl<T:InteropRecive + InteropSend> InteropRecive for Option<Array<T>>{
    type SourceType = *mut crate::binds::MonoArray;
    fn get_rust_rep(arg:Self::SourceType)->Self{
        return unsafe{Array::<T>::from_ptr(arg)};
    }
}
impl<T:InteropRecive + InteropSend> InteropSend for Option<Array<T>>{
    type TargetType = *mut crate::binds::MonoArray;
    fn get_mono_rep(arg:Self)->Self::TargetType{
        return match arg{ Some(arg)=>arg.get_ptr(),None=>null_mut()};
    }
}