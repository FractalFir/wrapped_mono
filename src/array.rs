///Safe representation of MonoArray. Reqiures it's generic argument to implement InvokePass in order to automaticaly convert value from managed type to rust type.
pub struct Array<T:crate::invokable::InvokePass + crate::invokable::InvokeReturn>{
    arr_ptr:*mut crate::binds::MonoArray,
    pd:std::marker::PhantomData<T>,
} 
impl<T:crate::invokable::InvokePass + crate::invokable::InvokeReturn> Array<T>{
    ///Function returning element at *index*.
    pub fn get(&self,index:usize)->T{
        let src:T::SourceType = unsafe{*(crate::binds::mono_array_addr_with_size(self.arr_ptr,std::mem::size_of::<T::SourceType>() as i32,index) as *const T::SourceType)};
        return T::get_rust_rep(src);
    }
    ///Function seting element at *index* to *value*
    pub fn set(&self,index:usize,value:T){
        let tmp = T::get_mono_rep(value);
        let ptr =  unsafe{(crate::binds::mono_array_addr_with_size(
            self.arr_ptr,std::mem::size_of::<T::ReturnType>() as i32,index)
            as *mut T::ReturnType)};
        unsafe{(*ptr) = tmp};
    }
    ///Function returning length of the array.
    pub fn len(&self)->usize{
        return unsafe{crate::binds::mono_array_length(self.arr_ptr) as usize};
    }
    ///Function creating Array<T> from a pointer to MonoArray
    pub unsafe fn from_ptr(ptr:*mut crate::binds::MonoArray)->Self{
        return Array{arr_ptr:ptr,pd:std::marker::PhantomData};
    }
    ///Alocate new array in *domain* holding *n* elements of type *class*. 
    pub fn new(domain:crate::domain::Domain,class:crate::class::Class,n:usize)->Self{
        return unsafe{Self::from_ptr(
            crate::binds::mono_array_new(domain.get_ptr(),class.get_ptr(),n)
        )};
    }
    ///Function returning copy internal pointer to MonoArray
    pub fn get_ptr(&self)->*mut crate::binds::MonoArray{
        return self.arr_ptr;
    }
}
impl<T:crate::invokable::InvokePass + crate::invokable::InvokeReturn> crate::invokable::InvokePass for Array<T>{
    type SourceType = *mut crate::binds::MonoArray;
    fn get_rust_rep(arg:Self::SourceType)->Self{
        return unsafe{Self::from_ptr(arg)};
    }
}
impl<T:crate::invokable::InvokePass + crate::invokable::InvokeReturn> Clone for Array<T>{
    fn clone(&self)->Self{
        return unsafe{Self::from_ptr(crate::binds::mono_array_clone(self.arr_ptr))};
    }
}
use crate::binds::MonoObject;
impl<T:crate::invokable::InvokePass + crate::invokable::InvokeReturn> crate::object::ObjectTrait for Array<T>{
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
    fn is_inst(&self,class:crate::class::Class)->Option<crate::object::Object>{
        return unsafe{crate::object::Object::from_ptr(
            crate::binds::mono_object_isinst(self.get_ptr() as *mut crate::binds::MonoObject,class.get_ptr())
        )};
    }
}