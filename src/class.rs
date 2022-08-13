pub struct Class{
    class_ptr:*mut crate::binds::MonoClass,
} 
impl Class{
    pub unsafe fn get_ptr(&self)->*mut crate::binds::MonoClass{
        return self.class_ptr;
    }
    pub unsafe fn from_ptr(class_ptr:*mut crate::binds::MonoClass)->Option<Self>{
        if class_ptr == core::ptr::null_mut(){
            return None;
        }
        return Some(Self{class_ptr:class_ptr});
    }
    pub fn from_name(image:&crate::image::Image,namespace:&str,name:&str)->Option<Self>{
        use std::ffi::CString;
        let cstr_nspace = CString::new(namespace).expect("Could not create CString");
        let cstr_name = CString::new(name).expect("Could not create CString");
        let res = unsafe{crate::binds::mono_class_from_name(image.to_ptr(),cstr_nspace.as_ptr(),cstr_name.as_ptr())};
        return unsafe{Self::from_ptr(res)};
    } 
    ///Case sensitve version of Class::from_name
    pub fn from_name_case(image:&crate::image::Image,namespace:&str,name:&str)->Option<Self>{
        use std::ffi::CString;
        let cstr_nspace = CString::new(namespace).expect("Could not create CString");
        let cstr_name = CString::new(name).expect("Could not create CString");
        let res = unsafe{crate::binds::mono_class_from_name_case(image.to_ptr(),cstr_nspace.as_ptr(),cstr_name.as_ptr())};
        return unsafe{Self::from_ptr(res)};
    } 
    pub fn get_field_from_name(&self,name:&str)->Option<ClassField>{
        let cstr = std::ffi::CString::new(name).expect("Could not create CString");
        let res = unsafe{ClassField::from_ptr(crate::binds::mono_class_get_field_from_name(self.get_ptr(),cstr.as_ptr()))};
        drop(cstr);
        return res;
    }
}
pub struct ClassField{
    cf_ptr:*mut crate::binds::MonoClassField,
}
impl ClassField{
    pub fn from_ptr(cf_ptr:*mut crate::binds::MonoClassField)->Option<Self>{
        if cf_ptr == core::ptr::null_mut(){
            return None;
        }
        return Some(Self{cf_ptr:cf_ptr});
    }
    pub fn get_ptr(&self)->*mut crate::binds::MonoClassField{
        return self.cf_ptr;
    }
    pub fn get_name(&self)->String{
        let cstr = unsafe{std::ffi::CString::from_raw(crate::binds::mono_field_get_name(self.get_ptr()) as *mut i8)};
        let s = cstr.to_str().expect("Could not create String from ptr").to_owned();
        drop(cstr);
        return s;
    }
    ///get metadata of a field. **not** it's value
    pub fn get_data(&self) -> *const ::std::os::raw::c_char{
        return unsafe{crate::binds::mono_field_get_data(self.get_ptr())};
    }
    */
    pub fn get_parent(&self)->Class{
        return unsafe{Class::from_ptr(crate::binds:: mono_field_get_parent(self.get_ptr()))}.expect("Could not get ClassFiled of Class");
    }
}