use core::ffi::c_void;
//for argument procesing
type VoidPtr = *mut c_void;
//Conversion of a tuple to pointers
pub trait TupleToPtrs{
    type Res;
    fn get_ptrs(ptr:*mut Self)->Self::Res;
}
impl<A,B> TupleToPtrs for (A,B){
    type Res = [*mut c_void;2];
    fn get_ptrs(ptr:*mut Self)->Self::Res{
        let a = ptr as usize;
        let b = a + std::mem::size_of::<A>();
        [a as VoidPtr, b as VoidPtr]
    }
}
impl<A,B,C> TupleToPtrs for (A,B,C){
    type Res = [*mut c_void;3];
    fn get_ptrs(ptr:*mut Self)->Self::Res{
        let a = ptr as usize;
        let b = a + std::mem::size_of::<A>();
        let c = b + std::mem::size_of::<B>();
        [a as VoidPtr, b as VoidPtr,c as VoidPtr]
    }
}
impl<A,B,C,D> TupleToPtrs for (A,B,C,D){
    type Res = [*mut c_void;4];
    fn get_ptrs(ptr:*mut Self)->Self::Res{
        let a = ptr as usize;
        let b = a + std::mem::size_of::<A>();
        let c = b + std::mem::size_of::<B>();
        let d = c + std::mem::size_of::<C>();
        [a as VoidPtr, b as VoidPtr,c as VoidPtr,d as VoidPtr]
    }
}
impl<A,B,C,D,E> TupleToPtrs for (A,B,C,D,E){
    type Res = [*mut c_void;5];
    fn get_ptrs(ptr:*mut Self)->Self::Res{
        let a = ptr as usize;
        let b = a + std::mem::size_of::<A>();
        let c = b + std::mem::size_of::<B>();
        let d = c + std::mem::size_of::<C>();
        let e = d + std::mem::size_of::<D>();
        [a as VoidPtr, b as VoidPtr,c as VoidPtr,d as VoidPtr,e as VoidPtr]
    }
}
impl<A,B,C,D,E,F> TupleToPtrs for (A,B,C,D,E,F){
    type Res = [*mut c_void;6];
    fn get_ptrs(ptr:*mut Self)->Self::Res{
        let a = ptr as usize;
        let b = a + std::mem::size_of::<A>();
        let c = b + std::mem::size_of::<B>();
        let d = c + std::mem::size_of::<C>();
        let e = d + std::mem::size_of::<D>();
        let f = e + std::mem::size_of::<E>();
        [a as VoidPtr, b as VoidPtr,c as VoidPtr,d as VoidPtr,e as VoidPtr,f as VoidPtr]
    }
}
impl<A,B,C,D,E,F,G> TupleToPtrs for (A,B,C,D,E,F,G){
    type Res = [*mut c_void;7];
    fn get_ptrs(ptr:*mut Self)->Self::Res{
        let a = ptr as usize;
        let b = a + std::mem::size_of::<A>();
        let c = b + std::mem::size_of::<B>();
        let d = c + std::mem::size_of::<C>();
        let e = d + std::mem::size_of::<D>();
        let f = e + std::mem::size_of::<E>();
        let g = f + std::mem::size_of::<F>();
        [a as VoidPtr, b as VoidPtr,c as VoidPtr,d as VoidPtr,e as VoidPtr,f as VoidPtr,g as VoidPtr]
    }
}
impl<A,B,C,D,E,F,G,H> TupleToPtrs for (A,B,C,D,E,F,G,H){
    type Res = [*mut c_void;8];
    fn get_ptrs(ptr:*mut Self)->Self::Res{
            let a = ptr as usize;
            let b = a + std::mem::size_of::<A>();
            let c = b + std::mem::size_of::<B>();
            let d = c + std::mem::size_of::<C>();
            let e = d + std::mem::size_of::<D>();
            let f = e + std::mem::size_of::<E>();
            let g = f + std::mem::size_of::<F>();
            let h = g + std::mem::size_of::<G>();
            [a as VoidPtr, b as VoidPtr,c as VoidPtr,d as VoidPtr,e as VoidPtr,f as VoidPtr,g as VoidPtr,h as VoidPtr]
    }
}
impl<A,B,C,D,E,F,G,H,I> TupleToPtrs for (A,B,C,D,E,F,G,H,I){
    type Res = [*mut c_void;9];
    fn get_ptrs(ptr:*mut Self)->Self::Res{
            let a = ptr as usize;
            let b = a + std::mem::size_of::<A>();
            let c = b + std::mem::size_of::<B>();
            let d = c + std::mem::size_of::<C>();
            let e = d + std::mem::size_of::<D>();
            let f = e + std::mem::size_of::<E>();
            let g = f + std::mem::size_of::<F>();
            let h = g + std::mem::size_of::<G>();
            let i = h + std::mem::size_of::<H>();
            [a as VoidPtr, b as VoidPtr,c as VoidPtr,d as VoidPtr,e as VoidPtr,f as VoidPtr,
            g as VoidPtr,h as VoidPtr,i as VoidPtr]
    }
}
impl<A,B,C,D,E,F,G,H,I,J> TupleToPtrs for (A,B,C,D,E,F,G,H,I,J){
    type Res = [*mut c_void;10];
    fn get_ptrs(ptr:*mut Self)->Self::Res{
            let a = ptr as usize;
            let b = a + std::mem::size_of::<A>();
            let c = b + std::mem::size_of::<B>();
            let d = c + std::mem::size_of::<C>();
            let e = d + std::mem::size_of::<D>();
            let f = e + std::mem::size_of::<E>();
            let g = f + std::mem::size_of::<F>();
            let h = g + std::mem::size_of::<G>();
            let i = h + std::mem::size_of::<H>();
            let j = i + std::mem::size_of::<I>();
            [a as VoidPtr, b as VoidPtr,c as VoidPtr,d as VoidPtr,e as VoidPtr,f as VoidPtr,
            g as VoidPtr,h as VoidPtr,i as VoidPtr,j as VoidPtr]
    }
}
impl<A,B,C,D,E,F,G,H,I,J,K> TupleToPtrs for (A,B,C,D,E,F,G,H,I,J,K){
    type Res = [*mut c_void;11];
    fn get_ptrs(ptr:*mut Self)->Self::Res{
            let a = ptr as usize;
            let b = a + std::mem::size_of::<A>();
            let c = b + std::mem::size_of::<B>();
            let d = c + std::mem::size_of::<C>();
            let e = d + std::mem::size_of::<D>();
            let f = e + std::mem::size_of::<E>();
            let g = f + std::mem::size_of::<F>();
            let h = g + std::mem::size_of::<G>();
            let i = h + std::mem::size_of::<H>();
            let j = i + std::mem::size_of::<I>();
            let k = j + std::mem::size_of::<J>();
            [a as VoidPtr, b as VoidPtr,c as VoidPtr,d as VoidPtr,e as VoidPtr,f as VoidPtr,
            g as VoidPtr,h as VoidPtr,i as VoidPtr,j as VoidPtr,k as VoidPtr]
    }
} 
impl<A,B,C,D,E,F,G,H,I,J,K,L> TupleToPtrs for (A,B,C,D,E,F,G,H,I,J,K,L){
    type Res = [*mut c_void;12];
    fn get_ptrs(ptr:*mut Self)->Self::Res{
            let a = ptr as usize;
            let b = a + std::mem::size_of::<A>();
            let c = b + std::mem::size_of::<B>();
            let d = c + std::mem::size_of::<C>();
            let e = d + std::mem::size_of::<D>();
            let f = e + std::mem::size_of::<E>();
            let g = f + std::mem::size_of::<F>();
            let h = g + std::mem::size_of::<G>();
            let i = h + std::mem::size_of::<H>();
            let j = i + std::mem::size_of::<I>();
            let k = j + std::mem::size_of::<J>();
            let l = k + std::mem::size_of::<K>();
            [a as VoidPtr, b as VoidPtr,c as VoidPtr,d as VoidPtr,e as VoidPtr,f as VoidPtr,
            g as VoidPtr,h as VoidPtr,i as VoidPtr,j as VoidPtr,k as VoidPtr,l as VoidPtr]
    }
} 
impl<A,B,C,D,E,F,G,H,I,J,K,L,M> TupleToPtrs for (A,B,C,D,E,F,G,H,I,J,K,L,M){
    type Res = [*mut c_void;13];
    fn get_ptrs(ptr:*mut Self)->Self::Res{
            let a = ptr as usize;
            let b = a + std::mem::size_of::<A>();
            let c = b + std::mem::size_of::<B>();
            let d = c + std::mem::size_of::<C>();
            let e = d + std::mem::size_of::<D>();
            let f = e + std::mem::size_of::<E>();
            let g = f + std::mem::size_of::<F>();
            let h = g + std::mem::size_of::<G>();
            let i = h + std::mem::size_of::<H>();
            let j = i + std::mem::size_of::<I>();
            let k = j + std::mem::size_of::<J>();
            let l = k + std::mem::size_of::<K>();
            let m = l + std::mem::size_of::<L>();
            [a as VoidPtr, b as VoidPtr,c as VoidPtr,d as VoidPtr,e as VoidPtr,f as VoidPtr,
            g as VoidPtr,h as VoidPtr,i as VoidPtr,j as VoidPtr,k as VoidPtr,l as VoidPtr,m as VoidPtr]
    }
} 
impl<A,B,C,D,E,F,G,H,I,J,K,L,M,N> TupleToPtrs for (A,B,C,D,E,F,G,H,I,J,K,L,M,N){
    type Res = [*mut c_void;14];
    fn get_ptrs(ptr:*mut Self)->Self::Res{
            let a = ptr as usize;
            let b = a + std::mem::size_of::<A>();
            let c = b + std::mem::size_of::<B>();
            let d = c + std::mem::size_of::<C>();
            let e = d + std::mem::size_of::<D>();
            let f = e + std::mem::size_of::<E>();
            let g = f + std::mem::size_of::<F>();
            let h = g + std::mem::size_of::<G>();
            let i = h + std::mem::size_of::<H>();
            let j = i + std::mem::size_of::<I>();
            let k = j + std::mem::size_of::<J>();
            let l = k + std::mem::size_of::<K>();
            let m = l + std::mem::size_of::<L>();
            let n = m + std::mem::size_of::<M>();
            [a as VoidPtr, b as VoidPtr,c as VoidPtr,d as VoidPtr,e as VoidPtr,f as VoidPtr,
            g as VoidPtr,h as VoidPtr,i as VoidPtr,j as VoidPtr,k as VoidPtr,l as VoidPtr,m as VoidPtr,
            n as VoidPtr]
    }
} 
impl<A,B,C,D,E,F,G,H,I,J,K,L,M,N,O> TupleToPtrs for (A,B,C,D,E,F,G,H,I,J,K,L,M,N,O){
    type Res = [*mut c_void;15];
    fn get_ptrs(ptr:*mut Self)->Self::Res{
            let a = ptr as usize;
            let b = a + std::mem::size_of::<A>();
            let c = b + std::mem::size_of::<B>();
            let d = c + std::mem::size_of::<C>();
            let e = d + std::mem::size_of::<D>();
            let f = e + std::mem::size_of::<E>();
            let g = f + std::mem::size_of::<F>();
            let h = g + std::mem::size_of::<G>();
            let i = h + std::mem::size_of::<H>();
            let j = i + std::mem::size_of::<I>();
            let k = j + std::mem::size_of::<J>();
            let l = k + std::mem::size_of::<K>();
            let m = l + std::mem::size_of::<L>();
            let n = m + std::mem::size_of::<M>();
            let o = n + std::mem::size_of::<N>();
            [a as VoidPtr, b as VoidPtr,c as VoidPtr,d as VoidPtr,e as VoidPtr,f as VoidPtr,
            g as VoidPtr,h as VoidPtr,i as VoidPtr,j as VoidPtr,k as VoidPtr,l as VoidPtr,m as VoidPtr,
            n as VoidPtr,o as VoidPtr]
    }
} 
impl<A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P> TupleToPtrs for (A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P){
    type Res = [*mut c_void;16];
    fn get_ptrs(ptr:*mut Self)->Self::Res{
            let a = ptr as usize;
            let b = a + std::mem::size_of::<A>();
            let c = b + std::mem::size_of::<B>();
            let d = c + std::mem::size_of::<C>();
            let e = d + std::mem::size_of::<D>();
            let f = e + std::mem::size_of::<E>();
            let g = f + std::mem::size_of::<F>();
            let h = g + std::mem::size_of::<G>();
            let i = h + std::mem::size_of::<H>();
            let j = i + std::mem::size_of::<I>();
            let k = j + std::mem::size_of::<J>();
            let l = k + std::mem::size_of::<K>();
            let m = l + std::mem::size_of::<L>();
            let n = m + std::mem::size_of::<M>();
            let o = n + std::mem::size_of::<N>();
            let p = o + std::mem::size_of::<O>();
            [a as VoidPtr, b as VoidPtr,c as VoidPtr,d as VoidPtr,e as VoidPtr,f as VoidPtr,
            g as VoidPtr,h as VoidPtr,i as VoidPtr,j as VoidPtr,k as VoidPtr,l as VoidPtr,m as VoidPtr,
            n as VoidPtr,o as VoidPtr,p as VoidPtr]
    }
} 
use crate::{Class,InteropClass};
pub trait CompareClasses{
    fn compare(clases:&Vec<Class>)->bool;
}
impl<A:InteropClass,B:InteropClass> CompareClasses for (A,B) where (A,B):TupleToPtrs{
    fn compare(clases:&Vec<Class>)->bool{
        (clases.len() == 2)&&(A::get_mono_class().is_assignable_from(&clases[0]))&&(B::get_mono_class().is_assignable_from(&clases[1]))
    }
}
impl<A:InteropClass,B:InteropClass,C:InteropClass> CompareClasses for (A,B,C) where (A,B,C):TupleToPtrs{
    fn compare(clases:&Vec<Class>)->bool{
        (clases.len() == 3)&&(A::get_mono_class().is_assignable_from(&clases[0]))&&(B::get_mono_class().is_assignable_from(&clases[1]))&&
        (C::get_mono_class().is_assignable_from(&clases[2]))
    }
}
impl<A:InteropClass,B:InteropClass,C:InteropClass,D:InteropClass> CompareClasses for (A,B,C,D) where (A,B,C,D):TupleToPtrs{
    fn compare(clases:&Vec<Class>)->bool{
        (clases.len() == 4)&&(A::get_mono_class().is_assignable_from(&clases[0]))&&(B::get_mono_class().is_assignable_from(&clases[1]))&&
        (C::get_mono_class().is_assignable_from(&clases[2]))&&(D::get_mono_class().is_assignable_from(&clases[3]))
    }
}
impl<A:InteropClass,B:InteropClass,C:InteropClass,D:InteropClass,E:InteropClass> CompareClasses for (A,B,C,D,E) where (A,B,C,D,E):TupleToPtrs{
    fn compare(clases:&Vec<Class>)->bool{
        (clases.len() == 5)&&(A::get_mono_class().is_assignable_from(&clases[0]))&&(B::get_mono_class().is_assignable_from(&clases[1]))&&
        (C::get_mono_class().is_assignable_from(&clases[2]))&&(D::get_mono_class().is_assignable_from(&clases[3]))&&(E::get_mono_class().is_assignable_from(&clases[4]))
    }
}
impl<A:InteropClass,B:InteropClass,C:InteropClass,D:InteropClass,E:InteropClass,F:InteropClass> CompareClasses for (A,B,C,D,E,F) where (A,B,C,D,E,F):TupleToPtrs{
    fn compare(clases:&Vec<Class>)->bool{
        (clases.len() == 6)&&(A::get_mono_class().is_assignable_from(&clases[0]))&&(B::get_mono_class().is_assignable_from(&clases[1]))&&
        (C::get_mono_class().is_assignable_from(&clases[2]))&&(D::get_mono_class().is_assignable_from(&clases[3]))&&(E::get_mono_class().is_assignable_from(&clases[4]))&&
        (F::get_mono_class().is_assignable_from(&clases[5]))
    }
}
impl<A:InteropClass,B:InteropClass,C:InteropClass,D:InteropClass,E:InteropClass,F:InteropClass,G:InteropClass> CompareClasses 
for (A,B,C,D,E,F,G) where (A,B,C,D,E,F,G):TupleToPtrs{
    fn compare(clases:&Vec<Class>)->bool{
        (clases.len() == 7)&&(A::get_mono_class().is_assignable_from(&clases[0]))&&(B::get_mono_class().is_assignable_from(&clases[1]))&&
        (C::get_mono_class().is_assignable_from(&clases[2]))&&(D::get_mono_class().is_assignable_from(&clases[3]))&&(E::get_mono_class().is_assignable_from(&clases[4]))&&
        (F::get_mono_class().is_assignable_from(&clases[5]))&&(G::get_mono_class().is_assignable_from(&clases[6]))
    }
}
impl<A:InteropClass,B:InteropClass,C:InteropClass,D:InteropClass,E:InteropClass,F:InteropClass,G:InteropClass,H:InteropClass> CompareClasses 
for (A,B,C,D,E,F,G,H) where (A,B,C,D,E,F,G,H):TupleToPtrs{
    fn compare(clases:&Vec<Class>)->bool{
        (clases.len() == 8)&&(A::get_mono_class().is_assignable_from(&clases[0]))&&(B::get_mono_class().is_assignable_from(&clases[1]))&&
        (C::get_mono_class().is_assignable_from(&clases[2]))&&(D::get_mono_class().is_assignable_from(&clases[3]))&&(E::get_mono_class().is_assignable_from(&clases[4]))&&
        (F::get_mono_class().is_assignable_from(&clases[5]))&&(G::get_mono_class().is_assignable_from(&clases[6]))&&(H::get_mono_class().is_assignable_from(&clases[7]))
    }
}
impl<A:InteropClass,B:InteropClass,C:InteropClass,D:InteropClass,E:InteropClass,F:InteropClass,G:InteropClass,H:InteropClass,I:InteropClass> CompareClasses 
for (A,B,C,D,E,F,G,H,I) where (A,B,C,D,E,F,G,H,I):TupleToPtrs{
    fn compare(clases:&Vec<Class>)->bool{
        (clases.len() == 9)&&(A::get_mono_class().is_assignable_from(&clases[0]))&&(B::get_mono_class().is_assignable_from(&clases[1]))&&
        (C::get_mono_class().is_assignable_from(&clases[2]))&&(D::get_mono_class().is_assignable_from(&clases[3]))&&(E::get_mono_class().is_assignable_from(&clases[4]))&&
        (F::get_mono_class().is_assignable_from(&clases[5]))&&(G::get_mono_class().is_assignable_from(&clases[6]))&&(H::get_mono_class().is_assignable_from(&clases[7]))&&
        (I::get_mono_class().is_assignable_from(&clases[8]))
    }
}
impl<A:InteropClass,B:InteropClass,C:InteropClass,D:InteropClass,E:InteropClass,F:InteropClass,G:InteropClass,H:InteropClass,I:InteropClass,J:InteropClass> CompareClasses 
for (A,B,C,D,E,F,G,H,I,J) where (A,B,C,D,E,F,G,H,I,J):TupleToPtrs{
    fn compare(clases:&Vec<Class>)->bool{
        (clases.len() == 10)&&(A::get_mono_class().is_assignable_from(&clases[0]))&&(B::get_mono_class().is_assignable_from(&clases[1]))&&
        (C::get_mono_class().is_assignable_from(&clases[2]))&&(D::get_mono_class().is_assignable_from(&clases[3]))&&(E::get_mono_class().is_assignable_from(&clases[4]))&&
        (F::get_mono_class().is_assignable_from(&clases[5]))&&(G::get_mono_class().is_assignable_from(&clases[6]))&&(H::get_mono_class().is_assignable_from(&clases[7]))&&
        (I::get_mono_class().is_assignable_from(&clases[8]))&&(J::get_mono_class().is_assignable_from(&clases[9]))
    }
}
impl<A:InteropClass,B:InteropClass,C:InteropClass,D:InteropClass,E:InteropClass,F:InteropClass,G:InteropClass,H:InteropClass,I:InteropClass,J:InteropClass,
K:InteropClass> CompareClasses 
for (A,B,C,D,E,F,G,H,I,J,K) where (A,B,C,D,E,F,G,H,I,J,K):TupleToPtrs{
    fn compare(clases:&Vec<Class>)->bool{
        (clases.len() == 11)&&(A::get_mono_class().is_assignable_from(&clases[0]))&&(B::get_mono_class().is_assignable_from(&clases[1]))&&
        (C::get_mono_class().is_assignable_from(&clases[2]))&&(D::get_mono_class().is_assignable_from(&clases[3]))&&(E::get_mono_class().is_assignable_from(&clases[4]))&&
        (F::get_mono_class().is_assignable_from(&clases[5]))&&(G::get_mono_class().is_assignable_from(&clases[6]))&&(H::get_mono_class().is_assignable_from(&clases[7]))&&
        (I::get_mono_class().is_assignable_from(&clases[8]))&&(J::get_mono_class().is_assignable_from(&clases[9]))&&(K::get_mono_class().is_assignable_from(&clases[10]))
    }
}
impl<A:InteropClass,B:InteropClass,C:InteropClass,D:InteropClass,E:InteropClass,F:InteropClass,G:InteropClass,H:InteropClass,I:InteropClass,J:InteropClass,
K:InteropClass,L:InteropClass> CompareClasses 
for (A,B,C,D,E,F,G,H,I,J,K,L) where (A,B,C,D,E,F,G,H,I,J,K,L):TupleToPtrs{
    fn compare(clases:&Vec<Class>)->bool{
        (clases.len() == 12)&&(A::get_mono_class().is_assignable_from(&clases[0]))&&(B::get_mono_class().is_assignable_from(&clases[1]))&&
        (C::get_mono_class().is_assignable_from(&clases[2]))&&(D::get_mono_class().is_assignable_from(&clases[3]))&&(E::get_mono_class().is_assignable_from(&clases[4]))&&
        (F::get_mono_class().is_assignable_from(&clases[5]))&&(G::get_mono_class().is_assignable_from(&clases[6]))&&(H::get_mono_class().is_assignable_from(&clases[7]))&&
        (I::get_mono_class().is_assignable_from(&clases[8]))&&(J::get_mono_class().is_assignable_from(&clases[9]))&&(K::get_mono_class().is_assignable_from(&clases[10]))&&
        (L::get_mono_class().is_assignable_from(&clases[11]))
    }
}
impl<A:InteropClass,B:InteropClass,C:InteropClass,D:InteropClass,E:InteropClass,F:InteropClass,G:InteropClass,H:InteropClass,I:InteropClass,J:InteropClass,
K:InteropClass,L:InteropClass,M:InteropClass> CompareClasses 
for (A,B,C,D,E,F,G,H,I,J,K,L,M) where (A,B,C,D,E,F,G,H,I,J,K,L,M):TupleToPtrs{
    fn compare(clases:&Vec<Class>)->bool{
        (clases.len() == 13)&&(A::get_mono_class().is_assignable_from(&clases[0]))&&(B::get_mono_class().is_assignable_from(&clases[1]))&&
        (C::get_mono_class().is_assignable_from(&clases[2]))&&(D::get_mono_class().is_assignable_from(&clases[3]))&&(E::get_mono_class().is_assignable_from(&clases[4]))&&
        (F::get_mono_class().is_assignable_from(&clases[5]))&&(G::get_mono_class().is_assignable_from(&clases[6]))&&(H::get_mono_class().is_assignable_from(&clases[7]))&&
        (I::get_mono_class().is_assignable_from(&clases[8]))&&(J::get_mono_class().is_assignable_from(&clases[9]))&&(K::get_mono_class().is_assignable_from(&clases[10]))&&
        (L::get_mono_class().is_assignable_from(&clases[11]))&&(M::get_mono_class().is_assignable_from(&clases[12]))
    }
}
impl<A:InteropClass,B:InteropClass,C:InteropClass,D:InteropClass,E:InteropClass,F:InteropClass,G:InteropClass,H:InteropClass,I:InteropClass,J:InteropClass,
K:InteropClass,L:InteropClass,M:InteropClass,N:InteropClass> CompareClasses 
for (A,B,C,D,E,F,G,H,I,J,K,L,M,N) where (A,B,C,D,E,F,G,H,I,J,K,L,M,N):TupleToPtrs{
    fn compare(clases:&Vec<Class>)->bool{
        (clases.len() == 14)&&(A::get_mono_class().is_assignable_from(&clases[0]))&&(B::get_mono_class().is_assignable_from(&clases[1]))&&
        (C::get_mono_class().is_assignable_from(&clases[2]))&&(D::get_mono_class().is_assignable_from(&clases[3]))&&(E::get_mono_class().is_assignable_from(&clases[4]))&&
        (F::get_mono_class().is_assignable_from(&clases[5]))&&(G::get_mono_class().is_assignable_from(&clases[6]))&&(H::get_mono_class().is_assignable_from(&clases[7]))&&
        (I::get_mono_class().is_assignable_from(&clases[8]))&&(J::get_mono_class().is_assignable_from(&clases[9]))&&(K::get_mono_class().is_assignable_from(&clases[10]))&&
        (L::get_mono_class().is_assignable_from(&clases[11]))&&(M::get_mono_class().is_assignable_from(&clases[12]))&&(N::get_mono_class().is_assignable_from(&clases[13]))
    }
}
impl<A:InteropClass,B:InteropClass,C:InteropClass,D:InteropClass,E:InteropClass,F:InteropClass,G:InteropClass,H:InteropClass,I:InteropClass,J:InteropClass,
K:InteropClass,L:InteropClass,M:InteropClass,N:InteropClass,O:InteropClass> CompareClasses 
for (A,B,C,D,E,F,G,H,I,J,K,L,M,N,O) where (A,B,C,D,E,F,G,H,I,J,K,L,M,N,O):TupleToPtrs{
    fn compare(clases:&Vec<Class>)->bool{
        (clases.len() == 15)&&(A::get_mono_class().is_assignable_from(&clases[0]))&&(B::get_mono_class().is_assignable_from(&clases[1]))&&
        (C::get_mono_class().is_assignable_from(&clases[2]))&&(D::get_mono_class().is_assignable_from(&clases[3]))&&(E::get_mono_class().is_assignable_from(&clases[4]))&&
        (F::get_mono_class().is_assignable_from(&clases[5]))&&(G::get_mono_class().is_assignable_from(&clases[6]))&&(H::get_mono_class().is_assignable_from(&clases[7]))&&
        (I::get_mono_class().is_assignable_from(&clases[8]))&&(J::get_mono_class().is_assignable_from(&clases[9]))&&(K::get_mono_class().is_assignable_from(&clases[10]))&&
        (L::get_mono_class().is_assignable_from(&clases[11]))&&(M::get_mono_class().is_assignable_from(&clases[12]))&&(N::get_mono_class().is_assignable_from(&clases[13]))&&
        (O::get_mono_class().is_assignable_from(&clases[14]))
    }
}
impl<A:InteropClass,B:InteropClass,C:InteropClass,D:InteropClass,E:InteropClass,F:InteropClass,G:InteropClass,H:InteropClass,I:InteropClass,J:InteropClass,
K:InteropClass,L:InteropClass,M:InteropClass,N:InteropClass,O:InteropClass,P:InteropClass> CompareClasses 
for (A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P) where (A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P):TupleToPtrs{
    fn compare(clases:&Vec<Class>)->bool{
        (clases.len() == 16)&&(A::get_mono_class().is_assignable_from(&clases[0]))&&(B::get_mono_class().is_assignable_from(&clases[1]))&&
        (C::get_mono_class().is_assignable_from(&clases[2]))&&(D::get_mono_class().is_assignable_from(&clases[3]))&&(E::get_mono_class().is_assignable_from(&clases[4]))&&
        (F::get_mono_class().is_assignable_from(&clases[5]))&&(G::get_mono_class().is_assignable_from(&clases[6]))&&(H::get_mono_class().is_assignable_from(&clases[7]))&&
        (I::get_mono_class().is_assignable_from(&clases[8]))&&(J::get_mono_class().is_assignable_from(&clases[9]))&&(K::get_mono_class().is_assignable_from(&clases[10]))&&
        (L::get_mono_class().is_assignable_from(&clases[11]))&&(M::get_mono_class().is_assignable_from(&clases[12]))&&(N::get_mono_class().is_assignable_from(&clases[13]))&&
        (O::get_mono_class().is_assignable_from(&clases[14]))&&(P::get_mono_class().is_assignable_from(&clases[15]))
    }
}