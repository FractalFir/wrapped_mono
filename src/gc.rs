use crate::{Object, ObjectTrait};
/// Preform collection on *generation* and any generation lower than that.
/// WARNING: If raw object pointers are used, collection may collect objects pointed to by those pointers.
pub fn collect_generation(generation: i32) {
    unsafe { crate::binds::mono_gc_collect(generation) };
}
/// Preform collection on all generations of objects.
/// WARNING: If raw object pointers are used, collection may collect objects pointed to by those pointers.
pub fn collect() {
    unsafe { crate::binds::mono_gc_collect(max_generation()) };
}
///Get ammount of times garbage collection was preformed on *generation*.
#[must_use]
pub fn collection_count(generation: i32) -> i32 {
    unsafe { crate::binds::mono_gc_collection_count(generation) }
}
///Get the maximum generation used by garbage collector.
#[must_use]
pub fn max_generation() -> i32 {
    unsafe { crate::binds::mono_gc_max_generation() }
}
///Get generation *object* belongs to. It is only a hint and may not be exact.
#[must_use]
pub fn get_generation(object: &Object) -> i32 {
    unsafe { crate::binds::mono_gc_get_generation(object.get_ptr()) }
}
///Get size of the heap used by the garbage collector.
#[must_use]
pub fn get_heap_size() -> i64 {
    unsafe { crate::binds::mono_gc_get_heap_size() }
}
///Gets ammount of heap that is in currently occupied by objects.
#[must_use]
pub fn get_used_size() -> i64 {
    unsafe { crate::binds::mono_gc_get_used_size() }
}
/// A Garbage Collector handle. Should only be used if default feature referenced objects is disabled. Otherwise all of its functionality is handled automatically behind the scenes
pub struct GCHandle {
    handle: u32,
}
use crate::binds::MonoObject;
impl GCHandle {
    /// Gets a pointer to an object this handle targets.
    #[must_use]
    pub fn get_target(&self) -> *mut MonoObject {
        unsafe { crate::binds::mono_gchandle_get_target(self.handle) }
    }
    /// Creates a new Garbage Collector handle.
    /// # Safety
    /// *ptr* must be a pointer to a valid object.
    pub unsafe fn create(ptr: *mut MonoObject, pinned: bool) -> Self {
        Self {
            handle: unsafe { crate::binds::mono_gchandle_new(ptr, i32::from(pinned)) },
        }
    }
    /// Creates a new Garbage Collector handle with default pin settings(unpinned).
    /// # Safety
    /// *ptr* must be a pointer to a valid object.
    pub unsafe fn create_default(ptr: *mut MonoObject) -> Self {
        Self::create(ptr, false)
    }
    /// Frees this handle, deleting the reference to object it targets.
    #[allow(clippy::needless_pass_by_value)] //Not needles. Self is really consumed here.
    pub fn free(handle: Self) {
        unsafe { crate::binds::mono_gchandle_free(handle.handle) }
    }
}
#[cfg(feature = "referenced_objects")]
impl Drop for GCHandle {
    fn drop(&mut self) {
        unsafe { crate::binds::mono_gchandle_free(self.handle) }
    }
}
#[cfg(test)]
pub fn count_objects() -> u32 {
    use crate::binds::MonoClass;
    unsafe extern "C" fn heap_walker(
        _: *mut MonoObject,
        _: *mut MonoClass,
        _size: usize,
        _num: usize,
        _refs: *mut *mut MonoObject,
        _offsets: *mut usize,
        count: *mut std::ffi::c_void,
    ) -> i32 {
        let count = count as *mut u32;
        (*count) += 1;
        0
    }
    unsafe {
        let mut count: u32 = 0;
        crate::binds::mono_gc_walk_heap(0, Some(heap_walker), &mut count as *mut u32 as *mut _);
        count
    }
}
#[doc(hidden)]
#[repr(C)]
pub struct MonoStackData {
    pub stack_ptr: *const u8,
    pub dummy: i32,
}
#[cfg(old_gc_unsafe)]
extern "C" {
    #[doc(hidden)]
    pub fn mono_threads_enter_gc_unsafe_region_internal(msd: &MonoStackData) -> GCUnsafeAreaMarker;
    #[doc(hidden)]
    pub fn mono_threads_exit_gc_unsafe_region_internal(
        gc_unsafe_cookie: GCUnsafeAreaMarker,
        msd: &MonoStackData,
    );
}
#[cfg(not(old_gc_unsafe))]
extern "C" {
    #[doc(hidden)]
    pub fn mono_threads_enter_gc_unsafe_region(msd: &MonoStackData) -> GCUnsafeAreaMarker;
    #[doc(hidden)]
    pub fn mono_threads_exit_gc_unsafe_region(
        gc_unsafe_cookie: GCUnsafeAreaMarker,
        msd: &MonoStackData,
    );
}

#[doc(hidden)]
#[must_use = "GCUnsafeAreaMarker marks a section of code that could be disturbed by GarbageCollector and prevents this from happening.
 It is created at begging of that critical section and must be consumend at its end, otherwise GCUnsfae Mode will newer be exited which will result in bugs and crashes."]
#[repr(transparent)]
pub struct GCUnsafeAreaMarker {
    #[allow(dead_code)]
    // This field is not unused, but it seems like it tor rust since it does not know anything about the C side of things
    gc_unsafe_cookie: *mut i32,
}
#[doc(hidden)]
#[inline(always)]
#[allow(clippy::inline_always)]
pub fn gc_unsafe_enter() -> (GCUnsafeAreaMarker, MonoStackData) {
    #[cfg(old_gc_unsafe)]
    {
        let stack_item: u8 = 0; //Useless dummy value used to get the stack pointer.
        let msd = crate::gc::MonoStackData {
            dummy: 0,
            stack_ptr: std::ptr::addr_of!(stack_item),
        }; // StackDataObject used to restore the stack.
        let marker = unsafe { crate::gc::mono_threads_enter_gc_unsafe_region_internal(&msd) }; // Entering GC Unsafe mode (signalling to GC that we will be using managed objects that should not be moved)
        (marker, msd)
    }
    #[cfg(not(old_gc_unsafe))]
    {
        let stack_item: u8 = 0; //Useless dummy value used to get the stack pointer.
        let msd = crate::gc::MonoStackData {
            dummy: 0,
            stack_ptr: std::ptr::addr_of!(stack_item),
        }; // StackDataObject used to restore the stack.
        let marker = unsafe { crate::gc::mono_threads_enter_gc_unsafe_region(&msd) }; // Entering GC Unsafe mode (signalling to GC that we will be using managed objects that should not be moved)
        (marker, msd)
    }
}
#[doc(hidden)]
#[inline(always)]
#[allow(clippy::inline_always)]
pub fn gc_unsafe_exit(markers: (GCUnsafeAreaMarker, MonoStackData)) {
    #[cfg(old_gc_unsafe)]
    unsafe {
        crate::gc::mono_threads_exit_gc_unsafe_region_internal(markers.0, &markers.1);
    }
    #[cfg(not(old_gc_unsafe))]
    unsafe {
        crate::gc::mono_threads_exit_gc_unsafe_region(markers.0, &markers.1);
    }
}
