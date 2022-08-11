pub struct Image{
    img_ptr:*mut crate::binds::MonoImage,
}
impl Image{
    pub unsafe fn from_ptr(ptr:*mut crate::binds::MonoImage)->Self{
        return Self{img_ptr:ptr};
    }
    pub fn to_ptr(&self)->*mut crate::binds::MonoImage{
        return self.img_ptr;
    }
}
