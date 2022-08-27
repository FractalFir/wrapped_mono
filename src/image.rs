use crate::binds::MonoImage;
use crate::binds::MonoAssembly;
use crate::class::Class;
/// Safe representation [`MonoImage`], part of [`MonoAssembly`] holding CLI code.
#[derive(Copy,Clone)]
pub struct Image{
    img_ptr:*mut MonoImage,
}
use std::ffi::CString;
use crate::metadata::{MetadataTableInfo,MetadataTableKind,MetadataToken};
impl Image{
    ///Returns if function at path *path* has return type *rtype* and takes parameters *params*
    ///Not finished yet.
    fn check_fnc_sig(&self,path:&str,rtype:Class,params:&[Class])->bool{
        use std::ffi::CString;
        use crate::binds::MonoTableInfo;
        const MONO_TABLE_METHOD:i32 = crate::binds::MonoMetaTableEnum_MONO_TABLE_METHOD as i32;
        const MONO_ASSEMBLYREF_SIZE:usize = crate::binds::MONO_ASSEMBLYREF_SIZE as usize;
        let info = unsafe{crate::binds::mono_image_get_table_info(self.img_ptr,MONO_TABLE_METHOD)};
        let rows = unsafe{crate::binds::mono_table_info_get_rows(info)};
        for row in 0..rows{
            //let mut collums:[u32;MONO_ASSEMBLYREF_SIZE] = [0;MONO_ASSEMBLYREF_SIZE];
            let name_guid = unsafe{crate::binds::mono_metadata_decode_row_col (info,row,crate::binds::MONO_METHOD_NAME)};
            let name_ptr = unsafe{
                crate::binds::mono_metadata_string_heap(self.img_ptr, name_guid)
            };
            let cstr = unsafe{CString::from_raw(name_ptr as *mut i8)};
            let string = cstr.to_str().expect("Could not conert pointer to string!").to_owned();
            let _ = cstr.into_raw();
            println!("name: {},row:{}",string,row);
        }
        unimplemented!("Not done yet, {} rows, {:#x} ptr!",rows,info as usize);
    }
    ///Gets metadata table from an image.
    pub fn get_table_info(&self,kind:MetadataTableKind)->MetadataTableInfo{
        return unsafe{MetadataTableInfo::from_ptr(
            crate::binds::mono_image_get_table_info(self.img_ptr,kind as i32)
        ,kind)};
    }
    ///Gets string from metadata. *index* must be within the string heap.
    pub fn metadata_string_heap(&self,index:MetadataToken)->String{
        let cstr = unsafe{CString::from_raw(
            crate::binds::mono_metadata_string_heap(self.img_ptr,index) as *mut i8)
        };
        let res = cstr.to_str().expect(crate::CSTR2STR_ERR).to_owned();
        let _ = cstr.into_raw();
        return res;
    }
    ///Creates the value of [`Image`] type from a valid pointer.
    pub unsafe fn from_ptr(ptr:*mut crate::binds::MonoImage)->Self{
        return Self{img_ptr:ptr};
    }
    //Returns internal pointer
    pub fn get_ptr(&self)->*mut MonoImage{
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
        let s = cstr.to_str().expect(crate::STR2CSTR_ERR).to_owned();
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
