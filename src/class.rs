pub struct Class{
    class_ptr:*mut crate::binds::MonoClass,
} 
impl Class{
    pub unsafe fn from_ptr(class_ptr:*mut crate::binds::MonoClass)->Option<Self>{
        if class_ptr == core::ptr::null_mut(){
            return None;
        }
        return Some(Self{class_ptr:class_ptr});
    }
    pub fn from_name(image:crate::image::Image,namespace:&str,name:&str)->Option<Self>{
        use std::ffi::CString;
        let cstr_nspace = CString::new(namespace).expect("Could not create CString");
        let cstr_name = CString::new(name).expect("Could not create CString");
        let res = unsafe{crate::binds::mono_class_from_name(image.to_ptr(),cstr_nspace.as_ptr(),cstr_name.as_ptr())};
        return unsafe{Self::from_ptr(res)};
    } 
    ///Case sensitve version of Class::from_name
    pub fn from_name_case(image:crate::image::Image,namespace:&str,name:&str)->Option<Self>{
        use std::ffi::CString;
        let cstr_nspace = CString::new(namespace).expect("Could not create CString");
        let cstr_name = CString::new(name).expect("Could not create CString");
        let res = unsafe{crate::binds::mono_class_from_name_case(image.to_ptr(),cstr_nspace.as_ptr(),cstr_name.as_ptr())};
        return unsafe{Self::from_ptr(res)};
    } 
}
