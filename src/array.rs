///Safe representation of MonoArray. Reqiures it's generic argument to implement InvokePass in order to automaticaly convert value from managed type to rust type.
pub struct Array<T:crate::invokable::InvokePass>{
    arr_ptr:*mut crate::binds::MonoArray,
    pd:std::marker::PhantomData<T>,
} 
impl<T:crate::invokable::InvokePass> Array<T>{
    ///Function returning element at index
    pub fn get(&self,index:usize)->T{
        let src:T::SourceType = unsafe{*(crate::binds::mono_array_addr_with_size(self.arr_ptr,std::mem::size_of::<T>() as i32,index) as *const T::SourceType)};
        return T::get_rust_rep(src);
    }
    ///Function returning length of the array.
    pub fn len(&self)->usize{
        return unsafe{crate::binds::mono_array_length(self.arr_ptr) as usize};
    }
    ///Function creating Array<T> from a pointer to MonoArray
    pub fn from_ptr(ptr:*mut crate::binds::MonoArray)->Self{
        return Array{arr_ptr:ptr,pd:std::marker::PhantomData};
    }
}
impl<T:crate::invokable::InvokePass> crate::invokable::InvokePass for Array<T>{
    type SourceType = *mut crate::binds::MonoArray;
    fn get_rust_rep(arg:Self::SourceType)->Self{
        return Self::from_ptr(arg);
    }
}
