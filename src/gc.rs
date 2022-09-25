use crate::Object;
///Preform collection on *generation* and any generation lower than that.
pub fn collect(generation:i32){
    unsafe{crate::binds::mono_gc_collect(generation)};
}
///Get ammount of times garbage collection was preformed on *generation*.
pub fn collection_count(generation:i32)->i32{
    unsafe{crate::binds::mono_gc_collection_count(generation)}
}
///Get the maximum generation used by garbage collector.
pub fn mono_gc_max_generation()->i32{
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
//GC Handles