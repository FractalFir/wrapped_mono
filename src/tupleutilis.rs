#![allow(clippy::many_single_char_names)]
use core::ffi::c_void;
//for argument processing
type VoidPtr = *mut c_void;
//Conversion of a tuple to pointers
pub trait TupleToFFIPtrs {
    type PTRS;
    fn get_ptrs(&mut self) -> Self::PTRS;
}
impl TupleToFFIPtrs for () {
    type PTRS = [*mut c_void; 0];
    fn get_ptrs(&mut self) -> Self::PTRS {
        []
    }
}
impl<A: InteropSend> TupleToFFIPtrs for (A,) {
    type PTRS = [*mut c_void; 1];
    fn get_ptrs(&mut self) -> Self::PTRS {
        [self.0.get_ffi_ptr()]
    }
}
impl<A: InteropSend, B: InteropSend> TupleToFFIPtrs for (A, B) {
    type PTRS = [*mut c_void; 2];
    fn get_ptrs(&mut self) -> Self::PTRS {
        [self.0.get_ffi_ptr(), self.1.get_ffi_ptr()]
    }
}
impl<A: InteropSend, B: InteropSend, C: InteropSend> TupleToFFIPtrs for (A, B, C) {
    type PTRS = [*mut c_void; 3];
    fn get_ptrs(&mut self) -> Self::PTRS {
        [
            self.0.get_ffi_ptr(),
            self.1.get_ffi_ptr(),
            self.2.get_ffi_ptr(),
        ]
    }
}
impl<A: InteropSend, B: InteropSend, C: InteropSend, D: InteropSend> TupleToFFIPtrs
    for (A, B, C, D)
{
    type PTRS = [*mut c_void; 4];
    fn get_ptrs(&mut self) -> Self::PTRS {
        [
            self.0.get_ffi_ptr(),
            self.1.get_ffi_ptr(),
            self.2.get_ffi_ptr(),
            self.3.get_ffi_ptr(),
        ]
    }
}
impl<A: InteropSend, B: InteropSend, C: InteropSend, D: InteropSend, E: InteropSend> TupleToFFIPtrs
    for (A, B, C, D, E)
{
    type PTRS = [*mut c_void; 5];
    fn get_ptrs(&mut self) -> Self::PTRS {
        [
            self.0.get_ffi_ptr(),
            self.1.get_ffi_ptr(),
            self.2.get_ffi_ptr(),
            self.3.get_ffi_ptr(),
            self.4.get_ffi_ptr(),
        ]
    }
}
impl<
        A: InteropSend,
        B: InteropSend,
        C: InteropSend,
        D: InteropSend,
        E: InteropSend,
        F: InteropSend,
    > TupleToFFIPtrs for (A, B, C, D, E, F)
{
    type PTRS = [*mut c_void; 6];
    fn get_ptrs(&mut self) -> Self::PTRS {
        [
            self.0.get_ffi_ptr(),
            self.1.get_ffi_ptr(),
            self.2.get_ffi_ptr(),
            self.3.get_ffi_ptr(),
            self.4.get_ffi_ptr(),
            self.5.get_ffi_ptr(),
        ]
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
    > TupleToFFIPtrs for (A, B, C, D, E, F, G)
{
    type PTRS = [*mut c_void; 7];
    fn get_ptrs(&mut self) -> Self::PTRS {
        [
            self.0.get_ffi_ptr(),
            self.1.get_ffi_ptr(),
            self.2.get_ffi_ptr(),
            self.3.get_ffi_ptr(),
            self.4.get_ffi_ptr(),
            self.5.get_ffi_ptr(),
            self.6.get_ffi_ptr(),
        ]
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
    > TupleToFFIPtrs for (A, B, C, D, E, F, G, H)
{
    type PTRS = [*mut c_void; 8];
    fn get_ptrs(&mut self) -> Self::PTRS {
        [
            self.0.get_ffi_ptr(),
            self.1.get_ffi_ptr(),
            self.2.get_ffi_ptr(),
            self.3.get_ffi_ptr(),
            self.4.get_ffi_ptr(),
            self.5.get_ffi_ptr(),
            self.6.get_ffi_ptr(),
            self.7.get_ffi_ptr(),
        ]
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
    > TupleToFFIPtrs for (A, B, C, D, E, F, G, H, I)
{
    type PTRS = [*mut c_void; 9];
    fn get_ptrs(&mut self) -> Self::PTRS {
        [
            self.0.get_ffi_ptr(),
            self.1.get_ffi_ptr(),
            self.2.get_ffi_ptr(),
            self.3.get_ffi_ptr(),
            self.4.get_ffi_ptr(),
            self.5.get_ffi_ptr(),
            self.6.get_ffi_ptr(),
            self.7.get_ffi_ptr(),
            self.8.get_ffi_ptr(),
        ]
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
    > TupleToFFIPtrs for (A, B, C, D, E, F, G, H, I, J)
{
    type PTRS = [*mut c_void; 10];
    fn get_ptrs(&mut self) -> Self::PTRS {
        [
            self.0.get_ffi_ptr(),
            self.1.get_ffi_ptr(),
            self.2.get_ffi_ptr(),
            self.3.get_ffi_ptr(),
            self.4.get_ffi_ptr(),
            self.5.get_ffi_ptr(),
            self.6.get_ffi_ptr(),
            self.7.get_ffi_ptr(),
            self.8.get_ffi_ptr(),
            self.9.get_ffi_ptr(),
        ]
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
    > TupleToFFIPtrs for (A, B, C, D, E, F, G, H, I, J, K)
{
    type PTRS = [*mut c_void; 11];
    fn get_ptrs(&mut self) -> Self::PTRS {
        [
            self.0.get_ffi_ptr(),
            self.1.get_ffi_ptr(),
            self.2.get_ffi_ptr(),
            self.3.get_ffi_ptr(),
            self.4.get_ffi_ptr(),
            self.5.get_ffi_ptr(),
            self.6.get_ffi_ptr(),
            self.7.get_ffi_ptr(),
            self.8.get_ffi_ptr(),
            self.9.get_ffi_ptr(),
            self.10.get_ffi_ptr(),
        ]
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
    > TupleToFFIPtrs for (A, B, C, D, E, F, G, H, I, J, K, L)
{
    type PTRS = [*mut c_void; 12];
    fn get_ptrs(&mut self) -> Self::PTRS {
        [
            self.0.get_ffi_ptr(),
            self.1.get_ffi_ptr(),
            self.2.get_ffi_ptr(),
            self.3.get_ffi_ptr(),
            self.4.get_ffi_ptr(),
            self.5.get_ffi_ptr(),
            self.6.get_ffi_ptr(),
            self.7.get_ffi_ptr(),
            self.8.get_ffi_ptr(),
            self.9.get_ffi_ptr(),
            self.10.get_ffi_ptr(),
            self.11.get_ffi_ptr(),
        ]
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
    > TupleToFFIPtrs for (A, B, C, D, E, F, G, H, I, J, K, L, M)
{
    type PTRS = [*mut c_void; 13];
    fn get_ptrs(&mut self) -> Self::PTRS {
        [
            self.0.get_ffi_ptr(),
            self.1.get_ffi_ptr(),
            self.2.get_ffi_ptr(),
            self.3.get_ffi_ptr(),
            self.4.get_ffi_ptr(),
            self.5.get_ffi_ptr(),
            self.6.get_ffi_ptr(),
            self.7.get_ffi_ptr(),
            self.8.get_ffi_ptr(),
            self.9.get_ffi_ptr(),
            self.10.get_ffi_ptr(),
            self.11.get_ffi_ptr(),
            self.12.get_ffi_ptr(),
        ]
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
    > TupleToFFIPtrs for (A, B, C, D, E, F, G, H, I, J, K, L, M, N)
{
    type PTRS = [*mut c_void; 14];
    fn get_ptrs(&mut self) -> Self::PTRS {
        [
            self.0.get_ffi_ptr(),
            self.1.get_ffi_ptr(),
            self.2.get_ffi_ptr(),
            self.3.get_ffi_ptr(),
            self.4.get_ffi_ptr(),
            self.5.get_ffi_ptr(),
            self.6.get_ffi_ptr(),
            self.7.get_ffi_ptr(),
            self.8.get_ffi_ptr(),
            self.9.get_ffi_ptr(),
            self.10.get_ffi_ptr(),
            self.11.get_ffi_ptr(),
            self.12.get_ffi_ptr(),
            self.13.get_ffi_ptr(),
        ]
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
    > TupleToFFIPtrs for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O)
{
    type PTRS = [*mut c_void; 15];
    fn get_ptrs(&mut self) -> Self::PTRS {
        [
            self.0.get_ffi_ptr(),
            self.1.get_ffi_ptr(),
            self.2.get_ffi_ptr(),
            self.3.get_ffi_ptr(),
            self.4.get_ffi_ptr(),
            self.5.get_ffi_ptr(),
            self.6.get_ffi_ptr(),
            self.7.get_ffi_ptr(),
            self.8.get_ffi_ptr(),
            self.9.get_ffi_ptr(),
            self.10.get_ffi_ptr(),
            self.11.get_ffi_ptr(),
            self.12.get_ffi_ptr(),
            self.13.get_ffi_ptr(),
            self.14.get_ffi_ptr(),
        ]
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
    > TupleToFFIPtrs for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)
{
    type PTRS = [*mut c_void; 16];
    fn get_ptrs(&mut self) -> Self::PTRS {
        [
            self.0.get_ffi_ptr(),
            self.1.get_ffi_ptr(),
            self.2.get_ffi_ptr(),
            self.3.get_ffi_ptr(),
            self.4.get_ffi_ptr(),
            self.5.get_ffi_ptr(),
            self.6.get_ffi_ptr(),
            self.7.get_ffi_ptr(),
            self.8.get_ffi_ptr(),
            self.9.get_ffi_ptr(),
            self.10.get_ffi_ptr(),
            self.11.get_ffi_ptr(),
            self.12.get_ffi_ptr(),
            self.13.get_ffi_ptr(),
            self.14.get_ffi_ptr(),
            self.15.get_ffi_ptr(),
        ]
    }
}
use crate::{Class, InteropClass, InteropSend};
pub trait CompareClasses {
    fn compare(classes: &[Class]) -> bool;
}
impl CompareClasses for () {
    fn compare(classes: &[Class]) -> bool {
        classes.is_empty()
    }
}
impl<A: InteropClass> CompareClasses for (A,) {
    fn compare(classes: &[Class]) -> bool {
        (classes.len() == 1) && (A::get_mono_class().is_assignable_from(&classes[0]))
    }
}
impl<A: InteropClass, B: InteropClass> CompareClasses for (A, B)
where
    (A, B): TupleToFFIPtrs,
{
    fn compare(classes: &[Class]) -> bool {
        (classes.len() == 2)
            && (A::get_mono_class().is_assignable_from(&classes[0]))
            && (B::get_mono_class().is_assignable_from(&classes[1]))
    }
}
impl<A: InteropClass, B: InteropClass, C: InteropClass> CompareClasses for (A, B, C)
where
    (A, B, C): TupleToFFIPtrs,
{
    fn compare(classes: &[Class]) -> bool {
        (classes.len() == 3)
            && (A::get_mono_class().is_assignable_from(&classes[0]))
            && (B::get_mono_class().is_assignable_from(&classes[1]))
            && (C::get_mono_class().is_assignable_from(&classes[2]))
    }
}
impl<A: InteropClass, B: InteropClass, C: InteropClass, D: InteropClass> CompareClasses
    for (A, B, C, D)
where
    (A, B, C, D): TupleToFFIPtrs,
{
    fn compare(classes: &[Class]) -> bool {
        (classes.len() == 4)
            && (A::get_mono_class().is_assignable_from(&classes[0]))
            && (B::get_mono_class().is_assignable_from(&classes[1]))
            && (C::get_mono_class().is_assignable_from(&classes[2]))
            && (D::get_mono_class().is_assignable_from(&classes[3]))
    }
}
impl<A: InteropClass, B: InteropClass, C: InteropClass, D: InteropClass, E: InteropClass>
    CompareClasses for (A, B, C, D, E)
where
    (A, B, C, D, E): TupleToFFIPtrs,
{
    fn compare(classes: &[Class]) -> bool {
        (classes.len() == 5)
            && (A::get_mono_class().is_assignable_from(&classes[0]))
            && (B::get_mono_class().is_assignable_from(&classes[1]))
            && (C::get_mono_class().is_assignable_from(&classes[2]))
            && (D::get_mono_class().is_assignable_from(&classes[3]))
            && (E::get_mono_class().is_assignable_from(&classes[4]))
    }
}
impl<
        A: InteropClass,
        B: InteropClass,
        C: InteropClass,
        D: InteropClass,
        E: InteropClass,
        F: InteropClass,
    > CompareClasses for (A, B, C, D, E, F)
where
    (A, B, C, D, E, F): TupleToFFIPtrs,
{
    fn compare(classes: &[Class]) -> bool {
        (classes.len() == 6)
            && (A::get_mono_class().is_assignable_from(&classes[0]))
            && (B::get_mono_class().is_assignable_from(&classes[1]))
            && (C::get_mono_class().is_assignable_from(&classes[2]))
            && (D::get_mono_class().is_assignable_from(&classes[3]))
            && (E::get_mono_class().is_assignable_from(&classes[4]))
            && (F::get_mono_class().is_assignable_from(&classes[5]))
    }
}
impl<
        A: InteropClass,
        B: InteropClass,
        C: InteropClass,
        D: InteropClass,
        E: InteropClass,
        F: InteropClass,
        G: InteropClass,
    > CompareClasses for (A, B, C, D, E, F, G)
where
    (A, B, C, D, E, F, G): TupleToFFIPtrs,
{
    fn compare(classes: &[Class]) -> bool {
        (classes.len() == 7)
            && (A::get_mono_class().is_assignable_from(&classes[0]))
            && (B::get_mono_class().is_assignable_from(&classes[1]))
            && (C::get_mono_class().is_assignable_from(&classes[2]))
            && (D::get_mono_class().is_assignable_from(&classes[3]))
            && (E::get_mono_class().is_assignable_from(&classes[4]))
            && (F::get_mono_class().is_assignable_from(&classes[5]))
            && (G::get_mono_class().is_assignable_from(&classes[6]))
    }
}
impl<
        A: InteropClass,
        B: InteropClass,
        C: InteropClass,
        D: InteropClass,
        E: InteropClass,
        F: InteropClass,
        G: InteropClass,
        H: InteropClass,
    > CompareClasses for (A, B, C, D, E, F, G, H)
where
    (A, B, C, D, E, F, G, H): TupleToFFIPtrs,
{
    fn compare(classes: &[Class]) -> bool {
        (classes.len() == 8)
            && (A::get_mono_class().is_assignable_from(&classes[0]))
            && (B::get_mono_class().is_assignable_from(&classes[1]))
            && (C::get_mono_class().is_assignable_from(&classes[2]))
            && (D::get_mono_class().is_assignable_from(&classes[3]))
            && (E::get_mono_class().is_assignable_from(&classes[4]))
            && (F::get_mono_class().is_assignable_from(&classes[5]))
            && (G::get_mono_class().is_assignable_from(&classes[6]))
            && (H::get_mono_class().is_assignable_from(&classes[7]))
    }
}
impl<
        A: InteropClass,
        B: InteropClass,
        C: InteropClass,
        D: InteropClass,
        E: InteropClass,
        F: InteropClass,
        G: InteropClass,
        H: InteropClass,
        I: InteropClass,
    > CompareClasses for (A, B, C, D, E, F, G, H, I)
where
    (A, B, C, D, E, F, G, H, I): TupleToFFIPtrs,
{
    fn compare(classes: &[Class]) -> bool {
        (classes.len() == 9)
            && (A::get_mono_class().is_assignable_from(&classes[0]))
            && (B::get_mono_class().is_assignable_from(&classes[1]))
            && (C::get_mono_class().is_assignable_from(&classes[2]))
            && (D::get_mono_class().is_assignable_from(&classes[3]))
            && (E::get_mono_class().is_assignable_from(&classes[4]))
            && (F::get_mono_class().is_assignable_from(&classes[5]))
            && (G::get_mono_class().is_assignable_from(&classes[6]))
            && (H::get_mono_class().is_assignable_from(&classes[7]))
            && (I::get_mono_class().is_assignable_from(&classes[8]))
    }
}
impl<
        A: InteropClass,
        B: InteropClass,
        C: InteropClass,
        D: InteropClass,
        E: InteropClass,
        F: InteropClass,
        G: InteropClass,
        H: InteropClass,
        I: InteropClass,
        J: InteropClass,
    > CompareClasses for (A, B, C, D, E, F, G, H, I, J)
where
    (A, B, C, D, E, F, G, H, I, J): TupleToFFIPtrs,
{
    fn compare(classes: &[Class]) -> bool {
        (classes.len() == 10)
            && (A::get_mono_class().is_assignable_from(&classes[0]))
            && (B::get_mono_class().is_assignable_from(&classes[1]))
            && (C::get_mono_class().is_assignable_from(&classes[2]))
            && (D::get_mono_class().is_assignable_from(&classes[3]))
            && (E::get_mono_class().is_assignable_from(&classes[4]))
            && (F::get_mono_class().is_assignable_from(&classes[5]))
            && (G::get_mono_class().is_assignable_from(&classes[6]))
            && (H::get_mono_class().is_assignable_from(&classes[7]))
            && (I::get_mono_class().is_assignable_from(&classes[8]))
            && (J::get_mono_class().is_assignable_from(&classes[9]))
    }
}
impl<
        A: InteropClass,
        B: InteropClass,
        C: InteropClass,
        D: InteropClass,
        E: InteropClass,
        F: InteropClass,
        G: InteropClass,
        H: InteropClass,
        I: InteropClass,
        J: InteropClass,
        K: InteropClass,
    > CompareClasses for (A, B, C, D, E, F, G, H, I, J, K)
where
    (A, B, C, D, E, F, G, H, I, J, K): TupleToFFIPtrs,
{
    fn compare(classes: &[Class]) -> bool {
        (classes.len() == 11)
            && (A::get_mono_class().is_assignable_from(&classes[0]))
            && (B::get_mono_class().is_assignable_from(&classes[1]))
            && (C::get_mono_class().is_assignable_from(&classes[2]))
            && (D::get_mono_class().is_assignable_from(&classes[3]))
            && (E::get_mono_class().is_assignable_from(&classes[4]))
            && (F::get_mono_class().is_assignable_from(&classes[5]))
            && (G::get_mono_class().is_assignable_from(&classes[6]))
            && (H::get_mono_class().is_assignable_from(&classes[7]))
            && (I::get_mono_class().is_assignable_from(&classes[8]))
            && (J::get_mono_class().is_assignable_from(&classes[9]))
            && (K::get_mono_class().is_assignable_from(&classes[10]))
    }
}
impl<
        A: InteropClass,
        B: InteropClass,
        C: InteropClass,
        D: InteropClass,
        E: InteropClass,
        F: InteropClass,
        G: InteropClass,
        H: InteropClass,
        I: InteropClass,
        J: InteropClass,
        K: InteropClass,
        L: InteropClass,
    > CompareClasses for (A, B, C, D, E, F, G, H, I, J, K, L)
where
    (A, B, C, D, E, F, G, H, I, J, K, L): TupleToFFIPtrs,
{
    fn compare(classes: &[Class]) -> bool {
        (classes.len() == 12)
            && (A::get_mono_class().is_assignable_from(&classes[0]))
            && (B::get_mono_class().is_assignable_from(&classes[1]))
            && (C::get_mono_class().is_assignable_from(&classes[2]))
            && (D::get_mono_class().is_assignable_from(&classes[3]))
            && (E::get_mono_class().is_assignable_from(&classes[4]))
            && (F::get_mono_class().is_assignable_from(&classes[5]))
            && (G::get_mono_class().is_assignable_from(&classes[6]))
            && (H::get_mono_class().is_assignable_from(&classes[7]))
            && (I::get_mono_class().is_assignable_from(&classes[8]))
            && (J::get_mono_class().is_assignable_from(&classes[9]))
            && (K::get_mono_class().is_assignable_from(&classes[10]))
            && (L::get_mono_class().is_assignable_from(&classes[11]))
    }
}
impl<
        A: InteropClass,
        B: InteropClass,
        C: InteropClass,
        D: InteropClass,
        E: InteropClass,
        F: InteropClass,
        G: InteropClass,
        H: InteropClass,
        I: InteropClass,
        J: InteropClass,
        K: InteropClass,
        L: InteropClass,
        M: InteropClass,
    > CompareClasses for (A, B, C, D, E, F, G, H, I, J, K, L, M)
where
    (A, B, C, D, E, F, G, H, I, J, K, L, M): TupleToFFIPtrs,
{
    fn compare(classes: &[Class]) -> bool {
        (classes.len() == 13)
            && (A::get_mono_class().is_assignable_from(&classes[0]))
            && (B::get_mono_class().is_assignable_from(&classes[1]))
            && (C::get_mono_class().is_assignable_from(&classes[2]))
            && (D::get_mono_class().is_assignable_from(&classes[3]))
            && (E::get_mono_class().is_assignable_from(&classes[4]))
            && (F::get_mono_class().is_assignable_from(&classes[5]))
            && (G::get_mono_class().is_assignable_from(&classes[6]))
            && (H::get_mono_class().is_assignable_from(&classes[7]))
            && (I::get_mono_class().is_assignable_from(&classes[8]))
            && (J::get_mono_class().is_assignable_from(&classes[9]))
            && (K::get_mono_class().is_assignable_from(&classes[10]))
            && (L::get_mono_class().is_assignable_from(&classes[11]))
            && (M::get_mono_class().is_assignable_from(&classes[12]))
    }
}
impl<
        A: InteropClass,
        B: InteropClass,
        C: InteropClass,
        D: InteropClass,
        E: InteropClass,
        F: InteropClass,
        G: InteropClass,
        H: InteropClass,
        I: InteropClass,
        J: InteropClass,
        K: InteropClass,
        L: InteropClass,
        M: InteropClass,
        N: InteropClass,
    > CompareClasses for (A, B, C, D, E, F, G, H, I, J, K, L, M, N)
where
    (A, B, C, D, E, F, G, H, I, J, K, L, M, N): TupleToFFIPtrs,
{
    fn compare(classes: &[Class]) -> bool {
        (classes.len() == 14)
            && (A::get_mono_class().is_assignable_from(&classes[0]))
            && (B::get_mono_class().is_assignable_from(&classes[1]))
            && (C::get_mono_class().is_assignable_from(&classes[2]))
            && (D::get_mono_class().is_assignable_from(&classes[3]))
            && (E::get_mono_class().is_assignable_from(&classes[4]))
            && (F::get_mono_class().is_assignable_from(&classes[5]))
            && (G::get_mono_class().is_assignable_from(&classes[6]))
            && (H::get_mono_class().is_assignable_from(&classes[7]))
            && (I::get_mono_class().is_assignable_from(&classes[8]))
            && (J::get_mono_class().is_assignable_from(&classes[9]))
            && (K::get_mono_class().is_assignable_from(&classes[10]))
            && (L::get_mono_class().is_assignable_from(&classes[11]))
            && (M::get_mono_class().is_assignable_from(&classes[12]))
            && (N::get_mono_class().is_assignable_from(&classes[13]))
    }
}
impl<
        A: InteropClass,
        B: InteropClass,
        C: InteropClass,
        D: InteropClass,
        E: InteropClass,
        F: InteropClass,
        G: InteropClass,
        H: InteropClass,
        I: InteropClass,
        J: InteropClass,
        K: InteropClass,
        L: InteropClass,
        M: InteropClass,
        N: InteropClass,
        O: InteropClass,
    > CompareClasses for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O)
where
    (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O): TupleToFFIPtrs,
{
    fn compare(classes: &[Class]) -> bool {
        (classes.len() == 15)
            && (A::get_mono_class().is_assignable_from(&classes[0]))
            && (B::get_mono_class().is_assignable_from(&classes[1]))
            && (C::get_mono_class().is_assignable_from(&classes[2]))
            && (D::get_mono_class().is_assignable_from(&classes[3]))
            && (E::get_mono_class().is_assignable_from(&classes[4]))
            && (F::get_mono_class().is_assignable_from(&classes[5]))
            && (G::get_mono_class().is_assignable_from(&classes[6]))
            && (H::get_mono_class().is_assignable_from(&classes[7]))
            && (I::get_mono_class().is_assignable_from(&classes[8]))
            && (J::get_mono_class().is_assignable_from(&classes[9]))
            && (K::get_mono_class().is_assignable_from(&classes[10]))
            && (L::get_mono_class().is_assignable_from(&classes[11]))
            && (M::get_mono_class().is_assignable_from(&classes[12]))
            && (N::get_mono_class().is_assignable_from(&classes[13]))
            && (O::get_mono_class().is_assignable_from(&classes[14]))
    }
}
impl<
        A: InteropClass,
        B: InteropClass,
        C: InteropClass,
        D: InteropClass,
        E: InteropClass,
        F: InteropClass,
        G: InteropClass,
        H: InteropClass,
        I: InteropClass,
        J: InteropClass,
        K: InteropClass,
        L: InteropClass,
        M: InteropClass,
        N: InteropClass,
        O: InteropClass,
        P: InteropClass,
    > CompareClasses for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)
where
    (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P): TupleToFFIPtrs,
{
    fn compare(classes: &[Class]) -> bool {
        (classes.len() == 16)
            && (A::get_mono_class().is_assignable_from(&classes[0]))
            && (B::get_mono_class().is_assignable_from(&classes[1]))
            && (C::get_mono_class().is_assignable_from(&classes[2]))
            && (D::get_mono_class().is_assignable_from(&classes[3]))
            && (E::get_mono_class().is_assignable_from(&classes[4]))
            && (F::get_mono_class().is_assignable_from(&classes[5]))
            && (G::get_mono_class().is_assignable_from(&classes[6]))
            && (H::get_mono_class().is_assignable_from(&classes[7]))
            && (I::get_mono_class().is_assignable_from(&classes[8]))
            && (J::get_mono_class().is_assignable_from(&classes[9]))
            && (K::get_mono_class().is_assignable_from(&classes[10]))
            && (L::get_mono_class().is_assignable_from(&classes[11]))
            && (M::get_mono_class().is_assignable_from(&classes[12]))
            && (N::get_mono_class().is_assignable_from(&classes[13]))
            && (O::get_mono_class().is_assignable_from(&classes[14]))
            && (P::get_mono_class().is_assignable_from(&classes[15]))
    }
}
//use lazy_static::*;
