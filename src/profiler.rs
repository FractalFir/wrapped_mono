use crate::binds::{MonoProfilerHandle,MonoProfiler,_MonoProfiler};
use std::ptr::null_mut;
struct _Profiler<T>{
    handle:MonoProfilerHandle,
    rtime_init_cb:Option<fn (profiler:&mut Profiler<T>)>,
    data:Box<T>,
} 
impl<T> _Profiler<T>{
    pub fn create(data:Box<T>)->*mut Self{
        use std::alloc::{alloc, dealloc, Layout};
        let ptr = unsafe{
            let ptr = alloc(Layout::new::<Self>()) as *mut Self;
            (*ptr).data = data;
            *(&mut (*ptr).rtime_init_cb as *mut Option<for<'r> fn(&'r mut Profiler<T>)> as *mut usize) = 0;
            (*ptr).handle = crate::binds::mono_profiler_create(ptr as *mut MonoProfiler);
            ptr
        };
        return ptr;
    }
    pub fn destroy(profiler:*mut Self){
        use std::alloc::{alloc, dealloc, Layout};
        unsafe{dealloc(profiler as *mut u8, Layout::new::<Self>())};
    }
    unsafe extern "C" fn runtime_init_callback(profiler:*mut _Profiler<T>){
        let this = &mut *(profiler);
        match this.rtime_init_cb{
            Some(cb)=>{
                let cb = cb(&mut Profiler::<T>::from_ptr(profiler as *mut MonoProfiler));
            }
            None=>panic!("No callback!"),
        }
    }
    pub fn set_rtime_init_cb(&mut self,cb:fn (profiler:&mut Profiler<T>)){
        unsafe{
            crate::binds::mono_profiler_set_runtime_initialized_callback(self.handle,
                std::mem::transmute::<unsafe extern "C" fn(*mut _Profiler<T>),Option<unsafe extern "C" fn(*mut _MonoProfiler)>>
                (Self::runtime_init_callback as unsafe extern "C" fn(*mut _Profiler<T>)));
            self.rtime_init_cb = Some(cb);
        }
    }
}
pub struct Profiler<T>{
    ptr:*mut _Profiler<T>,
}
impl<T> Profiler<T>{
    fn from_ptr(profiler:*mut MonoProfiler)->Self{
        return Self{ptr:profiler as *mut _Profiler<T>};
    }
    fn get_handle(&self)->MonoProfilerHandle{
        return unsafe{(*self.ptr).handle};
    }
    ///Creates a new profiler with *data*. Data is user defined struct used to pass data to callbacks.
    pub fn create(data:T)->Profiler<T>{
        let dbox = Box::new(data);
        return Self{ptr:_Profiler::<T>::create(dbox)};
    }
    ///Destroys the profiler.
    /// # Safety 
    /// makes all of copies of Profiler invalid invalid
    pub fn destroy(self){
        _Profiler::<T>::destroy(self.ptr);
    }

    pub fn set_runtime_initialized_callback(&self,cb: fn (profiler:&mut Profiler<T>)){
       unsafe{(*self.ptr).set_rtime_init_cb(cb)};
    }
}