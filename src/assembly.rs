use std::sync::Arc;
use crate::binds::{MonoAssembly}; 
struct _Assembly{
    ptr:*mut MonoAssembly,
}
pub type Assembly = Arc<_Assembly>;