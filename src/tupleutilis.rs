use core::ffi::c_void;
//for argument procesing
type voidptr = *mut c_void;
//Conversion of a tuple to pointers
pub trait TupleToPtrs{
    fn get_ptrs(ptr:*mut Self)->Vec<*mut c_void>;
}
impl<A,B> TupleToPtrs for (A,B){
    fn get_ptrs(ptr:*mut Self)->Vec<*mut c_void>{
        let a = ptr as usize;
        let b = a + std::mem::size_of::<A>();
        return vec![a as voidptr, b as voidptr];
    }
}
impl<A,B,C> TupleToPtrs for (A,B,C){
    fn get_ptrs(ptr:*mut Self)->Vec<*mut c_void>{
        let a = ptr as usize;
        let b = a + std::mem::size_of::<A>();
        let c = b + std::mem::size_of::<B>();
        return vec![a as voidptr, b as voidptr,c as voidptr];
    }
}
impl<A,B,C,D> TupleToPtrs for (A,B,C,D){
    fn get_ptrs(ptr:*mut Self)->Vec<*mut c_void>{
        let a = ptr as usize;
        let b = a + std::mem::size_of::<A>();
        let c = b + std::mem::size_of::<B>();
        let d = c + std::mem::size_of::<C>();
        return vec![a as voidptr, b as voidptr,c as voidptr,d as voidptr];
    }
}
impl<A,B,C,D,E> TupleToPtrs for (A,B,C,D,E){
    fn get_ptrs(ptr:*mut Self)->Vec<*mut c_void>{
        let a = ptr as usize;
        let b = a + std::mem::size_of::<A>();
        let c = b + std::mem::size_of::<B>();
        let d = c + std::mem::size_of::<C>();
        let e = d + std::mem::size_of::<D>();
        return vec![a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr];
    }
}
impl<A,B,C,D,E,F> TupleToPtrs for (A,B,C,D,E,F){
    fn get_ptrs(ptr:*mut Self)->Vec<*mut c_void>{
        let a = ptr as usize;
        let b = a + std::mem::size_of::<A>();
        let c = b + std::mem::size_of::<B>();
        let d = c + std::mem::size_of::<C>();
        let e = d + std::mem::size_of::<D>();
        let f = e + std::mem::size_of::<E>();
        return vec![a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr,f as voidptr];
    }
}
impl<A,B,C,D,E,F,G> TupleToPtrs for (A,B,C,D,E,F,G){
    fn get_ptrs(ptr:*mut Self)->Vec<*mut c_void>{
        let a = ptr as usize;
        let b = a + std::mem::size_of::<A>();
        let c = b + std::mem::size_of::<B>();
        let d = c + std::mem::size_of::<C>();
        let e = d + std::mem::size_of::<D>();
        let f = e + std::mem::size_of::<E>();
        let g = f + std::mem::size_of::<F>();
        return vec![a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr,f as voidptr,g as voidptr];
    }
}
impl<A,B,C,D,E,F,G,H> TupleToPtrs for (A,B,C,D,E,F,G,H){
    fn get_ptrs(ptr:*mut Self)->Vec<*mut c_void>{
            let a = ptr as usize;
            let b = a + std::mem::size_of::<A>();
            let c = b + std::mem::size_of::<B>();
            let d = c + std::mem::size_of::<C>();
            let e = d + std::mem::size_of::<D>();
            let f = e + std::mem::size_of::<E>();
            let g = f + std::mem::size_of::<F>();
            let h = g + std::mem::size_of::<G>();
            return vec![a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr,f as voidptr,g as voidptr,h as voidptr];
    }
}
impl<A,B,C,D,E,F,G,H,I> TupleToPtrs for (A,B,C,D,E,F,G,H,I){
    fn get_ptrs(ptr:*mut Self)->Vec<*mut c_void>{
            let a = ptr as usize;
            let b = a + std::mem::size_of::<A>();
            let c = b + std::mem::size_of::<B>();
            let d = c + std::mem::size_of::<C>();
            let e = d + std::mem::size_of::<D>();
            let f = e + std::mem::size_of::<E>();
            let g = f + std::mem::size_of::<F>();
            let h = g + std::mem::size_of::<G>();
            let i = h + std::mem::size_of::<H>();
            return vec![a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr,f as voidptr,
            g as voidptr,h as voidptr,i as voidptr];
    }
}
impl<A,B,C,D,E,F,G,H,I,J> TupleToPtrs for (A,B,C,D,E,F,G,H,I,J){
    fn get_ptrs(ptr:*mut Self)->Vec<*mut c_void>{
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
            return vec![a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr,f as voidptr,
            g as voidptr,h as voidptr,i as voidptr,j as voidptr];
    }
}
impl<A,B,C,D,E,F,G,H,I,J,K> TupleToPtrs for (A,B,C,D,E,F,G,H,I,J,K){
    fn get_ptrs(ptr:*mut Self)->Vec<*mut c_void>{
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
            return vec![a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr,f as voidptr,
            g as voidptr,h as voidptr,i as voidptr,j as voidptr,k as voidptr];
    }
} 
impl<A,B,C,D,E,F,G,H,I,J,K,L> TupleToPtrs for (A,B,C,D,E,F,G,H,I,J,K,L){
    fn get_ptrs(ptr:*mut Self)->Vec<*mut c_void>{
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
            return vec![a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr,f as voidptr,
            g as voidptr,h as voidptr,i as voidptr,j as voidptr,k as voidptr,l as voidptr];
    }
} 
impl<A,B,C,D,E,F,G,H,I,J,K,L,M> TupleToPtrs for (A,B,C,D,E,F,G,H,I,J,K,L,M){
    fn get_ptrs(ptr:*mut Self)->Vec<*mut c_void>{
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
            return vec![a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr,f as voidptr,
            g as voidptr,h as voidptr,i as voidptr,j as voidptr,k as voidptr,l as voidptr,m as voidptr];
    }
} 
impl<A,B,C,D,E,F,G,H,I,J,K,L,M,N> TupleToPtrs for (A,B,C,D,E,F,G,H,I,J,K,L,M,N){
    fn get_ptrs(ptr:*mut Self)->Vec<*mut c_void>{
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
            return vec![a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr,f as voidptr,
            g as voidptr,h as voidptr,i as voidptr,j as voidptr,k as voidptr,l as voidptr,m as voidptr,
            n as voidptr];
    }
} 
impl<A,B,C,D,E,F,G,H,I,J,K,L,M,N,O> TupleToPtrs for (A,B,C,D,E,F,G,H,I,J,K,L,M,N,O){
    fn get_ptrs(ptr:*mut Self)->Vec<*mut c_void>{
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
            return vec![a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr,f as voidptr,
            g as voidptr,h as voidptr,i as voidptr,j as voidptr,k as voidptr,l as voidptr,m as voidptr,
            n as voidptr,o as voidptr];
    }
} 
impl<A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P> TupleToPtrs for (A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P){
    fn get_ptrs(ptr:*mut Self)->Vec<*mut c_void>{
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
            return vec![a as voidptr, b as voidptr,c as voidptr,d as voidptr,e as voidptr,f as voidptr,
            g as voidptr,h as voidptr,i as voidptr,j as voidptr,k as voidptr,l as voidptr,m as voidptr,
            n as voidptr,o as voidptr,p as voidptr];
    }
} 