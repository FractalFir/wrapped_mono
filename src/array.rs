use crate::binds::{MonoArray, MonoObject};
use crate::gc::{gc_unsafe_enter, gc_unsafe_exit, GCHandle};
use crate::interop::{InteropClass, InteropRecive, InteropSend};
use crate::{
    dimensions::DimensionTrait, domain::Domain, mstring::MString, Class, Exception, Object,
    ObjectTrait,
};
use core::marker::PhantomData;
use core::ptr::null_mut;
use std::borrow::{Borrow, BorrowMut};
use std::ops::Index;
// Documentation finished.
/// Safe representation of [`MonoArray`] (a reference to a managed array). Requires it's generic argument to implement [`InvokePass`] in order to automatically convert value from managed type to rust type.
/// Will panic on creating an array with type mismatch between runtime and rust.
/// # Nullable support
/// [`Array<T>`] is non-nullable on default and will panic when null passed as argument form managed code. For nullable support use [`Option<Array<T>>`].
/*
    why is there a weird constraint "where [();DIMENSIONS as usize]:Copy" in array type? It guarantees that Dimensions is higher than 0 and size array is larger than 0,
    so Array<DIMENSIONS,T> can exist.
*/
pub struct Array<Dim: DimensionTrait, T: InteropSend + InteropRecive + InteropClass>
where
    Dim::Lengths: std::ops::IndexMut<usize> + BorrowMut<[usize]> + Copy,
    <Dim::Lengths as std::ops::Index<usize>>::Output: BorrowMut<usize>,
    <<Dim as DimensionTrait>::Lengths as Index<usize>>::Output: Sized + Into<usize> + Copy,
{
    #[cfg(not(feature = "referneced_objects"))]
    arr_ptr: *mut MonoArray,
    #[cfg(feature = "referneced_objects")]
    handle: GCHandle,
    pd: PhantomData<T>,
    lengths: Dim::Lengths,
}
impl<T: InteropSend + InteropRecive + InteropClass, Dim: DimensionTrait> Array<Dim, T>
where
    Dim::Lengths: std::ops::IndexMut<usize> + BorrowMut<[usize]> + Copy + Copy,
    <Dim::Lengths as std::ops::Index<usize>>::Output: BorrowMut<usize>,
    <<Dim as DimensionTrait>::Lengths as Index<usize>>::Output: Sized + Into<usize> + Copy,
    <Dim as DimensionTrait>::Lengths: BorrowMut<[usize]>,
{
    // Private function used to calculate index in an array based on its dimensions.
    fn get_index(&self, indices: Dim::Lengths) -> usize {
        //size of current dimension
        let mut size = 1;
        let mut index = 0;
        for (n, ind) in indices.borrow().iter().enumerate() {
            let len = self.lengths[n];
            #[cfg(not(feature = "unsafe_arrays"))]
            assert!(
                *ind < len.into(),
                "index ({}) outside of array bound ({})",
                ind,
                len.into()
            );
            index += ind * size;
            size *= len.into();
        }
        index
    }
    /// Function returning element at *index*
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |self|&Self|[`Array`] to read from.|
    /// |indices|`[usize;DIMENSIONS as usize]`| An n-dimensional array containing indices to read value at|
    /// # Examples
    /// ```no_run
    /// # use wrapped_mono::*;
    /// fn some_get_fn(input:&Array<Dim1D,f32>)->f32{
    ///     let a = input.get([0]);  
    ///     let b = input.get([1]);
    ///     a + b
    /// }
    /// ```
    /// ```no_run
    /// # use wrapped_mono::*;
    /// fn some_get_fn_2D(input:&Array<Dim2D,f32>)->f32{
    ///     let a = input.get([0,0]);  
    ///     let b = input.get([1,1]);
    ///     let c = input.get([0,1]);
    ///      a + b + c
    /// }
    /// ```
    pub fn get(&self, indices: Dim::Lengths) -> T {
        let index = self.get_index(indices);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_possible_wrap)]
        let src: T::SourceType = unsafe {
            *(crate::binds::mono_array_addr_with_size(
                self.get_ptr(),
                std::mem::size_of::<T::SourceType>() as i32,
                index,
            ) as *const T::SourceType)
        };
        let rr = T::get_rust_rep(src);
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        rr
    }
    /// Function setting element at *index* of [`Array`] to *value*
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |self|&Self|[`Array`] to write value to.|
    /// |indices|`[usize;DIMENSIONS as usize]`| An n-dimensional array containing indices to set value at|
    /// |value  |`T`|value to set element at index to.|
    /// # Example
    /// ```no_run
    /// # use wrapped_mono::*;
    /// fn set_fn(input:&mut Array<Dim1D,i32>){
    ///     input.set([0],0);
    ///     input.set([1],1);
    /// }
    /// ```
    /// ```no_run
    /// # use wrapped_mono::*;
    /// fn set_fn_2D(input:&mut Array<Dim2D,i32>){
    ///     input.set([0,0],0);
    ///     input.set([1,1],1);
    ///     input.set([1,0],9);
    /// }
    /// ```
    pub fn set(&mut self, indices: Dim::Lengths, value: T) {
        let tmp = T::get_mono_rep(value);
        let index = self.get_index(indices);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let ptr = unsafe {
            #[allow(clippy::cast_possible_truncation)]
            #[allow(clippy::cast_possible_wrap)]
            crate::binds::mono_array_addr_with_size(
                self.get_ptr(),
                std::mem::size_of::<T::TargetType>() as i32,
                index,
            )
            .cast()
        };
        unsafe { (*ptr) = tmp };
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
    }
    /// Function returning 1D length of the array(element count).
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |self|&Self|[`Array`] to get length of|
    /// # Example
    /// ```no_run
    /// # use wrapped_mono::*;
    /// fn get_avg(input:&Array<Dim1D,f32>)->f32{
    ///     let mut sum = 0.0;
    ///     for i in 0..input.len(){
    ///         sum += input.get([i]);
    ///     }
    ///     sum/(input.len() as f32)
    /// }
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let len = unsafe { crate::binds::mono_array_length(self.get_ptr()) };
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        len
    }
    /// Checks if [`Array`] is empty.
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |self|&Self|[`Array`] to check if is empty|
    #[must_use]
    pub fn is_empty(&self) -> bool {
        0 == self.len()
    }
    /// Function creating [`Array<T>`] from a pointer to [`MonoArray`]
    /// # Safety
    /// Pointer must be either a pointer to valid [`MonoArray`] of the same type, or a null pointer. Invalid values may lead to undefined behaviour and crashes.
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |ptr| *mut [`MonoArray`] | pointer to array to create representation for|
    /// # Panics
    /// Will panic if the array dimension count or element type mismatch.
    #[must_use]
    pub unsafe fn from_ptr(ptr: *mut MonoArray) -> Option<Self> {
        use crate::Method;
        if ptr.is_null() {
            return None;
        }
        #[cfg(not(feature = "referneced_objects"))]
        let mut res = Array {
            arr_ptr: ptr,
            pd: PhantomData,
            lengths: [0; DIMENSIONS as usize],
        };
        #[cfg(feature = "referneced_objects")]
        let mut res = Array {
            handle: GCHandle::create_default(ptr.cast()),
            pd: PhantomData,
            lengths: Dim::zeroed(),
        };
        #[cfg(not(feature = "unsafe_arrays"))]
        {
            #[allow(clippy::cast_sign_loss)]
            let rank = res.get_class().get_rank() as usize;
            assert_eq!(rank, Dim::DIMENSIONS, "Array dimension mismatch!",);
            let source_class = res.to_object().get_class();
            let target_class = <Self as InteropClass>::get_mono_class();
            assert!(
                !(source_class.get_element_class() != target_class.get_element_class()),
                "tried to create array of type `{}` from object of type `{}`",
                &target_class.get_name(),
                &source_class.get_name()
            );
        }
        //get array size
        {
            let dim: Method<(i32,)> = Method::get_from_name(&Class::get_array(), "GetLength", 1)
                .expect("Array type does not have GetLength method, even toug it is impossible.");
            #[allow(
                clippy::cast_possible_wrap,
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss
            )]
            for i in 0..Dim::DIMENSIONS {
                let dim_obj = dim
                    .invoke(Some(res.to_object()), (i as i32,))
                    .expect("Got an exception while calling Array.GetLength")
                    .expect("Got null instead of int");
                let len_ref: &mut Dim::Lengths = &mut res.lengths;
                let len_ref: &mut [usize] = (len_ref).borrow_mut();
                len_ref[i] = dim_obj.unbox::<i32>() as usize;
            }
        }
        Some(res)
    }
    /// Converts [`Array`] to [`Object`]
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |self| &Array | array to cast to object|
    #[must_use]
    pub fn to_object(&self) -> Object {
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe { Object::from_ptr(self.get_ptr().cast()) }
            .expect("Could not create object from array!");
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Allocate new array in *domain* with size *DIMENSIONS* with elements of type *class*.
    /// # Example
    /// ```no_run
    /// # use wrapped_mono::*;
    /// # let domain = Domain::get_current().unwrap();
    /// # let int_managed_class = Class::get_int_32();
    /// let arr_len = 8;
    /// let arr = Array::<Dim1D,i32>::new(&domain,&[arr_len]);
    /// assert!(arr.len() == arr_len);
    /// ```
    ///
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |domain| &[`Domain`] | domain to create array in|
    /// |size|`&[usize;DIMENSIONS as usize]`| size of the array to create|
    #[must_use]
    pub fn new(domain: &Domain, size: &Dim::Lengths) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        let class = <T as InteropClass>::get_mono_class().get_array_class(Dim::DIMENSIONS as u32);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let arr = unsafe {
            Self::from_ptr(crate::binds::mono_array_new_full(
                domain.get_ptr(),
                class.get_ptr(),
                size as *const _ as *mut usize,
                null_mut(),
            ))
        }
        .expect("could not create a new array!");
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        arr
    }
    /// Function returning a copy of internal pointer to [`MonoArray`]
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |self| &Array | Rust representation of Array to get internal pointer to|
    #[must_use]
    pub fn get_ptr(&self) -> *mut crate::binds::MonoArray {
        #[cfg(not(feature = "referneced_objects"))]
        return self.arr_ptr;
        #[cfg(feature = "referneced_objects")]
        return self.handle.get_target().cast();
    }
    /// Clones managed array, **not** the reference to it.
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |self|&Array|Array to clone|
    #[must_use]
    pub fn clone_managed_array(&self) -> Self {
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe { Self::from_ptr(crate::binds::mono_array_clone(self.get_ptr())) }
            .expect("coud not create copy of an array!");
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    ///Returns class of this array
    #[must_use]
    pub fn get_class() -> Class {
        #[allow(clippy::cast_possible_truncation)]
        Class::get_array_class(
            &<T as InteropClass>::get_mono_class(),
            Dim::DIMENSIONS as u32,
        )
    }
    /// Returns n-dimensional length of this array.
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |self|&Array|Array to get size of|
    pub fn get_lenghts(&self) -> Dim::Lengths {
        self.lengths
    }
}
impl<Dim: DimensionTrait, T: InteropSend + InteropRecive + InteropClass> InteropRecive
    for Array<Dim, T>
where
    Dim::Lengths: std::ops::IndexMut<usize> + BorrowMut<[usize]> + Copy + Copy,
    <Dim::Lengths as std::ops::Index<usize>>::Output: BorrowMut<usize>,
    <<Dim as DimensionTrait>::Lengths as Index<usize>>::Output: Sized + Into<usize> + Copy,
{
    type SourceType = *mut crate::binds::MonoArray;
    // unless this function is abused, this argument should come from the mono runtime, so it should be always valid.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn get_rust_rep(arg: Self::SourceType) -> Self {
        use crate::exception::except_managed;
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let opt = unsafe { Self::from_ptr(arg) };
        let res = except_managed(
            opt,
            "Got null in an not nullable type. For nullable support use Option<Array>",
        );
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
}
impl<Dim: DimensionTrait, T: InteropSend + InteropRecive + InteropClass> InteropSend
    for Array<Dim, T>
where
    Dim::Lengths: std::ops::IndexMut<usize> + BorrowMut<[usize]> + Copy,
    <Dim::Lengths as std::ops::Index<usize>>::Output: BorrowMut<usize>,
    <<Dim as DimensionTrait>::Lengths as Index<usize>>::Output: Sized + Into<usize> + Copy,
{
    type TargetType = *mut crate::binds::MonoArray;
    fn get_mono_rep(arg: Self) -> Self::TargetType {
        arg.get_ptr()
    }
}
impl<Dim: DimensionTrait, T: InteropSend + InteropRecive + InteropClass> InteropClass
    for Array<Dim, T>
where
    Dim::Lengths: std::ops::IndexMut<usize> + BorrowMut<[usize]> + Copy,
    <Dim::Lengths as std::ops::Index<usize>>::Output: BorrowMut<usize>,
    <<Dim as DimensionTrait>::Lengths as Index<usize>>::Output: Sized + Into<usize> + Copy,
{
    fn get_mono_class() -> Class {
        Self::get_class()
    }
}
impl<Dim: DimensionTrait, T: InteropSend + InteropRecive + InteropClass> ObjectTrait
    for Array<Dim, T>
where
    Dim::Lengths: std::ops::IndexMut<usize> + BorrowMut<[usize]> + Copy,
    <Dim::Lengths as std::ops::Index<usize>>::Output: BorrowMut<usize>,
    <<Dim as DimensionTrait>::Lengths as Index<usize>>::Output: Sized + Into<usize> + Copy,
{
    fn hash(&self) -> i32 {
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let hash = unsafe { crate::binds::mono_object_hash(self.get_ptr().cast()) };
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        hash
    }
    fn get_domain(&self) -> crate::domain::Domain {
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let dom = unsafe {
            crate::domain::Domain::from_ptr(crate::binds::mono_object_get_domain(
                self.get_ptr().cast(),
            ))
        };
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        dom
    }
    fn get_size(&self) -> u32 {
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let size = unsafe { crate::binds::mono_object_get_size(self.get_ptr().cast()) };
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        size
    }
    fn reflection_get_token(&self) -> u32 {
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let token = unsafe { crate::binds::mono_reflection_get_token(self.get_ptr().cast()) };
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        token
    }
    fn get_class(&self) -> crate::class::Class {
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let class = unsafe {
            crate::class::Class::from_ptr(crate::binds::mono_object_get_class(
                self.get_ptr().cast(),
            ))
            .expect("Could not get class of an object")
        };
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        class
    }
    fn to_mstring(&self) -> Result<Option<MString>, Exception> {
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let mut exc: *mut crate::binds::MonoException = core::ptr::null_mut();
        let res = unsafe {
            MString::from_ptr(crate::binds::mono_object_to_string(
                self.get_ptr().cast(),
                std::ptr::addr_of_mut!(exc).cast::<*mut MonoObject>(),
            ))
        };
        let exc = unsafe { Exception::from_ptr(exc) };
        let res = match exc {
            Some(e) => Err(e),
            None => Ok(res),
        };
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    fn cast_to_object(&self) -> Object {
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let obj = unsafe { Object::from_ptr(self.get_ptr().cast()) }.unwrap(); //impossible. If array exists, then object exists too.
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        obj
    }
    /// Cast [`Object`] to [`Array`]. Returns [`None`] if cast failed.
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |object| &Object | object to cast from |
    fn cast_from_object(object: &Object) -> Option<Array<Dim, T>> {
        let source_class = object.get_class();
        let target_class = <Self as InteropClass>::get_mono_class();
        if source_class.get_element_class() != target_class.get_element_class() {
            return None;
        }
        unsafe { Self::from_ptr(object.get_ptr().cast()) }
    }
}
impl<Dim: DimensionTrait, T: InteropSend + InteropRecive + InteropClass> InteropRecive
    for Option<Array<Dim, T>>
where
    Dim::Lengths: std::ops::IndexMut<usize> + BorrowMut<[usize]> + Copy,
    <Dim::Lengths as std::ops::Index<usize>>::Output: BorrowMut<usize>,
    <<Dim as DimensionTrait>::Lengths as Index<usize>>::Output: Sized + Into<usize> + Copy,
{
    type SourceType = *mut crate::binds::MonoArray;
    // unless this function is abused, this argument should come from the mono runtime, so it should be always valid.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn get_rust_rep(arg: Self::SourceType) -> Self {
        unsafe { Array::<Dim, T>::from_ptr(arg) }
    }
}
impl<Dim: DimensionTrait, T: InteropSend + InteropRecive + InteropClass> InteropSend
    for Option<Array<Dim, T>>
where
    Dim::Lengths: std::ops::IndexMut<usize> + BorrowMut<[usize]> + Copy,
    <Dim::Lengths as std::ops::Index<usize>>::Output: BorrowMut<usize>,
    <<Dim as DimensionTrait>::Lengths as Index<usize>>::Output: Sized + Into<usize> + Copy,
{
    type TargetType = *mut crate::binds::MonoArray;
    fn get_mono_rep(arg: Self) -> Self::TargetType {
        match arg {
            Some(arg) => arg.get_ptr(),
            None => null_mut(),
        }
    }
}
impl<Dim: DimensionTrait, T: InteropSend + InteropRecive + InteropClass> Clone for Array<Dim, T>
where
    Dim::Lengths: std::ops::IndexMut<usize> + BorrowMut<[usize]> + Copy,
    <Dim::Lengths as std::ops::Index<usize>>::Output: BorrowMut<usize>,
    <<Dim as DimensionTrait>::Lengths as Index<usize>>::Output: Sized + Into<usize> + Copy,
{
    fn clone(&self) -> Self {
        unsafe { Self::from_ptr(self.get_ptr()).unwrap() } //If object exists then it can't be null
    }
}
impl<Dim: DimensionTrait, T: InteropSend + InteropRecive + InteropClass, O: ObjectTrait>
    PartialEq<O> for Array<Dim, T>
where
    Dim::Lengths: std::ops::IndexMut<usize> + BorrowMut<[usize]> + Copy,
    <Dim::Lengths as std::ops::Index<usize>>::Output: BorrowMut<usize>,
    <<Dim as DimensionTrait>::Lengths as Index<usize>>::Output: Sized + Into<usize> + Copy,
{
    fn eq(&self, other: &O) -> bool {
        self.get_ptr().cast() == other.cast_to_object().get_ptr()
    }
}
use crate::dimensions::Dim1D;
impl<T: InteropSend + InteropRecive + InteropClass + Clone> From<&[T]> for Array<Dim1D, T> {
    fn from(src: &[T]) -> Self {
        let size = src.len();
        let dom = Domain::get_current().expect("Can't create arrays before JIT starts!");
        let mut res: Array<Dim1D, T> = Array::new(&dom, &[size]);
        for (i, src) in src.iter().enumerate() {
            res.set([i], src.clone());
        }
        res
    }
}
