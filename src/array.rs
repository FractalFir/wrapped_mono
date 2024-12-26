use crate::binds::MonoObject;
use crate::gc::{gc_unsafe_enter, gc_unsafe_exit, GCHandle};
use crate::interop::{InteropClass, InteropRecive, InteropSend};
use crate::{dimensions::DimensionTrait, domain::Domain, Class, Object, ObjectTrait};
use core::marker::PhantomData;
use core::ptr::null_mut;
use std::borrow::{Borrow, BorrowMut};
use std::ops::Index;
// Documentation finished.
/// Safe, rust representation of `MonoArray` (a reference to a managed array).
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
    arr_ptr: *mut crate::binds::MonoArray,
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
                self.get_ptr().cast(),
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
    pub fn set(&mut self, indices: Dim::Lengths, mut value: T)
    where
        T: InteropSend,
    {
        let index = self.get_index(indices);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let ptr = unsafe {
            #[allow(clippy::cast_possible_truncation)]
            #[allow(clippy::cast_possible_wrap)]
            crate::binds::mono_array_addr_with_size(
                self.get_ptr().cast(),
                std::mem::size_of::<T>() as i32,
                index,
            )
        };
        if T::is_class_type() {
            unsafe { (*ptr.cast()) = value.get_ffi_ptr() };
        } else {
            unsafe { (*ptr.cast()) = value };
        }

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
        let len = unsafe { crate::binds::mono_array_length(self.get_ptr().cast()) };
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
            Self::from_ptr(
                crate::binds::mono_array_new_full(
                    domain.get_ptr(),
                    class.get_ptr(),
                    size as *const _ as *mut usize,
                    null_mut(),
                )
                .cast(),
            )
        }
        .expect("could not create a new array!");
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        arr
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
        let res =
            unsafe { Self::from_ptr(crate::binds::mono_array_clone(self.get_ptr().cast()).cast()) }
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
    #[must_use]
    fn get_ptr(&self) -> *mut crate::binds::MonoObject {
        #[cfg(not(feature = "referneced_objects"))]
        return self.arr_ptr.cast();
        #[cfg(feature = "referneced_objects")]
        return self.handle.get_target();
    }
    #[must_use]
    unsafe fn from_ptr_unchecked(ptr: *mut MonoObject) -> Self {
        use crate::Method;
        #[cfg(not(feature = "referneced_objects"))]
        let mut res = Self {
            arr_ptr: ptr.cast(),
            pd: PhantomData,
            lengths: Dim::zeroed(),
        };
        #[cfg(feature = "referneced_objects")]
        let mut res = Self {
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
        res
    }
}
impl<Dim: DimensionTrait, T: InteropSend + InteropRecive + InteropClass> Clone for Array<Dim, T>
where
    Dim::Lengths: std::ops::IndexMut<usize> + BorrowMut<[usize]> + Copy,
    <Dim::Lengths as std::ops::Index<usize>>::Output: BorrowMut<usize>,
    <<Dim as DimensionTrait>::Lengths as Index<usize>>::Output: Sized + Into<usize> + Copy,
{
    fn clone(&self) -> Self {
        unsafe { Self::from_ptr(self.get_ptr().cast()).unwrap() } //If object exists then it can't be null
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
        self.get_ptr() == other.get_ptr().cast()
    }
}
use crate::dimensions::Dim1D;

impl<T: InteropSend + InteropRecive + InteropClass + Clone> From<&[T]> for Array<Dim1D, T> {
    fn from(src: &[T]) -> Self {
        let size = src.len();
        let dom = Domain::get_current().expect("Can't create arrays before JIT starts!");
        let mut res = Self::new(&dom, &[size]);
        for (i, src) in src.iter().enumerate() {
            res.set([i], src.clone());
        }
        res
    }
}
