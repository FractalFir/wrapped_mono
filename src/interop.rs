use std::os::raw::c_void;

use crate::object::ObjectTrait;
/// Trait specifying how to convert a type when transferring it between managed and unmanaged code. It specifies how to convert
/// `SourceType` used by `MonoRuntime` to type implementing this trait.
pub trait InteropReceive {
    ///Source type used by `MonoRuntime` when calling functions exposed by `add_internal_call`, or getting a value back from a method, that can be converted to a rust type.
    type SourceType: Copy;
    ///Function converting [`Self::SourceType`] to type implementing [`InteropReceive`] trait.
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self;
}
/// Trait specifying how to convert a type when transferring it between managed and unmanaged code. It specifies how to convert type implementing this trait
/// to `TargetType` used by `MonoRuntime`.
/// # Safety
/// This type has the appropriate layout on the mono side.
pub unsafe trait InteropSend: Sized {
    ///Function converting type implementing [`InteropReceive`] trait to type that should be returned to `MonoRuntime`.
    fn get_ffi_ptr(&mut self) -> *mut c_void {
        std::ptr::addr_of_mut!(*self) as *mut c_void
    }
    fn is_class_type() -> bool {
        false
    }
    /// Internal function used for returning values from Rust callbacks to Mono functions
    unsafe fn return_value_to_mono(mut self) -> Self {
        if Self::is_class_type() {
            assert_eq!(std::mem::size_of::<Self>(), std::mem::size_of::<*mut ()>());
            let ptr = self.get_ffi_ptr();
            (&ptr as *const *mut c_void as *const Self).read()
        } else {
            self
        }
    }
}
impl InteropReceive for String {
    type SourceType = *mut crate::binds::MonoString;
    // unless this function is abused, this argument should come from the mono runtime, so it should be always valid.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        use std::ffi::CString;
        let cstr = unsafe { CString::from_raw(crate::binds::mono_string_to_utf8(mono_arg)) };
        let res = cstr
            .to_str()
            .expect("Could not convert MonoString to String!")
            .to_owned();
        unsafe { crate::binds::mono_free(cstr.into_raw().cast::<std::ffi::c_void>()) };
        res
    }
}
impl InteropReceive for usize {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropReceive for isize {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropReceive for i8 {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropReceive for i16 {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropReceive for i32 {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropReceive for i64 {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropReceive for u8 {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropReceive for u16 {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropReceive for u32 {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropReceive for u64 {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropReceive for f32 {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropReceive for f64 {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl<T> InteropReceive for *mut T {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl<T> InteropReceive for *const T {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropReceive for bool {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropReceive for char {
    type SourceType = u16;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        let src = [mono_arg];
        Self::decode_utf16(src).next().unwrap().unwrap()
    }
}
//return section
unsafe impl InteropSend for i8 {}
unsafe impl InteropSend for i16 {}
unsafe impl InteropSend for i32 {}
unsafe impl InteropSend for i64 {}
unsafe impl InteropSend for u8 {}
unsafe impl InteropSend for u16 {}
unsafe impl InteropSend for u32 {}
unsafe impl InteropSend for u64 {}
unsafe impl InteropSend for f32 {}
unsafe impl InteropSend for f64 {}
unsafe impl InteropSend for usize {}
unsafe impl InteropSend for isize {}
unsafe impl<T> InteropSend for *mut T {}
unsafe impl<T> InteropSend for *const T {}

unsafe impl InteropSend for bool {}
unsafe impl InteropSend for () {}
unsafe impl InteropSend for &str {
    fn get_ffi_ptr(&mut self) -> *mut c_void {
        use crate::MString;
        MString::new(
            &crate::Domain::get_current()
                .expect("Could not get current domain when sending strings to mono runtime!"),
            self,
        )
        .get_ffi_ptr()
    }
    fn is_class_type() -> bool {
        true
    }
}
unsafe impl InteropSend for String {
    fn get_ffi_ptr(&mut self) -> *mut c_void {
        self.as_str().get_ffi_ptr()
    }
    fn is_class_type() -> bool {
        true
    }
}

use crate::class::Class;
/// Trait allowing for boxing and unboxing type from objects
/// # Safety
/// Managed type returned by `get_mono_class` of `InteropClass` **must** be boxable, otherwise a crash may occur.
pub trait InteropBox
where
    Self: InteropReceive + InteropSend + InteropClass,
{
}
impl<T: ObjectTrait> InteropReceive for T {
    type SourceType = *mut crate::binds::MonoObject;
    fn get_rust_rep(src: Self::SourceType) -> T {
        if src.is_null() {
            panic!("Received null on non-nullable");
        }
        unsafe {
            match T::from_ptr(src) {
                Some(res) => res,
                None => {
                    let src_type = crate::Object::from_ptr_unchecked(src).get_class();
                    let target_type = Self::get_mono_class();
                    panic!("Can't assign from type {src_type:?} to target type {target_type:?}");
                }
            }
        }
    }
}
unsafe impl<T: ObjectTrait> InteropSend for T {
    fn get_ffi_ptr(&mut self) -> *mut c_void {
        self.get_ptr() as *mut c_void
    }
    fn is_class_type() -> bool {
        true
    }
}
unsafe impl<T: ObjectTrait> InteropSend for Option<T> {
    fn get_ffi_ptr(&mut self) -> *mut c_void {
        match self {
            Some(src) => src.get_ffi_ptr(),
            None => std::ptr::null_mut(),
        }
    }
    fn is_class_type() -> bool {
        true
    }
}
impl<T: ObjectTrait> InteropReceive for Option<T> {
    type SourceType = *mut crate::binds::MonoObject;
    fn get_rust_rep(src: Self::SourceType) -> Option<T> {
        if src.is_null() {
            return None;
        }
        unsafe {
            match T::from_ptr(src) {
                Some(res) => Some(res),
                None => {
                    let src_type = crate::Object::from_ptr_unchecked(src).get_class();
                    let target_type = T::get_mono_class();
                    panic!("Can't assign from type {src_type:?} to target type {target_type:?}");
                }
            }
        }
    }
}
/// Trait allowing managed class representing this type to be got.
/// Type of value `Self::InteropSend::TargetType` must match managed type represented by [`Class`] returned by `get_mono_class`.
pub trait InteropClass {
    fn get_mono_class() -> Class;
}
impl InteropClass for i8 {
    fn get_mono_class() -> Class {
        Class::get_sbyte()
    }
}
impl InteropClass for i16 {
    fn get_mono_class() -> Class {
        Class::get_int_16()
    }
}
impl InteropClass for i32 {
    fn get_mono_class() -> Class {
        Class::get_int_32()
    }
}
impl InteropClass for i64 {
    fn get_mono_class() -> Class {
        Class::get_int_64()
    }
}
impl InteropClass for u8 {
    fn get_mono_class() -> Class {
        Class::get_byte()
    }
}
impl InteropClass for u16 {
    fn get_mono_class() -> Class {
        Class::get_uint_16()
    }
}
impl InteropClass for u32 {
    fn get_mono_class() -> Class {
        Class::get_uint_32()
    }
}
impl InteropClass for u64 {
    fn get_mono_class() -> Class {
        Class::get_uint_64()
    }
}
impl InteropClass for f32 {
    fn get_mono_class() -> Class {
        Class::get_single()
    }
}
impl InteropClass for f64 {
    fn get_mono_class() -> Class {
        Class::get_double()
    }
}
impl InteropClass for isize {
    fn get_mono_class() -> Class {
        Class::get_int_ptr()
    }
}
impl InteropClass for usize {
    fn get_mono_class() -> Class {
        Class::get_uint_ptr()
    }
}
impl<T> InteropClass for *mut T {
    fn get_mono_class() -> Class {
        Class::get_uint_ptr()
    }
}
impl<T> InteropClass for *const T {
    fn get_mono_class() -> Class {
        Class::get_uint_ptr()
    }
}
impl InteropClass for char {
    fn get_mono_class() -> Class {
        Class::get_char()
    }
}
impl InteropClass for bool {
    fn get_mono_class() -> Class {
        Class::get_boolean()
    }
}
impl InteropClass for String {
    fn get_mono_class() -> Class {
        Class::get_string()
    }
}
impl InteropBox for i8 {}
impl InteropBox for i16 {}
impl InteropBox for i32 {}
impl InteropBox for i64 {}
impl InteropBox for u8 {}
impl InteropBox for u16 {}
impl InteropBox for u32 {}
impl InteropBox for u64 {}
impl InteropBox for f32 {}
impl InteropBox for f64 {}
impl InteropBox for isize {}
impl InteropBox for usize {}
impl InteropBox for bool {}
