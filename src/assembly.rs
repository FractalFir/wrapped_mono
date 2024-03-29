use crate::binds::MonoAssembly;
use crate::image::Image;
use std::ffi::CString;
/// Safe representation of an executable file containing managed code and data about it.
#[derive(Clone, Copy)]
pub struct Assembly {
    ptr: *mut crate::binds::MonoAssembly,
}
impl Assembly {
    /// Creates [`Assembly`] from a [`MonoAssembly`] pointer.
    /// # Safety
    /// *ptr* must be a valid [`MonoAssembly`] pointer.
    #[must_use]
    pub unsafe fn from_ptr(ptr: *mut MonoAssembly) -> Self {
        Self { ptr }
    }
    /// Returns the internal pointer to [`MonoAssembly`] this object represents.
    #[must_use]
    pub fn get_ptr(&self) -> *mut MonoAssembly {
        self.ptr
    }
    /// Gets the [`Image`] from this assembly(part of the assembly containing executable code)
    #[must_use]
    pub fn get_image(&self) -> Image {
        unsafe { Image::from_ptr(crate::binds::mono_assembly_get_image(self.ptr)) }
    }
    /// Returns main assembly(first loaded assembly)
    #[must_use]
    pub fn get_main() -> Option<Self> {
        let ptr = unsafe { crate::binds::mono_assembly_get_main() };
        if ptr.is_null() {
            None
        } else {
            unsafe { Some(Self::from_ptr(ptr)) }
        }
    }
    /// Gets name of assembly.
    #[must_use]
    pub fn get_name(&self) -> String {
        // assembly_name does not have to be freed, because it lives as long as the assembly.
        let assembly_name_ptr = unsafe { crate::binds::mono_assembly_get_name(self.ptr) };
        let cstr_name = unsafe {
            CString::from_raw(
                crate::binds::mono_assembly_name_get_name(assembly_name_ptr) as *mut i8
            )
        };
        let name = cstr_name
            .to_str()
            .expect("Could not create String!")
            .to_owned();
        let _ = cstr_name.into_raw(); //release pointer
        name
    }
    /// Checks if assembly *name* is loaded, and if it is returns that assembly.
    #[must_use]
    pub fn assembly_loaded(name: &str) -> Option<Self> {
        let cstr = CString::new(name).expect(crate::STR2CSTR_ERR);
        let assembly_name = unsafe { crate::binds::mono_assembly_name_new(cstr.as_ptr()) };
        let ptr = unsafe { crate::binds::mono_assembly_loaded(assembly_name) };
        drop(cstr);
        unsafe { crate::binds::mono_assembly_name_free(assembly_name) };
        if ptr.is_null() {
            None
        } else {
            unsafe { Some(Self::from_ptr(ptr)) }
        }
    }
    /// Releases reference to assembly. Assembly is closed when all outside references  to it are released.
    pub fn close(self) {
        unsafe { crate::binds::mono_assembly_close(self.ptr) };
    }
}
