use crate::binds::MonoImage;
/// Safe representation of [`MonoImage`], the part of [`MonoAssembly`] holding CLI code.
#[derive(Copy, Clone)]
pub struct Image {
    img_ptr: *mut MonoImage,
}
#[allow(unused_imports)]
use crate::binds::MonoAssembly; // For documentation
use crate::metadata::{MetadataTableInfo, MetadataTableKind, MetadataToken};
use std::ffi::CString;
impl Image {
    /// Gets metadata table from an image.
    #[must_use]
    pub fn get_table_info(&self, kind: MetadataTableKind) -> MetadataTableInfo {
        unsafe {
            MetadataTableInfo::from_ptr(
                crate::binds::mono_image_get_table_info(self.img_ptr, kind as i32),
                kind,
            )
        }
    }
    /// Gets string from metadata string heap. *index* must be within the string heap.
    #[must_use]
    pub fn metadata_string_heap(&self, index: MetadataToken) -> String {
        let cstr = unsafe {
            CString::from_raw(
                crate::binds::mono_metadata_string_heap(self.img_ptr, index) as *mut i8
            )
        };
        let res = cstr.to_str().expect(crate::CSTR2STR_ERR).to_owned();
        let _ = cstr.into_raw();
        res
    }
    /// Gets a binary blob from metadata blob heap. Index must be within range of the blob heap.
    pub fn blob_heap(&self,index:u32)->&[u8]{
        unsafe{
        let ptr = crate::binds::mono_metadata_blob_heap(self.get_ptr(),index);
        let mut blob_ptr:*const i8 = std::ptr::null();
        let blob_size = crate::binds::mono_metadata_decode_blob_size(ptr,&mut blob_ptr as *mut _);
        std::slice::from_raw_parts(blob_ptr.cast(),blob_size as usize)}
    }
    /// Creates the value of [`Image`] type from a [`MonoImage`].
    /// # Safety
    /// *ptr* must be a pointer to a valid [`MonoImage`].
    #[must_use]
    pub unsafe fn from_ptr(ptr: *mut crate::binds::MonoImage) -> Self {
        Self { img_ptr: ptr }
    }
    /// Returns internal pointer to [`MonoImage`] this [`Image`] represents.
    #[must_use]
    pub fn get_ptr(&self) -> *mut MonoImage {
        self.img_ptr
    }
    /// Initializes all global variables in image(static members of classes).
    pub fn init(&self) {
        unsafe { crate::binds::mono_image_init(self.img_ptr) };
    }
    /// Returns name of this image
    #[must_use]
    pub fn get_name(&self) -> String {
        let ptr = unsafe { crate::binds::mono_image_get_name(self.img_ptr) };
        let cstr = unsafe { CString::from_raw(ptr as *mut i8) };
        let name = cstr.to_str().expect(crate::STR2CSTR_ERR).to_owned();
        let _ = cstr.into_raw();
        name
    }
    /// Closes this image, unloading it from memory.
    /// # Safety
    /// All references to types within image will be made invalid after this call.
    pub unsafe fn close(&mut self) {
        crate::binds::mono_image_close(self.img_ptr);
        let _ = &self;
    }
}
