use std::sync::Arc;
use crate::binds::{MonoAssembly};
pub type Assembly = Arc<_Assembly>;
pub struct _Assembly{
    pub ptr:*mut MonoAssembly,
} 
pub trait AssemblyTrait{
    unsafe fn create_from_ptr(ptr:*mut MonoAssembly) -> Assembly;
}
impl AssemblyTrait for Assembly{
    unsafe fn create_from_ptr(ptr:*mut MonoAssembly) -> Assembly{
        return Arc::new(_Assembly{ptr:ptr});
    }
}
