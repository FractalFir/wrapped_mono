pub use crate::binds::MonoImage;
pub use crate::binds::MonoAssembly;
/// Safe representation [`MonoImage`], part of [`MonoAssembly`] holding CLI code.
pub struct Image{
    img_ptr:*mut MonoImage,
}
impl Image{
    pub unsafe fn from_ptr(ptr:*mut crate::binds::MonoImage)->Self{
        return Self{img_ptr:ptr};
    }
    pub fn get_ptr(&self)->*mut MonoImage{
        return self.img_ptr;
    }
    pub fn to_ptr(&self)->*mut crate::binds::MonoImage{
        return self.img_ptr;
    }
    ///Initialize all global varaibles in image.
    pub fn init(&self){
        unsafe{crate::binds:: mono_image_init(self.img_ptr)};
    }
    ///Returns name of the image
    pub fn get_name(&self)->String{
        let ptr = unsafe{crate::binds::mono_image_get_name(self.img_ptr)};
        use std::ffi::CString;
        let cstr = unsafe{CString::from_raw(ptr as *mut i8)};
        let s = cstr.to_str().expect("Could not create string grom  CString").to_owned();
        let _ = cstr.into_raw();
        return s;
    }
    ///Closes this image. 
    /// # Safety
    /// All referneces to types within image will be made invalid after this call.
    pub fn close(&mut self){
        unsafe{crate::binds::mono_image_close(self.img_ptr)};
        drop(self);
    }
}
