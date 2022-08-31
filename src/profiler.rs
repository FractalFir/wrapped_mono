use crate::binds::{MonoProfilerHandle,MonoProfiler,_MonoProfiler};
use std::ptr::null_mut; 
struct _Profiler<T>{
    handle:MonoProfilerHandle,
    rtime_init_cb:Option<fn (profiler:&mut Profiler<T>)>,
    cleanup_cb:Option<fn (profiler:&mut Profiler<T>)>,
    pub data:T,
} 
impl<T> _Profiler<T>{
    pub fn create(data:T)->*mut Self{
        use std::alloc::{alloc, dealloc, Layout};
        let ptr = unsafe{
            let ptr = alloc(Layout::new::<Self>()) as *mut Self;
            (*ptr).rtime_init_cb = None;
            (*ptr).cleanup_cb = None;
            (*ptr).handle = crate::binds::mono_profiler_create(ptr as *mut MonoProfiler);
            (*ptr).data = data;
            ptr
        };
        return ptr;
    }
    pub fn destroy(profiler:*mut Self){
        use std::alloc::{alloc, dealloc, Layout};
        let prof = unsafe{&mut *profiler};
        match prof.rtime_init_cb{
            Some(_)=>unsafe{crate::binds::mono_profiler_set_runtime_initialized_callback(prof.handle,None)},
            None=>(),
        }
        match prof.cleanup_cb{
            Some(_)=>unsafe{crate::binds::mono_profiler_set_cleanup_callback(prof.handle,None)},
            None=>(),
        }
        unsafe{dealloc(profiler as *mut u8, Layout::new::<Self>())};
    }
    //#####################################################################
    //Runtime init
    unsafe extern "C" fn runtime_init_callback(profiler:*mut _Profiler<T>){
        let this = &mut *(profiler);
        match this.rtime_init_cb{
            Some(cb)=>{
                let cb = cb(&mut Profiler::<T>::from_ptr(profiler as *mut MonoProfiler));
            }
            None=>(),
        }
    }
    pub fn remove_rtime_init_cb(&mut self){
        //Check if another callback has been registered and if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_runtime_initialized_callback(self.handle,None);
            self.rtime_init_cb = None;
        }
    }
    pub fn add_rtime_init_cb(&mut self,cb:fn (profiler:&mut Profiler<T>)){
        //Check if another callback has been registered and if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_runtime_initialized_callback(self.handle,
                std::mem::transmute::<unsafe extern "C" fn(*mut _Profiler<T>),Option<unsafe extern "C" fn(*mut _MonoProfiler)>>
                (Self::runtime_init_callback as unsafe extern "C" fn(*mut _Profiler<T>)));
            self.rtime_init_cb = Some(cb);
        }
    }
    //################################################################
    //Cleanup
    unsafe extern "C" fn cleanup_callback(profiler:*mut _Profiler<T>){
        let this = &mut *(profiler);
        match this.cleanup_cb{
            Some(cb)=>{
                let cb = cb(&mut Profiler::<T>::from_ptr(profiler as *mut MonoProfiler));
            }
            None=>(),
        }
    }
    pub fn remove_cleanup_cb(&mut self){
        //Check if another callback has been registered and if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_cleanup_callback(self.handle,None);
            self.rtime_init_cb = None;
        }
    }
    pub fn add_cleanup_cb(&mut self,cb:fn (profiler:&mut Profiler<T>)){
        //Check if another callback has been registered and if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_cleanup_callback(self.handle,
                std::mem::transmute::<unsafe extern "C" fn(*mut _Profiler<T>),Option<unsafe extern "C" fn(*mut _MonoProfiler)>>
                (Self::cleanup_callback as unsafe extern "C" fn(*mut _Profiler<T>)));
            self.rtime_init_cb = Some(cb);
        }
    }
}
/// A structure representing a profiler with custom user data. This structure will be passed when callbacks are called. No more than one callback per profiler can be registered.
pub struct Profiler<T>{
    ptr:*mut _Profiler<T>,
}
impl<T> Profiler<T>{
    fn from_ptr(profiler:*mut MonoProfiler)->Self{
        return Self{ptr:profiler as *mut _Profiler<T>};
    }
    ///Creates a new profiler with *data*. Data is user defined struct used to pass data to callbacks.
    pub fn create(data:T)->Profiler<T>{
        return Self{ptr:_Profiler::<T>::create(data)};
    }
    /// Destroys the profiler.
    /// # Safety 
    /// makes all of copies of Profiler invalid
    pub fn destroy(self){
        _Profiler::<T>::destroy(self.ptr);
    }
    ///Returns refernece to internal data.
    pub fn get_internal_data(&mut self)->&mut T{
        return unsafe{(&mut (*self.ptr).data)};
    }
    ///Removes callback added by [`add_runtime_initialized_callback`]
    pub fn remove_runtime_initialized_callback(&mut self){
        unsafe{(*self.ptr).remove_rtime_init_cb()};
    }
    ///Adds callback to be called when runtime is started.
    pub fn add_runtime_initialized_callback(&mut self,cb: fn (profiler:&mut Profiler<T>)){
       unsafe{(*self.ptr).add_rtime_init_cb(cb)};
    }
    ///Removes callback added by [`add_cleanup_callback`]
    pub fn remove_cleanup_callback(&mut self){
        unsafe{(*self.ptr).remove_rtime_init_cb()};
    }
    ///Adds callback to be called when runtime is started.
    pub fn add_cleanup_callback(&mut self,cb: fn (profiler:&mut Profiler<T>)){
       unsafe{(*self.ptr).add_rtime_init_cb(cb)};
    }

}