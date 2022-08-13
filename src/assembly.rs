use crate::binds::{MonoAssembly};
pub struct Assembly{
    ptr:*mut crate::binds::MonoAssembly,
}
impl Assembly{
    pub unsafe fn from_ptr(ptr:*mut MonoAssembly) -> Assembly{
        return Assembly{ptr:ptr};
    }
    pub unsafe fn get_ptr(&self)->*mut MonoAssembly{
        return self.ptr;
    }
    pub fn get_image(&self)->crate::image::Image{
        return unsafe{crate::image::Image::from_ptr(crate::binds::mono_assembly_get_image(self.ptr))};
    }
}
