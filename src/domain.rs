use crate::binds::{MonoDomain, mono_domain_create,mono_domain_assembly_open};
use crate::assembly::{Assembly,AssemblyTrait};
use core::ptr::null_mut;
/// Safe representation of MonoDoamin type.
pub struct Domain{
    ptr:*mut MonoDomain,
} 
use std::ffi::CString;
impl Domain{ 
    ///Loads Assembly at path into domain, returns **None** if assembly could not be loaded(is missing or broken), and **Some(Assembly)** if it was succesfuly loaded. 
    pub fn assembly_open(&self,path:&str)->Option<Assembly>{
        //! <br>**Example:**<br>
        //!```rust
        //! let asm = domain.assembly_open("SomeAssembly.dll").expect("Could not load assembly!");
        //!```
        let cstr = CString::new(path).expect("Couldn't create cstring!");
        let ptr = unsafe{mono_domain_assembly_open(self.get_ptr(),cstr.as_ptr())};
        if ptr == null_mut(){
            return None;
        }
        drop(cstr);
        return Some(unsafe{Assembly::create_from_ptr(ptr)});
    }
    /// Creates new empty domain
    pub fn create()->Domain{
        //! <br>**Example:**<br>
        //!```rust
        //! let domain1 = jit::init();
        //! let domain2 = Domain::create();
        //!```
        return unsafe{Self::create_from_ptr(mono_domain_create())};
    }
    /// Function creating MonoDomain type from a pointer.
     pub unsafe fn create_from_ptr(ptr:*mut MonoDomain)->Domain{
        return Self{ptr:ptr};
    }
    /// Function returning internal pointer
    pub unsafe fn get_ptr(&self)->*mut MonoDomain{
        return self.ptr;
    }
}