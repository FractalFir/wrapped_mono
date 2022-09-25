use core::ffi::c_void;
//for argument procesing
type voidptr = *mut c_void;
//Conversion of a tuple to pointers
pub trait TupleToPtrs{
    type res;
    fn get_ptrs(ptr:*mut Self)->Self::res;
}
impl<A,B> TupleToPtrs for (A,B){
    type res = [*mut c_void;2];
    fn get_ptrs(ptr:*mut Self)->Self::res{
        let a = ptr as usize;
        let b = a + std::mem::size_of::<A>();
        return [a as voidptr, b as voidptr];
    }
}
impl<A,B,C> TupleToPtrs for (A,B,C){
    type res = [*mut c_void;3];
    fn get_ptrs(ptr:*mut Self)->Self::res{
        let a = ptr as usize;
        let b = a + std::mem::size_of::<A>();
        let c = b + std::mem::size_of::<B>();
        return [a as voidptr, b as voidptr,c as voidptr];
    }
}
impl<A,B,C,D> TupleToPtrs for (A,B,C,D){
    type res = [*mut c_void;4];
    fn get_ptrs(ptr:*mut Self)->Self::res{
        let a = ptr as usize;
        let b = a + std::mem::size_of::<A>();
        let c = b + std::mem::size_of::<B>();
        let d = c + std::mem::size_of::<C>();
        return [a as voidptr, b as voidptr,c as voidptr,d as voidptr];
    }
}
impl<A,B,C,D,E> TupleToPtrs for (A,B,C,D,E){
    type res = [*mut c_void;5];
    fn get_ptrs(ptr:*mut Self)->Self::res{
        let a = ptr as usize;
        let b = a + std::mem::size_of::<A>();
        let c = b + std::mem::size_of::<B>();
        let d = c + std::mem::size_of::<C>();
        let e = d + std::mem::size_of::<D>();
        return [a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr];
    }
}
impl<A,B,C,D,E,F> TupleToPtrs for (A,B,C,D,E,F){
    type res = [*mut c_void;6];
    fn get_ptrs(ptr:*mut Self)->Self::res{
        let a = ptr as usize;
        let b = a + std::mem::size_of::<A>();
        let c = b + std::mem::size_of::<B>();
        let d = c + std::mem::size_of::<C>();
        let e = d + std::mem::size_of::<D>();
        let f = e + std::mem::size_of::<E>();
        return [a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr,f as voidptr];
    }
}
impl<A,B,C,D,E,F,G> TupleToPtrs for (A,B,C,D,E,F,G){
    type res = [*mut c_void;7];
    fn get_ptrs(ptr:*mut Self)->Self::res{
        let a = ptr as usize;
        let b = a + std::mem::size_of::<A>();
        let c = b + std::mem::size_of::<B>();
        let d = c + std::mem::size_of::<C>();
        let e = d + std::mem::size_of::<D>();
        let f = e + std::mem::size_of::<E>();
        let g = f + std::mem::size_of::<F>();
        return [a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr,f as voidptr,g as voidptr];
    }
}
impl<A,B,C,D,E,F,G,H> TupleToPtrs for (A,B,C,D,E,F,G,H){
    type res = [*mut c_void;8];
    fn get_ptrs(ptr:*mut Self)->Self::res{
            let a = ptr as usize;
            let b = a + std::mem::size_of::<A>();
            let c = b + std::mem::size_of::<B>();
            let d = c + std::mem::size_of::<C>();
            let e = d + std::mem::size_of::<D>();
            let f = e + std::mem::size_of::<E>();
            let g = f + std::mem::size_of::<F>();
            let h = g + std::mem::size_of::<G>();
            return [a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr,f as voidptr,g as voidptr,h as voidptr];
    }
}
impl<A,B,C,D,E,F,G,H,I> TupleToPtrs for (A,B,C,D,E,F,G,H,I){
    type res = [*mut c_void;9];
    fn get_ptrs(ptr:*mut Self)->Self::res{
            let a = ptr as usize;
            let b = a + std::mem::size_of::<A>();
            let c = b + std::mem::size_of::<B>();
            let d = c + std::mem::size_of::<C>();
            let e = d + std::mem::size_of::<D>();
            let f = e + std::mem::size_of::<E>();
            let g = f + std::mem::size_of::<F>();
            let h = g + std::mem::size_of::<G>();
            let i = h + std::mem::size_of::<H>();
            return [a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr,f as voidptr,
            g as voidptr,h as voidptr,i as voidptr];
    }
}
impl<A,B,C,D,E,F,G,H,I,J> TupleToPtrs for (A,B,C,D,E,F,G,H,I,J){
    type res = [*mut c_void;10];
    fn get_ptrs(ptr:*mut Self)->Self::res{
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
            return [a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr,f as voidptr,
            g as voidptr,h as voidptr,i as voidptr,j as voidptr];
    }
}
impl<A,B,C,D,E,F,G,H,I,J,K> TupleToPtrs for (A,B,C,D,E,F,G,H,I,J,K){
    type res = [*mut c_void;11];
    fn get_ptrs(ptr:*mut Self)->Self::res{
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
            return [a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr,f as voidptr,
            g as voidptr,h as voidptr,i as voidptr,j as voidptr,k as voidptr];
    }
} 
impl<A,B,C,D,E,F,G,H,I,J,K,L> TupleToPtrs for (A,B,C,D,E,F,G,H,I,J,K,L){
    type res = [*mut c_void;12];
    fn get_ptrs(ptr:*mut Self)->Self::res{
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
            return [a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr,f as voidptr,
            g as voidptr,h as voidptr,i as voidptr,j as voidptr,k as voidptr,l as voidptr];
    }
} 
impl<A,B,C,D,E,F,G,H,I,J,K,L,M> TupleToPtrs for (A,B,C,D,E,F,G,H,I,J,K,L,M){
    type res = [*mut c_void;13];
    fn get_ptrs(ptr:*mut Self)->Self::res{
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
            return [a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr,f as voidptr,
            g as voidptr,h as voidptr,i as voidptr,j as voidptr,k as voidptr,l as voidptr,m as voidptr];
    }
} 
impl<A,B,C,D,E,F,G,H,I,J,K,L,M,N> TupleToPtrs for (A,B,C,D,E,F,G,H,I,J,K,L,M,N){
    type res = [*mut c_void;14];
    fn get_ptrs(ptr:*mut Self)->Self::res{
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
            return [a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr,f as voidptr,
            g as voidptr,h as voidptr,i as voidptr,j as voidptr,k as voidptr,l as voidptr,m as voidptr,
            n as voidptr];
    }
} 
impl<A,B,C,D,E,F,G,H,I,J,K,L,M,N,O> TupleToPtrs for (A,B,C,D,E,F,G,H,I,J,K,L,M,N,O){
    type res = [*mut c_void;15];
    fn get_ptrs(ptr:*mut Self)->Self::res{
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
            return [a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr,f as voidptr,
            g as voidptr,h as voidptr,i as voidptr,j as voidptr,k as voidptr,l as voidptr,m as voidptr,
            n as voidptr,o as voidptr];
    }
} 
impl<A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P> TupleToPtrs for (A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P){
    type res = [*mut c_void;16];
    fn get_ptrs(ptr:*mut Self)->Self::res{
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
            return [a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr,f as voidptr,
            g as voidptr,h as voidptr,i as voidptr,j as voidptr,k as voidptr,l as voidptr,m as voidptr,
            n as voidptr,o as voidptr,p as voidptr];
    }
} 