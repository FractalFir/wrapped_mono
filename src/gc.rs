use crate::Object;
/// Preform collection on *generation* and any generation lower than that.
/// WARNING: All references in wrapped_mono are temporary and do not survive collection. To make objects persistant set 
pub fn collect(generation:i32){
    unsafe{crate::binds::mono_gc_collect(generation)};
}
///Get ammount of times garbage collection was preformed on *generation*.
pub fn collection_count(generation:i32)->i32{
    unsafe{crate::binds::mono_gc_collection_count(generation)}
}
///Get the maximum generation used by garbage collector.
pub fn max_generation()->i32{
    unsafe{crate::binds::mono_gc_max_generation()}
}
///Get generation *object* belongs to. It is only a hint and may not be exact.
pub fn get_generation(object:&Object)->i32{
    unsafe{crate::binds::mono_gc_get_generation(object.get_ptr())}
}
///Get size of the heap used by the garbage collector.
pub fn get_heap_size()->i64{
    unsafe{crate::binds:: mono_gc_get_heap_size()}
}
///Gets ammount of heap that is in currently occupied by objects.
pub fn get_used_size()->i64{
    unsafe{crate::binds::mono_gc_get_used_size()}
}
pub struct GCHandle{
    handle:u32,
}
use crate::binds::MonoObject;
impl GCHandle{
    pub fn get_target(&self)->*mut MonoObject{
        unsafe{crate::binds::mono_gchandle_get_target(self.handle)}
    }
    pub fn create(ptr:*mut MonoObject,pinned:bool)->GCHandle{
        GCHandle{handle:unsafe{crate::binds::mono_gchandle_new(ptr,pinned as i32)}}
    }
    pub fn create_default(ptr:*mut MonoObject)->GCHandle{
        GCHandle{handle:unsafe{crate::binds::mono_gchandle_new(ptr,false as i32)}}
    }
    pub fn free(handle:Self){
        unsafe{crate::binds::mono_gchandle_free(handle.handle)}
    }
}
#[cfg(feature = "referneced_objects")]
impl Drop for GCHandle{
    fn drop(&mut self){
        unsafe{crate::binds::mono_gchandle_free(self.handle)}
    }
}
#[cfg(test)]
pub fn count_objects()->u32{
    use crate::binds::MonoClass;
    unsafe extern "C" fn heap_walker(_:*mut MonoObject, _:*mut MonoClass,size:usize, num:usize, refs:*mut *mut MonoObject,offsets:*mut usize, count:*mut std::ffi::c_void)->i32{
        let count = count as *mut u32;
        (*count) += 1;
        return 0;
    }
    unsafe{
        let mut count:u32 = 0;
        crate::binds::mono_gc_walk_heap(0,Some(heap_walker),&mut count as *mut u32 as *mut _);
        return count;
    }
}
