use crate::object::ObjectTrait;
/// Trait specifying how to convert a type when transferring it between managed and unmanaged code. It specifies how to convert
/// `SourceType` used by `MonoRuntime` to type implementing this trait.
pub trait InteropRecive {
    ///Souce type used by `MonoRuntime` when calling functions exposed by `add_internal_call`, or getting a value back from a method, that can be converted to a rust type.
    type SourceType: Copy;
    ///Function converting [`Self::SourceType`] to type implementing [`InteropRecive`] trait.
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self;
}
/// Trait specifying how to convert a type when transferring it between managed and unmanaged code. It specifies how to convert type implementing this trait
/// to `TargetType` used by `MonoRuntime`.
pub trait InteropSend {
    ///Type used by `MonoRuntime`, that type implementing [`InteropSend`] trait should be converted to when returning it to `MonoRuntime`.
    type TargetType: Copy;
    ///Function converting type implementing [`InteropRecive`] trait to type that should be returned to `MonoRuntime`.
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType;
}
impl InteropRecive for String {
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
impl InteropRecive for usize {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropRecive for isize {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropRecive for i8 {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropRecive for i16 {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropRecive for i32 {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropRecive for i64 {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropRecive for u8 {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropRecive for u16 {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropRecive for u32 {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropRecive for u64 {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropRecive for f32 {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropRecive for f64 {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl<T> InteropRecive for *mut T {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl<T> InteropRecive for *const T {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropRecive for bool {
    type SourceType = Self;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        mono_arg
    }
}
impl InteropRecive for char {
    type SourceType = u16;
    fn get_rust_rep(mono_arg: Self::SourceType) -> Self {
        let src = [mono_arg];
        Self::decode_utf16(src).next().unwrap().unwrap()
    }
}
//return section
impl InteropSend for i8 {
    type TargetType = Self;
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        rust_arg
    }
}
impl InteropSend for i16 {
    type TargetType = Self;
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        rust_arg
    }
}
impl InteropSend for i32 {
    type TargetType = Self;
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        rust_arg
    }
}
impl InteropSend for i64 {
    type TargetType = Self;
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        rust_arg
    }
}
impl InteropSend for u8 {
    type TargetType = Self;
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        rust_arg
    }
}
impl InteropSend for u16 {
    type TargetType = Self;
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        rust_arg
    }
}
impl InteropSend for u32 {
    type TargetType = Self;
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        rust_arg
    }
}
impl InteropSend for u64 {
    type TargetType = Self;
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        rust_arg
    }
}
impl InteropSend for f32 {
    type TargetType = Self;
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        rust_arg
    }
}
impl InteropSend for f64 {
    type TargetType = Self;
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        rust_arg
    }
}
impl InteropSend for usize {
    type TargetType = Self;
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        rust_arg
    }
}
impl InteropSend for isize {
    type TargetType = Self;
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        rust_arg
    }
}
impl<T> InteropSend for *mut T {
    type TargetType = Self;
    fn get_mono_rep(mono_arg: Self::TargetType) -> Self {
        mono_arg
    }
}
impl<T> InteropSend for *const T {
    type TargetType = Self;
    fn get_mono_rep(mono_arg: Self::TargetType) -> Self {
        mono_arg
    }
}
impl InteropSend for char {
    type TargetType = u16;
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        let mut tmp = [0; 2];
        rust_arg.encode_utf16(&mut tmp);
        tmp[0]
    }
}
impl InteropSend for bool {
    type TargetType = Self;
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        rust_arg
    }
}
impl InteropSend for () {
    type TargetType = Self;
    fn get_mono_rep(_: Self) -> Self::TargetType {}
}
impl InteropSend for String {
    type TargetType = *mut crate::binds::MonoString;
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        use crate::MString;
        MString::new(
            &crate::Domain::get_current()
                .expect("Could not get current domain when sending strings to mono runtime!"),
            &rust_arg,
        )
        .get_ptr()
        .cast()
    }
}
use crate::class::Class;
/// Trait allowing for boxing and unboxing type from objects
/// # Safety
/// Managed type returned by `get_mono_class` of `InteropClass` **must** be boxable, otherwise a crash may occur.
pub trait InteropBox
where
    Self: InteropRecive + InteropSend + InteropClass,
{
}
impl<T: ObjectTrait> InteropRecive for T {
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
impl<T: ObjectTrait> InteropSend for T {
    type TargetType = *mut crate::binds::MonoObject;
    fn get_mono_rep(src: Self) -> Self::TargetType {
        src.get_ptr()
    }
}
impl<T: ObjectTrait> InteropSend for Option<T> {
    type TargetType = *mut crate::binds::MonoObject;
    fn get_mono_rep(src: Self) -> Self::TargetType {
        match src {
            Some(src) => src.get_ptr(),
            None => std::ptr::null_mut(),
        }
    }
}
impl<T: ObjectTrait> InteropRecive for Option<T> {
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
impl InteropBox for char {}

//use crate::tupleutilis::*;
//Conversion of a tuple from one format to another
impl<A: InteropSend> InteropSend for (A,) {
    type TargetType = (A::TargetType,);
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        (A::get_mono_rep(rust_arg.0),)
    }
}
impl<A: InteropSend, B: InteropSend> InteropSend for (A, B) {
    type TargetType = (A::TargetType, B::TargetType);
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        (A::get_mono_rep(rust_arg.0), B::get_mono_rep(rust_arg.1))
    }
}
impl<A: InteropSend, B: InteropSend, C: InteropSend> InteropSend for (A, B, C) {
    type TargetType = (A::TargetType, B::TargetType, C::TargetType);
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        (
            A::get_mono_rep(rust_arg.0),
            B::get_mono_rep(rust_arg.1),
            C::get_mono_rep(rust_arg.2),
        )
    }
}
impl<A: InteropSend, B: InteropSend, C: InteropSend, D: InteropSend> InteropSend for (A, B, C, D) {
    type TargetType = (A::TargetType, B::TargetType, C::TargetType, D::TargetType);
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        (
            A::get_mono_rep(rust_arg.0),
            B::get_mono_rep(rust_arg.1),
            C::get_mono_rep(rust_arg.2),
            D::get_mono_rep(rust_arg.3),
        )
    }
}
impl<A: InteropSend, B: InteropSend, C: InteropSend, D: InteropSend, E: InteropSend> InteropSend
    for (A, B, C, D, E)
{
    type TargetType = (
        A::TargetType,
        B::TargetType,
        C::TargetType,
        D::TargetType,
        E::TargetType,
    );
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        (
            A::get_mono_rep(rust_arg.0),
            B::get_mono_rep(rust_arg.1),
            C::get_mono_rep(rust_arg.2),
            D::get_mono_rep(rust_arg.3),
            E::get_mono_rep(rust_arg.4),
        )
    }
}
impl<
        A: InteropSend,
        B: InteropSend,
        C: InteropSend,
        D: InteropSend,
        E: InteropSend,
        F: InteropSend,
    > InteropSend for (A, B, C, D, E, F)
{
    type TargetType = (
        A::TargetType,
        B::TargetType,
        C::TargetType,
        D::TargetType,
        E::TargetType,
        F::TargetType,
    );
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        (
            A::get_mono_rep(rust_arg.0),
            B::get_mono_rep(rust_arg.1),
            C::get_mono_rep(rust_arg.2),
            D::get_mono_rep(rust_arg.3),
            E::get_mono_rep(rust_arg.4),
            F::get_mono_rep(rust_arg.5),
        )
    }
}
impl<
        A: InteropSend,
        B: InteropSend,
        C: InteropSend,
        D: InteropSend,
        E: InteropSend,
        F: InteropSend,
        G: InteropSend,
    > InteropSend for (A, B, C, D, E, F, G)
{
    type TargetType = (
        A::TargetType,
        B::TargetType,
        C::TargetType,
        D::TargetType,
        E::TargetType,
        F::TargetType,
        G::TargetType,
    );
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        (
            A::get_mono_rep(rust_arg.0),
            B::get_mono_rep(rust_arg.1),
            C::get_mono_rep(rust_arg.2),
            D::get_mono_rep(rust_arg.3),
            E::get_mono_rep(rust_arg.4),
            F::get_mono_rep(rust_arg.5),
            G::get_mono_rep(rust_arg.6),
        )
    }
}
impl<
        A: InteropSend,
        B: InteropSend,
        C: InteropSend,
        D: InteropSend,
        E: InteropSend,
        F: InteropSend,
        G: InteropSend,
        H: InteropSend,
    > InteropSend for (A, B, C, D, E, F, G, H)
{
    type TargetType = (
        A::TargetType,
        B::TargetType,
        C::TargetType,
        D::TargetType,
        E::TargetType,
        F::TargetType,
        G::TargetType,
        H::TargetType,
    );
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        (
            A::get_mono_rep(rust_arg.0),
            B::get_mono_rep(rust_arg.1),
            C::get_mono_rep(rust_arg.2),
            D::get_mono_rep(rust_arg.3),
            E::get_mono_rep(rust_arg.4),
            F::get_mono_rep(rust_arg.5),
            G::get_mono_rep(rust_arg.6),
            H::get_mono_rep(rust_arg.7),
        )
    }
}
impl<
        A: InteropSend,
        B: InteropSend,
        C: InteropSend,
        D: InteropSend,
        E: InteropSend,
        F: InteropSend,
        G: InteropSend,
        H: InteropSend,
        I: InteropSend,
    > InteropSend for (A, B, C, D, E, F, G, H, I)
{
    type TargetType = (
        A::TargetType,
        B::TargetType,
        C::TargetType,
        D::TargetType,
        E::TargetType,
        F::TargetType,
        G::TargetType,
        H::TargetType,
        I::TargetType,
    );
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        (
            A::get_mono_rep(rust_arg.0),
            B::get_mono_rep(rust_arg.1),
            C::get_mono_rep(rust_arg.2),
            D::get_mono_rep(rust_arg.3),
            E::get_mono_rep(rust_arg.4),
            F::get_mono_rep(rust_arg.5),
            G::get_mono_rep(rust_arg.6),
            H::get_mono_rep(rust_arg.7),
            I::get_mono_rep(rust_arg.8),
        )
    }
}
impl<
        A: InteropSend,
        B: InteropSend,
        C: InteropSend,
        D: InteropSend,
        E: InteropSend,
        F: InteropSend,
        G: InteropSend,
        H: InteropSend,
        I: InteropSend,
        J: InteropSend,
    > InteropSend for (A, B, C, D, E, F, G, H, I, J)
{
    type TargetType = (
        A::TargetType,
        B::TargetType,
        C::TargetType,
        D::TargetType,
        E::TargetType,
        F::TargetType,
        G::TargetType,
        H::TargetType,
        I::TargetType,
        J::TargetType,
    );
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        (
            A::get_mono_rep(rust_arg.0),
            B::get_mono_rep(rust_arg.1),
            C::get_mono_rep(rust_arg.2),
            D::get_mono_rep(rust_arg.3),
            E::get_mono_rep(rust_arg.4),
            F::get_mono_rep(rust_arg.5),
            G::get_mono_rep(rust_arg.6),
            H::get_mono_rep(rust_arg.7),
            I::get_mono_rep(rust_arg.8),
            J::get_mono_rep(rust_arg.9),
        )
    }
}
impl<
        A: InteropSend,
        B: InteropSend,
        C: InteropSend,
        D: InteropSend,
        E: InteropSend,
        F: InteropSend,
        G: InteropSend,
        H: InteropSend,
        I: InteropSend,
        J: InteropSend,
        K: InteropSend,
    > InteropSend for (A, B, C, D, E, F, G, H, I, J, K)
{
    type TargetType = (
        A::TargetType,
        B::TargetType,
        C::TargetType,
        D::TargetType,
        E::TargetType,
        F::TargetType,
        G::TargetType,
        H::TargetType,
        I::TargetType,
        J::TargetType,
        K::TargetType,
    );
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        (
            A::get_mono_rep(rust_arg.0),
            B::get_mono_rep(rust_arg.1),
            C::get_mono_rep(rust_arg.2),
            D::get_mono_rep(rust_arg.3),
            E::get_mono_rep(rust_arg.4),
            F::get_mono_rep(rust_arg.5),
            G::get_mono_rep(rust_arg.6),
            H::get_mono_rep(rust_arg.7),
            I::get_mono_rep(rust_arg.8),
            J::get_mono_rep(rust_arg.9),
            K::get_mono_rep(rust_arg.10),
        )
    }
}
impl<
        A: InteropSend,
        B: InteropSend,
        C: InteropSend,
        D: InteropSend,
        E: InteropSend,
        F: InteropSend,
        G: InteropSend,
        H: InteropSend,
        I: InteropSend,
        J: InteropSend,
        K: InteropSend,
        L: InteropSend,
    > InteropSend for (A, B, C, D, E, F, G, H, I, J, K, L)
{
    type TargetType = (
        A::TargetType,
        B::TargetType,
        C::TargetType,
        D::TargetType,
        E::TargetType,
        F::TargetType,
        G::TargetType,
        H::TargetType,
        I::TargetType,
        J::TargetType,
        K::TargetType,
        L::TargetType,
    );
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        (
            A::get_mono_rep(rust_arg.0),
            B::get_mono_rep(rust_arg.1),
            C::get_mono_rep(rust_arg.2),
            D::get_mono_rep(rust_arg.3),
            E::get_mono_rep(rust_arg.4),
            F::get_mono_rep(rust_arg.5),
            G::get_mono_rep(rust_arg.6),
            H::get_mono_rep(rust_arg.7),
            I::get_mono_rep(rust_arg.8),
            J::get_mono_rep(rust_arg.9),
            K::get_mono_rep(rust_arg.10),
            L::get_mono_rep(rust_arg.11),
        )
    }
}
impl<
        A: InteropSend,
        B: InteropSend,
        C: InteropSend,
        D: InteropSend,
        E: InteropSend,
        F: InteropSend,
        G: InteropSend,
        H: InteropSend,
        I: InteropSend,
        J: InteropSend,
        K: InteropSend,
        L: InteropSend,
        M: InteropSend,
    > InteropSend for (A, B, C, D, E, F, G, H, I, J, K, L, M)
{
    type TargetType = (
        A::TargetType,
        B::TargetType,
        C::TargetType,
        D::TargetType,
        E::TargetType,
        F::TargetType,
        G::TargetType,
        H::TargetType,
        I::TargetType,
        J::TargetType,
        K::TargetType,
        L::TargetType,
        M::TargetType,
    );
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        (
            A::get_mono_rep(rust_arg.0),
            B::get_mono_rep(rust_arg.1),
            C::get_mono_rep(rust_arg.2),
            D::get_mono_rep(rust_arg.3),
            E::get_mono_rep(rust_arg.4),
            F::get_mono_rep(rust_arg.5),
            G::get_mono_rep(rust_arg.6),
            H::get_mono_rep(rust_arg.7),
            I::get_mono_rep(rust_arg.8),
            J::get_mono_rep(rust_arg.9),
            K::get_mono_rep(rust_arg.10),
            L::get_mono_rep(rust_arg.11),
            M::get_mono_rep(rust_arg.12),
        )
    }
}
impl<
        A: InteropSend,
        B: InteropSend,
        C: InteropSend,
        D: InteropSend,
        E: InteropSend,
        F: InteropSend,
        G: InteropSend,
        H: InteropSend,
        I: InteropSend,
        J: InteropSend,
        K: InteropSend,
        L: InteropSend,
        M: InteropSend,
        N: InteropSend,
    > InteropSend for (A, B, C, D, E, F, G, H, I, J, K, L, M, N)
{
    type TargetType = (
        A::TargetType,
        B::TargetType,
        C::TargetType,
        D::TargetType,
        E::TargetType,
        F::TargetType,
        G::TargetType,
        H::TargetType,
        I::TargetType,
        J::TargetType,
        K::TargetType,
        L::TargetType,
        M::TargetType,
        N::TargetType,
    );
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        (
            A::get_mono_rep(rust_arg.0),
            B::get_mono_rep(rust_arg.1),
            C::get_mono_rep(rust_arg.2),
            D::get_mono_rep(rust_arg.3),
            E::get_mono_rep(rust_arg.4),
            F::get_mono_rep(rust_arg.5),
            G::get_mono_rep(rust_arg.6),
            H::get_mono_rep(rust_arg.7),
            I::get_mono_rep(rust_arg.8),
            J::get_mono_rep(rust_arg.9),
            K::get_mono_rep(rust_arg.10),
            L::get_mono_rep(rust_arg.11),
            M::get_mono_rep(rust_arg.12),
            N::get_mono_rep(rust_arg.13),
        )
    }
}
impl<
        A: InteropSend,
        B: InteropSend,
        C: InteropSend,
        D: InteropSend,
        E: InteropSend,
        F: InteropSend,
        G: InteropSend,
        H: InteropSend,
        I: InteropSend,
        J: InteropSend,
        K: InteropSend,
        L: InteropSend,
        M: InteropSend,
        N: InteropSend,
        O: InteropSend,
    > InteropSend for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O)
{
    type TargetType = (
        A::TargetType,
        B::TargetType,
        C::TargetType,
        D::TargetType,
        E::TargetType,
        F::TargetType,
        G::TargetType,
        H::TargetType,
        I::TargetType,
        J::TargetType,
        K::TargetType,
        L::TargetType,
        M::TargetType,
        N::TargetType,
        O::TargetType,
    );
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        (
            A::get_mono_rep(rust_arg.0),
            B::get_mono_rep(rust_arg.1),
            C::get_mono_rep(rust_arg.2),
            D::get_mono_rep(rust_arg.3),
            E::get_mono_rep(rust_arg.4),
            F::get_mono_rep(rust_arg.5),
            G::get_mono_rep(rust_arg.6),
            H::get_mono_rep(rust_arg.7),
            I::get_mono_rep(rust_arg.8),
            J::get_mono_rep(rust_arg.9),
            K::get_mono_rep(rust_arg.10),
            L::get_mono_rep(rust_arg.11),
            M::get_mono_rep(rust_arg.12),
            N::get_mono_rep(rust_arg.13),
            O::get_mono_rep(rust_arg.14),
        )
    }
}
impl<
        A: InteropSend,
        B: InteropSend,
        C: InteropSend,
        D: InteropSend,
        E: InteropSend,
        F: InteropSend,
        G: InteropSend,
        H: InteropSend,
        I: InteropSend,
        J: InteropSend,
        K: InteropSend,
        L: InteropSend,
        M: InteropSend,
        N: InteropSend,
        O: InteropSend,
        P: InteropSend,
    > InteropSend for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)
{
    type TargetType = (
        A::TargetType,
        B::TargetType,
        C::TargetType,
        D::TargetType,
        E::TargetType,
        F::TargetType,
        G::TargetType,
        H::TargetType,
        I::TargetType,
        J::TargetType,
        K::TargetType,
        L::TargetType,
        M::TargetType,
        N::TargetType,
        O::TargetType,
        P::TargetType,
    );
    fn get_mono_rep(rust_arg: Self) -> Self::TargetType {
        (
            A::get_mono_rep(rust_arg.0),
            B::get_mono_rep(rust_arg.1),
            C::get_mono_rep(rust_arg.2),
            D::get_mono_rep(rust_arg.3),
            E::get_mono_rep(rust_arg.4),
            F::get_mono_rep(rust_arg.5),
            G::get_mono_rep(rust_arg.6),
            H::get_mono_rep(rust_arg.7),
            I::get_mono_rep(rust_arg.8),
            J::get_mono_rep(rust_arg.9),
            K::get_mono_rep(rust_arg.10),
            L::get_mono_rep(rust_arg.11),
            M::get_mono_rep(rust_arg.12),
            N::get_mono_rep(rust_arg.13),
            O::get_mono_rep(rust_arg.14),
            P::get_mono_rep(rust_arg.15),
        )
    }
}
