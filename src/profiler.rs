use crate::binds::{MonoProfilerHandle,MonoProfiler,_MonoProfiler,MonoProfilerCallContext};
use crate::{Object,Domain,Method,MethodTrait};
use crate::interop::InteropRecive;
// Profiler is not finished so it has some dead code that will fixed addressed in the future TODO: fix profiler dead code
struct _Profiler<T>{
    handle:MonoProfilerHandle,
    rtime_init_cb:Option<fn (profiler:&mut Profiler<T>)>,
    rtime_shutdown_begin_cb:Option<fn (profiler:&mut Profiler<T>)>,
    rtime_shutdown_end_cb:Option<fn (profiler:&mut Profiler<T>)>,
    context_loaded_cb:Option<fn (profiler:&mut Profiler<T>)>,
    cleanup_cb:Option<fn (profiler:&mut Profiler<T>)>,
    context_unloaded_cb:Option<fn (profiler:&mut Profiler<T>)>,
    domain_loading_cb:Option<fn (profiler:&mut Profiler<T>,dom:&mut Domain)>,
    domain_loaded_cb:Option<fn (profiler:&mut Profiler<T>,dom:&mut Domain)>,
    domain_unloading_cb:Option<fn (profiler:&mut Profiler<T>,dom:&mut Domain)>,
    domain_unloaded_cb:Option<fn (profiler:&mut Profiler<T>,dom:&mut Domain)>,
    domain_set_name_cb:Option<fn (profiler:&mut Profiler<T>,dom:&mut Domain,&str)>,
    //TODO: fix jit_negin_cb: main method has either type Method<Object,iszie,isize,isize> or Method<String> signature.
    #[allow(clippy::type_complexity)]
    jit_begin_cb:Option<fn (profiler:&mut Profiler<T>,&Method<String>)>,
    pub data:T,
} 
pub struct ProfilerCallContext{
    ptr:*mut MonoProfilerCallContext,
}
impl ProfilerCallContext{
    pub fn get_this(&self)->Option<Object>{
        unsafe{Object::from_ptr(crate::binds::mono_profiler_call_context_get_this(self.ptr) as *mut crate::binds::MonoObject)}
    }
    ///Returns *index* argument of current call context. *index* must be within argument count of current method. Type must match argument.
    pub fn get_arg<T:InteropRecive>(&self,index:u32)->T{
        let ptr = unsafe{crate::binds:: mono_profiler_call_context_get_argument(self.ptr,index)} as *const <T as InteropRecive>::SourceType;
        let src:<T as InteropRecive>::SourceType = unsafe{*(ptr)};
        <T as InteropRecive>::get_rust_rep(src)
    }
    ///Return local argument from current call contex at *index*. Index must be valid and type must match.
    pub fn get_local<T:InteropRecive>(&self,index:u32)->T{
        let ptr = unsafe{crate::binds::mono_profiler_call_context_get_local(self.ptr,index)} as *const <T as InteropRecive>::SourceType;
        let src:<T as InteropRecive>::SourceType = unsafe{*(ptr)};
        <T as InteropRecive>::get_rust_rep(src)
    }
}
use std::alloc::{alloc, dealloc, Layout};
impl<T> _Profiler<T>{
    pub fn create(mut data:T)->*mut Self{
        unsafe{
            let ptr = alloc(Layout::new::<Self>());
            for i in 0..std::mem::size_of::<Self>(){
                *((ptr as usize + i) as *mut u8) = 0;
            }
            let ptr = ptr as *mut Self;
            (*ptr).handle = crate::binds::mono_profiler_create(ptr as *mut MonoProfiler);
            let src:&mut T = &mut data;
            let dst:&mut T = &mut (*ptr).data;
            std::mem::swap(src,dst);
            std::mem::forget(data);
            ptr
        }
    }
    pub fn destroy(profiler:*mut Self){
        let prof = unsafe{&mut *profiler};
        Self::remove_rtime_init_cb(prof);
        Self::remove_jit_begin_cb(prof);
        Self::remove_domain_loaded_cb(prof);
        Self::remove_domain_loading_cb(prof);
        Self::remove_domain_name_cb(prof);
        unsafe{dealloc(profiler as *mut u8, Layout::new::<Self>())};
    }
    //#####################################################################
    //Runtime init
    unsafe extern "C" fn runtime_init_callback(profiler:*mut _Profiler<T>){
        let this = &mut *(profiler);
        match this.rtime_init_cb{
            Some(cb)=>{
                let mut prof = Profiler::<T>::from_ptr(profiler as *mut MonoProfiler);
                cb(&mut prof);
                println!("Finished calling runtime init callback!");
            }
            None=>panic!("Invalid callback registration state. Callback registered for handler, yet handler has no callback function to call!"),
        }
    }
    pub fn remove_rtime_init_cb(&mut self){
        //Check if another callback has been registered and if so, remove it.
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
    //TODO: Check why cleanup callback colides with runtime_init callback and renable it.
    /* 
    //################################################################
    //Cleanup
    unsafe extern "C" fn cleanup_callback(profiler:*mut _Profiler<T>){
        let this = &mut *(profiler);
        println!("Clenup callback preparation!");
        match this.cleanup_cb{
            Some(cb)=>{
                cb(&mut Profiler::<T>::from_ptr(profiler as *mut MonoProfiler));
            }
            None=>panic!("Invalid callback registration state. Callback registered for handler, yet handler has no callback function to call!"),
        }
        println!("Clenup callback finished!");
    }
    pub fn remove_cleanup_cb(&mut self){
        //Check if another callback has been registered and if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_cleanup_callback(self.handle,None);
            self.cleanup_cb = None;
        }
    }
    pub fn add_cleanup_cb(&mut self,cb:fn (profiler:&mut Profiler<T>)){
        unsafe{
            crate::binds::mono_profiler_set_cleanup_callback(self.handle,
                std::mem::transmute::<unsafe extern "C" fn(*mut _Profiler<T>),Option<unsafe extern "C" fn(*mut _MonoProfiler)>>
                (Self::cleanup_callback as unsafe extern "C" fn(*mut _Profiler<T>)));
            self.cleanup_cb = Some(cb);
        }
    }*/
    //##################################################
    //Runtime shutdown begin callback.
    unsafe extern "C" fn runtime_shutown_begin_callback(profiler:*mut _Profiler<T>){
        let this = &mut *(profiler);
        match this.rtime_shutdown_begin_cb{
            Some(cb)=>{
                let mut prof = Profiler::<T>::from_ptr(profiler as *mut MonoProfiler);
                cb(&mut prof);
            },
            None=>panic!("Invalid callback registration state. Callback registered for handler, yet handler has no callback function to call!"),
        }
    }
    pub fn remove_runtime_shutdown_begin_cb(&mut self){
        //Check if another callback has been registered and if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_runtime_shutdown_begin_callback(self.handle,None);
            self.rtime_shutdown_begin_cb = None;
        }
    }
    pub fn add_runtime_shutdown_begin_cb(&mut self,cb:fn (profiler:&mut Profiler<T>)){
        //Check if another callback has been registered and if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_runtime_shutdown_begin_callback(self.handle,
                std::mem::transmute::<unsafe extern "C" fn(*mut _Profiler<T>),Option<unsafe extern "C" fn(*mut _MonoProfiler)>>
                (Self::runtime_shutown_begin_callback as unsafe extern "C" fn(*mut _Profiler<T>)));
            self.rtime_shutdown_begin_cb = Some(cb);
        }
    } 
    //##################################################
    //Runtime shutdown end callback.
    unsafe extern "C" fn runtime_shutown_end_callback(profiler:*mut _Profiler<T>){
        let this = &mut *(profiler);
        match this.rtime_shutdown_end_cb{
            Some(cb)=>{
                let mut prof = Profiler::<T>::from_ptr(profiler as *mut MonoProfiler);
                cb(&mut prof);
            },
            None=>panic!("Invalid callback registration state. Callback registered for handler, yet handler has no callback function to call!"),
        }
    }
    pub fn remove_runtime_shutdown_end_cb(&mut self){
        //Check if another callback has been registered and if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_runtime_shutdown_end_callback(self.handle,None);
            self.rtime_shutdown_end_cb = None;
        }
    }
    pub fn add_runtime_shutdown_end_cb(&mut self,cb:fn (profiler:&mut Profiler<T>)){
        //Check if another callback has been registered and if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_runtime_shutdown_end_callback(self.handle,
                std::mem::transmute::<unsafe extern "C" fn(*mut _Profiler<T>),Option<unsafe extern "C" fn(*mut _MonoProfiler)>>
                (Self::runtime_shutown_end_callback as unsafe extern "C" fn(*mut _Profiler<T>)));
            self.rtime_shutdown_end_cb = Some(cb);
        }
    } 
    //##################################################
    //Context loaded callback
    unsafe extern "C" fn context_loaded_callback(profiler:*mut _Profiler<T>,_:*mut crate::binds::MonoAppContext){
        let this = &mut *(profiler);
        match this.context_loaded_cb{
            Some(cb)=>{
                let mut prof = Profiler::<T>::from_ptr(profiler as *mut MonoProfiler);
                cb(&mut prof);
            },
            None=>panic!("Invalid callback registration state. Callback registered for handler, yet handler has no callback function to call!"),
        }
    }
    pub fn remove_context_loaded_cb(&mut self){
        //Check if another callback has been registered and if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_context_loaded_callback(self.handle,None);
            self.context_loaded_cb = None;
        }
    }
    pub fn add_context_loaded_cb(&mut self,cb:fn (profiler:&mut Profiler<T>)){
        //Check if another callback has been registered and if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_context_loaded_callback(self.handle,
                std::mem::transmute::<unsafe extern "C" fn(*mut _Profiler<T>,*mut crate::binds::MonoAppContext),Option<unsafe extern "C" fn(*mut _MonoProfiler,*mut crate::binds::MonoAppContext)>>
                (Self::context_loaded_callback as unsafe extern "C" fn(*mut _Profiler<T>,*mut crate::binds::MonoAppContext)));
            self.context_loaded_cb = Some(cb);
        }
    } 
    //##################################################
    //Context unloaded callback
    unsafe extern "C" fn context_unloaded_callback(profiler:*mut _Profiler<T>,_:*mut crate::binds::MonoAppContext){
        let this = &mut *(profiler);
        match this.context_unloaded_cb{
            Some(cb)=>{
                let mut prof = Profiler::<T>::from_ptr(profiler as *mut MonoProfiler);
                cb(&mut prof);
            },
            None=>panic!("Invalid callback registration state. Callback registered for handler, yet handler has no callback function to call!"),
        }
    }
    pub fn remove_context_unloaded_cb(&mut self){
        //Check if another callback has been registered and if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_context_unloaded_callback(self.handle,None);
            self.context_unloaded_cb = None;
        }
    }
    pub fn add_context_unloaded_cb(&mut self,cb:fn (profiler:&mut Profiler<T>)){
        //Check if another callback has been registered and if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_context_unloaded_callback(self.handle,
                std::mem::transmute::<unsafe extern "C" fn(*mut _Profiler<T>,*mut crate::binds::MonoAppContext),Option<unsafe extern "C" fn(*mut _MonoProfiler,*mut crate::binds::MonoAppContext)>>
                (Self::context_unloaded_callback as unsafe extern "C" fn(*mut _Profiler<T>,*mut crate::binds::MonoAppContext)));
            self.context_unloaded_cb = Some(cb);
        }
    } 
    //##################################################
    //Domain loading callback
    unsafe extern "C" fn domain_loading_callback(profiler:*mut _Profiler<T>,dom:*mut crate::binds::MonoDomain){
        let this = &mut *(profiler);
        let mut dom = Domain::from_ptr(dom);
        match this.domain_loading_cb{
            Some(cb)=>{
                let mut prof = Profiler::<T>::from_ptr(profiler as *mut MonoProfiler);
                cb(&mut prof,&mut dom);
            },
            None=>panic!("Invalid callback registration state. Callback registered for handler, yet handler has no callback function to call!"),
        }
    }
    pub fn remove_domain_loading_cb(&mut self){
        //Check if another callback has been registered and if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_domain_loading_callback(self.handle,None);
            self.domain_loading_cb = None;
        }
    }
    pub fn add_domain_loading_cb(&mut self,cb:fn (profiler:&mut Profiler<T>,dom:&mut Domain)){
        //Check if another callback has been registered and if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_domain_loading_callback(self.handle,
                std::mem::transmute::<unsafe extern "C" fn(*mut _Profiler<T>,*mut crate::binds::MonoDomain),Option<unsafe extern "C" fn(*mut _MonoProfiler,*mut crate::binds::MonoDomain)>>
                (Self::domain_loading_callback as unsafe extern "C" fn(*mut _Profiler<T>,*mut crate::binds::MonoDomain)));
            self.domain_loading_cb = Some(cb);
        }
    } 
    //##################################################
    //Domain loaded callback
    unsafe extern "C" fn domain_loaded_callback(profiler:*mut _Profiler<T>,dom:*mut crate::binds::MonoDomain){
        let this = &mut *(profiler);
        let mut dom = Domain::from_ptr(dom);
        match this.domain_loaded_cb{
            Some(cb)=>{
                let mut prof = Profiler::<T>::from_ptr(profiler as *mut MonoProfiler);
                cb(&mut prof,&mut dom);
            },
            None=>panic!("Invalid callback registration state. Callback registered for handler, yet handler has no callback function to call!"),
        }
    }
    pub fn remove_domain_loaded_cb(&mut self){
        //Check if another callback has been registered and if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_domain_loaded_callback(self.handle,None);
            self.domain_loaded_cb = None;
        }
    }
    pub fn add_domain_loaded_cb(&mut self,cb:fn (profiler:&mut Profiler<T>,dom:&mut Domain)){
        //Check if another callback has been registered and if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_domain_loaded_callback(self.handle,
                std::mem::transmute::<unsafe extern "C" fn(*mut _Profiler<T>,*mut crate::binds::MonoDomain),Option<unsafe extern "C" fn(*mut _MonoProfiler,*mut crate::binds::MonoDomain)>>
                (Self::domain_loaded_callback as unsafe extern "C" fn(*mut _Profiler<T>,*mut crate::binds::MonoDomain)));
            self.domain_loaded_cb = Some(cb);
        }
    } 
    //##################################################
    //Domain unloading callback
    unsafe extern "C" fn domain_unloading_callback(profiler:*mut _Profiler<T>,dom:*mut crate::binds::MonoDomain){
        let this = &mut *(profiler);
        let mut dom = Domain::from_ptr(dom);
        match this.domain_unloading_cb{
            Some(cb)=>{
                let mut prof = Profiler::<T>::from_ptr(profiler as *mut MonoProfiler);
                cb(&mut prof,&mut dom);
            },
            None=>panic!("Invalid callback registration state. Callback registered for handler, yet handler has no callback function to call!"),
        }
    }
    pub fn remove_domain_unloading_cb(&mut self){
        //Check if another callback has been registered and if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_domain_unloading_callback(self.handle,None);
            self.domain_unloading_cb = None;
        }
    }
    pub fn add_domain_unloading_cb(&mut self,cb:fn (profiler:&mut Profiler<T>,dom:&mut Domain)){
        //Check if another callback has been registered and if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_domain_unloading_callback(self.handle,
                std::mem::transmute::<unsafe extern "C" fn(*mut _Profiler<T>,*mut crate::binds::MonoDomain),Option<unsafe extern "C" fn(*mut _MonoProfiler,*mut crate::binds::MonoDomain)>>
                (Self::domain_unloading_callback as unsafe extern "C" fn(*mut _Profiler<T>,*mut crate::binds::MonoDomain)));
            self.domain_unloading_cb = Some(cb);
        }
    } 
    //##################################################
    //Domain unloaded callback
    unsafe extern "C" fn domain_unloaded_callback(profiler:*mut _Profiler<T>,dom:*mut crate::binds::MonoDomain){
        let this = &mut *(profiler);
        let mut dom = Domain::from_ptr(dom);
        match this.domain_unloaded_cb{
            Some(cb)=>{
                let mut prof = Profiler::<T>::from_ptr(profiler as *mut MonoProfiler);
                cb(&mut prof,&mut dom);
            },
            None=>panic!("Invalid callback registration state. Callback registered for handler, yet handler has no callback function to call!"),
        }
    }
    pub fn remove_domain_unloaded_cb(&mut self){
        //Check if another callback has been registered and if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_domain_unloaded_callback(self.handle,None);
            self.domain_unloaded_cb = None;
        }
    }
    pub fn add_domain_unloaded_cb(&mut self,cb:fn (profiler:&mut Profiler<T>,dom:&mut Domain)){
        //Check if another callback has been registered and if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_domain_unloaded_callback(self.handle,
                std::mem::transmute::<unsafe extern "C" fn(*mut _Profiler<T>,*mut crate::binds::MonoDomain),Option<unsafe extern "C" fn(*mut _MonoProfiler,*mut crate::binds::MonoDomain)>>
                (Self::domain_unloaded_callback as unsafe extern "C" fn(*mut _Profiler<T>,*mut crate::binds::MonoDomain)));
            self.domain_unloaded_cb = Some(cb);
        }
    } 
    //##################################################
    //Domain set name callback
    unsafe extern "C" fn domain_name_callback(profiler:*mut _Profiler<T>,dom:*mut crate::binds::MonoDomain,str_ptr:*const i8){
        let this = &mut *(profiler);
        let cstr = CString::from_raw(str_ptr as *mut i8);
        use std::ffi::CString;
        let st = cstr.to_str().expect("Could not create String!").to_owned();
        let mut dom = Domain::from_ptr(dom);
        match this.domain_set_name_cb{
            Some(cb)=>{
                let mut prof = Profiler::<T>::from_ptr(profiler as *mut MonoProfiler);
                cb(&mut prof,&mut dom,&st);
            },
            None=>panic!("Invalid callback registration state. Callback registered for handler, yet handler has no callback function to call!"),
        }
        let _ = cstr.into_raw();
    }
    pub fn remove_domain_name_cb(&mut self){
        //Check if another callback has been registered and if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_domain_name_callback(self.handle,None);
            self.domain_set_name_cb = None;
        }
    }
    pub fn add_domain_name_cb(&mut self,cb:fn (profiler:&mut Profiler<T>,dom:&mut Domain,name:&str)){
        //Check if another callback has been registered and if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_domain_name_callback(self.handle,
                std::mem::transmute::<unsafe extern "C" fn(*mut _Profiler<T>,*mut crate::binds::MonoDomain,*const i8),Option<unsafe extern "C" fn(*mut _MonoProfiler,*mut crate::binds::MonoDomain,*const i8)>>
                (Self::domain_name_callback as unsafe extern "C" fn(*mut _Profiler<T>,*mut crate::binds::MonoDomain,*const i8)));
            self.domain_set_name_cb = Some(cb);
        }
    }
    //##################################################
    //Domain set jit begin
    unsafe extern "C" fn jit_begin_callback(profiler:*mut _Profiler<T>,met:*mut crate::binds::MonoMethod){
        let this = &mut *(profiler);
        let method:Method<String> = Method::from_ptr(met).expect("Could not get jit main method while executing jit begin. This is an internal profiler error.");
        match this.jit_begin_cb{
            Some(cb)=>{
                let mut prof = Profiler::<T>::from_ptr(profiler as *mut MonoProfiler);
                cb(&mut prof,&method);
            },
            None=>panic!("Invalid callback registration state. Callback registered for hindler, yet hindler has no callback function to call!"),
        }
    }
    pub fn remove_jit_begin_cb(&mut self){
        //Check if another callback has been registered ind if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_jit_begin_callback(self.handle,None);
            self.jit_begin_cb = None;
        }
    }
    pub fn add_jit_begin_cb(&mut self,cb:fn (profiler:&mut Profiler<T>,&Method<String>)){
        //Check if another callback has been registered ind if so, renove it.
        unsafe{
            crate::binds::mono_profiler_set_jit_begin_callback(self.handle,
                std::mem::transmute::<unsafe extern "C" fn(*mut _Profiler<T>,*mut crate::binds::MonoMethod),
                Option<unsafe extern "C" fn(*mut _MonoProfiler,*mut crate::binds::MonoMethod)>>
                (Self::jit_begin_callback as unsafe extern "C" fn(*mut _Profiler<T>,*mut crate::binds::MonoMethod)));
            self.jit_begin_cb = Some(cb);
        }
    } 
}
//impliment mono_profiler_set_coverage_filter_callback
/// A structure representing a profiler with custom user data. This structure will be passed when callbacks are called. No more than one callback per profiler can be registered.
pub struct Profiler<T>{
    ptr:*mut _Profiler<T>,
}
impl<T> Profiler<T>{
    fn from_ptr(profiler:*mut MonoProfiler)->Self{
        Self{ptr:profiler as *mut _Profiler<T>}
    }
    ///Creates a new profiler with *data*. Data is user defined struct used to pass data to callbacks.
    pub fn create(data:T)->Profiler<T>{
        Self{ptr:_Profiler::<T>::create(data)}
    }
    /// Destroys the profiler.
    /// # Safety 
    /// makes all of copies of Profiler invalid
    pub fn destroy(self){
        _Profiler::<T>::destroy(self.ptr);
    }
    ///Returns refernece to internal data.
    pub fn get_internal_data(&mut self)->&mut T{
        unsafe{&mut (*self.ptr).data}
    }
    ///Removes callback added by [`add_runtime_initialized_callback`]
    pub fn remove_runtime_initialized_callback(&mut self){
        unsafe{(*self.ptr).remove_rtime_init_cb()};
    }
    ///Adds callback to be called when runtime is started.
    pub fn add_runtime_initialized_callback(&mut self,cb: fn (profiler:&mut Profiler<T>)){
       unsafe{(*self.ptr).add_rtime_init_cb(cb)};
    }
    /*
    ///Removes callback added by [`add_cleanup_callback`]
    pub fn remove_cleanup_callback(&mut self){
        unsafe{(*self.ptr).remove_rtime_init_cb()};
    }
    ///Adds callback to be called when runtime is started.
    pub fn add_cleanup_callback(&mut self,cb: fn (profiler:&mut Profiler<T>)){
       unsafe{(*self.ptr).add_rtime_init_cb(cb)};
    }*/
    ///Removes callback added by [`add_runtime_shutown_begin_callback`]
    pub fn remove_runtime_shutown_begin_callback(&mut self){
        unsafe{(*self.ptr).remove_runtime_shutdown_begin_cb()};
    }
    ///Adds callback to be called when runtime is started.
    pub fn add_runtime_shutown_begin_callback(&mut self,cb: fn (profiler:&mut Profiler<T>)){
        unsafe{(*self.ptr).add_runtime_shutdown_begin_cb(cb)};
    }
    ///Removes callback added by [`add_runtime_shutown_end_callback`]
    pub fn remove_runtime_shutown_end_callback(&mut self){
        unsafe{(*self.ptr).remove_runtime_shutdown_end_cb()};
    }
    ///Adds callback to be called when runtime is started.
    pub fn add_runtime_shutown_end_callback(&mut self,cb: fn (profiler:&mut Profiler<T>)){
        unsafe{(*self.ptr).add_runtime_shutdown_end_cb(cb)};
    }
    ///Removes callback added by [`add_context_loaded`]
    pub fn remove_context_loaded(&mut self){
        unsafe{(*self.ptr).remove_context_loaded_cb()};
    }
    ///Adds callback to be called when runtime is started.
    pub fn add_context_loaded(&mut self,cb: fn (profiler:&mut Profiler<T>)){
        unsafe{(*self.ptr).add_context_loaded_cb(cb)};
    }
    ///Removes callback added by [`add_context_unloaded`]
    pub fn remove_context_unloaded(&mut self){
        unsafe{(*self.ptr).remove_context_unloaded_cb()};
    }
    ///Adds callback to be called when runtime is started.
    pub fn add_context_unloaded(&mut self,cb: fn (profiler:&mut Profiler<T>)){
        unsafe{(*self.ptr).add_context_unloaded_cb(cb)};
    }
    ///Removes callback added by [`add_domain_loading`]
    pub fn remove_domain_loading(&mut self){
        unsafe{(*self.ptr).remove_domain_loading_cb()};
    }
    ///Adds callback to be called when runtime is started.
    pub fn add_domain_loading(&mut self,cb: fn (profiler:&mut Profiler<T>,&mut Domain)){
        unsafe{(*self.ptr).add_domain_loading_cb(cb)};
    }
    ///Removes callback added by [`add_domain_loading`]
    pub fn remove_domain_loaded(&mut self){
        unsafe{(*self.ptr).remove_domain_loading_cb()};
    }
    ///Adds callback to be called when runtime is started.
    pub fn add_domain_loaded(&mut self,cb: fn (profiler:&mut Profiler<T>,&mut Domain)){
        unsafe{(*self.ptr).add_domain_loading_cb(cb)};
    }
    ///Removes callback added by [`add_domain_unloading`]
    pub fn remove_domain_unloading(&mut self){
        unsafe{(*self.ptr).remove_domain_unloading_cb()};
    }
    ///Adds callback to be called when runtime is started.
    pub fn add_domain_unloading(&mut self,cb: fn (profiler:&mut Profiler<T>,&mut Domain)){
        unsafe{(*self.ptr).add_domain_unloading_cb(cb)};
    }
    ///Removes callback added by [`add_domain_unloading`]
    pub fn remove_domain_unloaded(&mut self){
        unsafe{(*self.ptr).remove_domain_unloading_cb()};
    }
    ///Adds callback to be called when runtime is started.
    pub fn add_domain_unloaded(&mut self,cb: fn (profiler:&mut Profiler<T>,&mut Domain)){
        unsafe{(*self.ptr).add_domain_unloading_cb(cb)};
    }
    ///Removes callback added by [`add_domain_unloading`]
    pub fn remove_domain_name(&mut self){
        unsafe{(*self.ptr).remove_domain_name_cb()};
    }
    ///Adds callback to be called when runtime is started.
    pub fn add_domain_name(&mut self,cb: fn (profiler:&mut Profiler<T>,&mut Domain,&str)){
        unsafe{(*self.ptr).add_domain_name_cb(cb)};
    }
    ///Removes callback added by [`add_domain_unloading`]
    pub fn remove_jit_begin(&mut self){
        unsafe{(*self.ptr).remove_jit_begin_cb()};
    }
    ///Adds callback to be called when runtime is started.
    pub fn add_jit_begin(&mut self,cb: fn (profiler:&mut Profiler<T>,&Method<String>)){
        unsafe{(*self.ptr).add_jit_begin_cb(cb)};
    }
}
