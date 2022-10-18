use crate::binds::MonoClass;
use crate::{Image,Method,MethodTrait,InteropSend};
use std::ffi::CString;
use core::ffi::c_void;
///  Safe representation of a managed class.(eg. System.Int64, System.Object, etc.);
#[derive(Eq,Copy,Clone)]
pub struct Class{
    class_ptr:*mut MonoClass,
} 
impl Class{
    /// Returns copy of internal pointer representing [`MonoClass`].
    pub fn get_ptr(&self)->*mut MonoClass{
        self.class_ptr
    }
    /// Creates [`Class`] from *class_ptr*. If it is not null, returns [`Some`], otherwise [`None`].
    /// # Safety
    /// *class_ptr* must me either a valid pointer to [`MonoClass`] or null pointer.
    pub unsafe fn from_ptr(class_ptr:*mut MonoClass)->Option<Self>{
        if class_ptr.is_null(){
            return None;
        }
        Some(Self{class_ptr})
    }
    /// Returns class named *name* in *namespace* in image *image*. Is not case sensitive!
    /// 
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |image| &[`Image`]| image to load class from |
    /// |namespace| &[`str`]| path to namespace this class is in |
    /// |name| &[`str`]| name of class to get |
    /// # Example
    /// ```rust
    /// let some_class = Class::from_name(&some_image,"soMeNameSpace","SomeClass").expect("Could not find a class!");
    /// ```
    pub fn from_name(image:&crate::image::Image,namespace:&str,name:&str)->Option<Self>{
        let cstr_nspace = CString::new(namespace).expect(crate::STR2CSTR_ERR);
        let cstr_name = CString::new(name).expect(crate::STR2CSTR_ERR);
        let res = unsafe{crate::binds::mono_class_from_name(image.get_ptr(),cstr_nspace.as_ptr(),cstr_name.as_ptr())};
        unsafe{Self::from_ptr(res)}
    } 
    /// Returns class named *name* in *namespace* in image *image*. It is case sensitive.
    ///
    /// # Arguments
    /// |Name   |Type   |Description|
    /// |-------|-------|------|
    /// |image| &[`Image`] | image to load class from |
    /// |namespace| &[`str`] | path to namespace this class is in |
    /// |name| &[`str`] | name of class to get |
    /// # Example
    /// ```rust
    /// let some_class = Class::from_name_case(&some_image,"SomeNamespace","SomeClass").expect("Could not find a class!");
    /// ```
    pub fn from_name_case(image:&crate::image::Image,namespace:&str,name:&str)->Option<Self>{
        let cstr_nspace = CString::new(namespace).expect(crate::STR2CSTR_ERR);
        let cstr_name = CString::new(name).expect(crate::STR2CSTR_ERR);
        let res = unsafe{crate::binds::mono_class_from_name_case(image.get_ptr(),cstr_nspace.as_ptr(),cstr_name.as_ptr())};
        unsafe{Self::from_ptr(res)}
    } 
    /// Gets field *name* of class. 
    /// # Safety
    /// Getters, Setters, and Indexers **are not** fields of classes!;
    /// # Example
    /// ## C#
    ///```csharp
    /// class SomeClass{
    ///     int someField;    
    /// }
    ///```
    /// ## Rust
    ///```rust
    /// let some_field = some_class.get_field_from_name("someField").expect("Could not find field!");
    ///```
    pub fn get_field_from_name(&self,name:&str)->Option<ClassField>{
        let cstr = CString::new(name).expect(crate::STR2CSTR_ERR);
        let res = unsafe{ClassField::from_ptr(crate::binds::mono_class_get_field_from_name(self.get_ptr(),cstr.as_ptr()))};
        crate::hold(&cstr);
        res
    }
    /// Returns name of this class
    pub fn get_name(&self)->String{
        let cstr = unsafe{CString::from_raw(crate::binds::mono_class_get_name(self.class_ptr) as *mut i8)};
        let res = cstr.to_str().expect("Could not covert CString to String!").to_owned();
        //pointer does not have to be released
        let _ = cstr.into_raw();
        res
    }
    /* TODO: Change get_ctos to include new funcion generic arguments
    ///Gets all of the constuctors of this class. **Does not get parent class construtors!**
    pub fn get_ctors(&self)->Vec<Method>{
        let mut gptr = 0 as *mut std::os::raw::c_void;
        let mut res = Vec::new();
        while let Some(cf) = unsafe{Method::from_ptr(
            crate::binds::mono_class_get_methods(self.class_ptr,&mut gptr as *mut *mut c_void)
        )}{
            if cf.get_name() == ".ctor"{
                res.push(cf);
            }
        }
        return res;
    }
    
    ///Gets all of the constuctors of this class, including parent class construtors.
    pub fn get_ctros_recursive(&self)->Vec<Method>{
        let mut ctors = self.get_ctors();
        let parent = self.get_parent();
        return match parent {
            Some(parent)=>{
                ctors.extend(parent.get_ctros_recursive());
                ctors
            },
            None=>ctors,
        }
    }
    */
    /// Gets the image this class exists in.
    pub fn get_image(&self)->Image{
        unsafe{Image::from_ptr(crate::binds:: mono_class_get_image(self.class_ptr))}
    }
    /// Returns amount of memory occupied by object when inside array.
    pub fn array_element_size(&self)->i32{
        unsafe{crate::binds:: mono_class_array_element_size(self.class_ptr)}
    }
    /// Gets a [`Vec`] containing all interfaces this class implements.
    pub fn get_interfaces(&self)->Vec<Class>{
        let mut gptr = std::ptr::null_mut::<i32>();
        let mut res = Vec::new();
        while let Some(class) = unsafe{Self::from_ptr(
            crate::binds::mono_class_get_interfaces(self.class_ptr,&mut gptr as *mut *mut  i32 as *mut *mut std::os::raw::c_void)
        )}{
            res.push(class);
        }
        res
    }
    /// Gets namespace this class is in, or "" string if it is not in any namespace.
    pub fn get_namespace(&self)->String{
        let cstr = unsafe{CString::from_raw(crate::binds::mono_class_get_namespace(self.class_ptr) as *mut i8)};
        let res = cstr.to_str().expect("Could not create CString!").to_owned();
        //got const pointer that does not have to be freed, so we release it.
        let _ = cstr.into_raw();
        res
    }
    ///Gets class this class is nested in, or [`None`] if it is not nested in any type.
    pub fn get_nesting_type(&self)->Option<Class>{
        unsafe{Self::from_ptr(
            crate::binds::mono_class_get_nesting_type(self.class_ptr)
        )}
    }
    /// Gets type this class derives from or [`None`] if it does not derive any type.
    /// # Example
    /// For a class `SomeClass`
    /// # C#
    ///```csharp
    /// class SomeClass:SomeparentClass{
    ///    
    /// }
    ///```
    ///
    /// Function will return `SomeParentClass`
    pub fn get_parent(&self)->Option<Class>{
        unsafe{Self::from_ptr(
            crate::binds::mono_class_get_parent(self.class_ptr)
        )}
    }
    /// Gets number of dimensions of array.
    /// # Constrains 
    /// *self* must be an array type, otherwise returns 0.
    pub fn get_rank(&self)->u32{
        unsafe{crate::binds::mono_class_get_rank(self.class_ptr) as u32}
    }
    /// Return size of static data of this class
    pub fn data_size(&self)->i32{
        unsafe{crate::binds::mono_class_data_size(self.class_ptr)}
    }
    /// Get element class of an array. *self* **must** be an array type, otherwise returns *self*.
    pub fn get_element_class(&self)->Class{
        unsafe{Self::from_ptr(
            crate::binds::mono_class_get_element_class(self.class_ptr))
        }.expect("Colud not get array element class!")
    }
    /// Returns if class implements interface **iface**.
    pub fn implements_interface(&self,iface:&Self)->bool{
        (unsafe{crate::binds::mono_class_implements_interface(self.class_ptr,iface.class_ptr)} != 0)
    } 
    /// Returns true if object of type *other* can be assigned to class *self*.
    pub fn is_assignable_from(&self,other:&Self)->bool{
        (unsafe{crate::binds::mono_class_is_assignable_from(self.class_ptr,other.class_ptr)} != 0)
    }
    /// Checks if *self* represents a delegate type.
    pub fn is_delegate(&self)->bool{
        (unsafe{crate::binds::mono_class_is_delegate(self.class_ptr)} != 0)
    }
    /// Checks if *self* represents an enumeration type.
    pub fn is_enum(&self)->bool{
        (unsafe{crate::binds::mono_class_is_enum(self.class_ptr)} != 0)
    }
    //TODO: consider implementing mono_class_is_subclass_of(it seems mostly redundant, but it may be useful)
    //TODO: figure out what exactly mono_class_num_events is supposed to do, and implement it.
    /// Gets amount of **static and instance** files of class
    pub fn num_fields(&self)->i32{
        unsafe{crate::binds::mono_class_num_fields(self.class_ptr)}
    }
    /// Gets amount of methods in the class *self*
    pub fn num_methods(&self)->i32{
        unsafe{crate::binds::mono_class_num_methods(self.class_ptr)}
    }
    //TODO: expand this description, since it does not seam to be fully clear.
    /// Gets number of properties in the class(getters,setters,indexers)
    pub fn num_properties(&self)->i32{
        unsafe{crate::binds::mono_class_num_properties(self.class_ptr)}
    }
    ///Checks if *self* represents a value type.
    pub fn is_valuetype(&self)->bool{
        (unsafe{crate::binds::mono_class_is_valuetype(self.class_ptr)} != 0)
    }
    /*
    TODO:figure out how this function works and fix it.
    /// Gets size of a value of type *self*
    pub fn value_size(&self)->i32{
        return unsafe{crate::binds::mono_class_value_size(self.class_ptr)};
    }
    */
    /// Returns [`Class`] representing `System.Object` type.
    pub fn get_object()->Class{
        unsafe{Self::from_ptr(
            crate::binds::mono_get_object_class()
        )}.expect("Could not get calls representing System.Object!")
    }
    /// Returns [`Class`] representing `System.Int16` type ([i16]).
    pub fn get_int_16()->Class{
        unsafe{Self::from_ptr(
            crate::binds::mono_get_int16_class()
        )}.expect("Could not get calls representing System.Int16!")
    }
    /// Returns [`Class`] representing `System.Int32` type ([i32]). 
    pub fn get_int_32()->Class{
        unsafe{Self::from_ptr(
            crate::binds::mono_get_int32_class()
        )}.expect("Could not get calls representing System.Int32!")
    }
    /// Returns [`Class`] representing `System.Int64` type ([i64]). 
    pub fn get_int_64()->Class{
        unsafe{Self::from_ptr(
            crate::binds::mono_get_int64_class()
        )}.expect("Could not get calls representing System.Int64!")
    }
    /// Returns [`Class`] representing `System.Double` type ([f64]).
    pub fn get_double()->Class{
        unsafe{Self::from_ptr(
            crate::binds:: mono_get_double_class()
        )}.expect("Could not get calls representing System.Double!")
    }
    /// Returns [`Class`] representing `System.Enum` type. 
    pub fn get_enum()->Class{
        unsafe{Self::from_ptr(
            crate::binds:: mono_get_enum_class()
        )}.expect("Could not get calls representing System.Enum!")
    }
    /// Returns [`Class`] representing `System.IntPtr` type ([isize]). 
    pub fn get_int_ptr()->Class{
        unsafe{Self::from_ptr(
            crate::binds:: mono_get_intptr_class()
        )}.expect("Could not get calls representing System.IntPtr!")
    }
    /// Returns [`Class`] representing `System.SByte` type ([i8]). 
    pub fn get_sbyte()->Class{
        unsafe{Self::from_ptr(
            crate::binds:: mono_get_sbyte_class()
        )}.expect("Could not get calls representing System.IntPtr!")
    }
    /// Returns [`Class`] representing `System.Single` type ([f32]).
    pub fn get_single()->Class{
        unsafe{Self::from_ptr(
            crate::binds:: mono_get_single_class()
        )}.expect("Could not get calls representing System.Single!")
    }
    /// Returns [`Class`] representing `System.String` type.
    pub fn get_string()->Class{
        unsafe{Self::from_ptr(
            crate::binds:: mono_get_string_class()
        )}.expect("Could not get calls representing System.String!")
    }
    /// Returns [`Class`] representing `System.Threading.Thread` type.
    pub fn get_thread()->Class{
        unsafe{Self::from_ptr(
            crate::binds:: mono_get_thread_class()
        )}.expect("Could not get calls representing System.Threading.Thread!")
    }
    /// Returns [`Class`] representing `System.UInt16` type([u16]).
    pub fn get_uint_16()->Class{
        unsafe{Self::from_ptr(
            crate::binds:: mono_get_uint16_class()
        )}.expect("Could not get calls representing System.UInt16!")
    }
    /// Returns [`Class`] representing `System.UInt32` type([u32]).
    pub fn get_uint_32()->Class{
        unsafe{Self::from_ptr(
            crate::binds:: mono_get_uint32_class()
        )}.expect("Could not get calls representing System.UInt32!")
    }
    /// Returns [`Class`] representing `System.UInt64` type([u64]).
    pub fn get_uint_64()->Class{
        unsafe{Self::from_ptr(
            crate::binds:: mono_get_uint64_class()
        )}.expect("Could not get calls representing System.UInt64!")
    }
    /// Returns [`Class`] representing `System.UIntPtr` type ([usize]). 
    pub fn get_uint_ptr()->Class{
        unsafe{Self::from_ptr(
            crate::binds:: mono_get_uintptr_class()
        )}.expect("Could not get calls representing System.IntPtr!")
    }
    /// Returns [`Class`] representing `System.Void` type. 
    pub fn get_void()->Class{
        unsafe{Self::from_ptr(
            crate::binds:: mono_get_void_class()
        )}.expect("Could not get calls representing System.Void!")
    }
    /// Returns [`Class`] representing `System.Array` type. 
    pub fn get_array()->Class{
        unsafe{Self::from_ptr(
            crate::binds:: mono_get_array_class()
        )}.expect("Could not get calls representing System.Array!")
    }
    /// Returns [`Class`] representing `System.Boolean` type ([bool]). 
    pub fn get_boolean()->Class{
        unsafe{Self::from_ptr(
            crate::binds:: mono_get_boolean_class()
        )}.expect("Could not get calls representing System.Boolean!")
    }
    /// Returns [`Class`] representing `System.Byte` type ([u8]). 
    pub fn get_byte()->Class{
        unsafe{Self::from_ptr(
            crate::binds:: mono_get_byte_class()
        )}.expect("Could not get calls representing System.Byte!")
    }
    /// Returns [`Class`] representing `System.Char` type ([char]). 
    pub fn get_char()->Class{
        unsafe{Self::from_ptr(
            crate::binds:: mono_get_char_class()
        )}.expect("Could not get calls representing System.Char!")
    }
    /// Gets class of an array of class *self* with rank (for int and rank 1, returns int[], for byte and rank 3 returns byte[][][],etc.)
    pub fn get_array_class(&self,rank:u32)->Class{
        unsafe{Self::from_ptr(
            crate::binds::mono_array_class_get(self.class_ptr,rank)
        )}.expect("Impossible condition reached")
    }
    /// Returns [`Class`] representing the type **System.Exception**.
    pub fn get_exception_class()->Class{
        unsafe{Class::from_ptr(crate::binds::mono_get_exception_class())}.expect("Could not get ExceptionClass!")
    }
    /// Returns [`Class`] representing the type **System.Delegate**.
    pub fn get_delegate_class()->Class{
        *DELEGATE
    }
    /// Returns all fields of a class
    pub fn get_fields(&self)->Vec<ClassField>{
        let mut gptr = std::ptr::null_mut::<std::os::raw::c_void>();
        let mut res = Vec::new();
        while let Some(cf) = unsafe{ClassField::from_ptr(
            crate::binds::mono_class_get_fields(self.class_ptr,&mut gptr as *mut *mut c_void)
        )}{
            res.push(cf);
        }
        res
    }
    /// Returns field with name *name*
    pub fn get_field(&self,name:&str)->Option<ClassField>{
        let mut gptr = std::ptr::null_mut::<std::os::raw::c_void>();
        while let Some(cf) = unsafe{ClassField::from_ptr(
            crate::binds::mono_class_get_fields(self.class_ptr,&mut gptr as *mut *mut c_void)
        )}{
            if cf.get_name() == name{
                return Some(cf);
            }
        }
        None
    }
    /* TODO: Fix it to use the new method type
    /// Returns all methods of a class
    pub fn get_methods(&self)->Vec<Method>{
        let mut gptr = 0 as *mut std::os::raw::c_void;
        let mut res = Vec::new();
        while let Some(cf) = unsafe{Method::from_ptr(
            crate::binds::mono_class_get_methods(self.class_ptr,&mut gptr as *mut *mut c_void)
        )}{
            res.push(cf);
        }
        return res;
    }
    */
    /// Gets all types nested inside this class.
    pub fn get_nested_types(&self)->Vec<Class>{
        let mut gptr = std::ptr::null_mut::<std::os::raw::c_void>();
        let mut res = Vec::new();
        while let Some(cf) = unsafe{Class::from_ptr(
            crate::binds::mono_class_get_nested_types(self.class_ptr,&mut gptr as *mut *mut c_void)
        )}{
            res.push(cf);
        }
        res
    }
    /// Returns property with name *name* or [`None`] if it is not inside class.
    pub fn get_property_from_name(&self,name:&str)->Option<ClassProperity>{
        let cstr = CString::new(name).expect(crate::STR2CSTR_ERR);
        let res = unsafe{ClassProperity::from_ptr(
            crate::binds::mono_class_get_property_from_name(self.class_ptr, cstr.as_ptr()) 
        )};
        drop(cstr);
        res
    }
    /// Returns all properties of class *self*.
    pub fn get_properities(&self)->Vec<ClassProperity>{
        let mut gptr = std::ptr::null_mut::<std::os::raw::c_void>();
        let mut res = Vec::new();
        while let Some(cf) = unsafe{ClassProperity::from_ptr(
            crate::binds::mono_class_get_properties(self.class_ptr,&mut gptr as *mut *mut c_void)
        )}{
            res.push(cf);
        }
        res
    }
    /// Returns for use in : "NAMESPACE.NAME"
    pub fn get_name_sig(&self)->String{
        let mut namespace = self.get_namespace();
        if !namespace.is_empty(){
            namespace += ".";
        }
        let name = self.get_name();
        namespace + &name
    }
}
impl std::cmp::PartialEq for Class{
    fn eq(&self,other:&Self)->bool{
        self.class_ptr == other.class_ptr
    }
}
use crate::object::Object;
use crate::binds::MonoClassField;
/// Representation of a class field. Accessors(getters,setters and indexers) are *not* fields, but properties! For them use [`ClassProperity`]
pub struct ClassField{
    cf_ptr:*mut MonoClassField,
}
impl ClassField{
    /// Creates [`ClassField`] form *cf_ptr*. Returns [`Some(ClassField)`] if pointer is not null, and [`None`] if it is.
    /// # Safety
    /// *cf_ptr* must be either a valid pointer to [`MonoClassField`] or null pointer.
    pub unsafe fn from_ptr(cf_ptr:*mut MonoClassField)->Option<Self>{
        if cf_ptr.is_null(){
            return None;
        }
        Some(Self{cf_ptr})
    }
    /// Gets internal [`MonoClassField`] pointer.
    pub fn get_ptr(&self)->*mut MonoClassField{
        self.cf_ptr
    }
    /// Gets the name of [`ClassField`]
    /// # Example
    ///```rust
    /// let some_field_name = "someField".
    /// let some_field = some_class.get_field_from_name(some_field_name).expect("Could not find field!");
    /// let name = some_field.get_name();
    /// assert!(some_filed_name == name);
    ///```
    pub fn get_name(&self)->String{
        let cstr = unsafe{std::ffi::CString::from_raw(crate::binds::mono_field_get_name(self.get_ptr()) as *mut i8)};
        let name = cstr.to_str().expect("Could not create String from ptr").to_owned();
        drop(cstr);
        name
    }
    /// Gets metadata(???) tokens of a field. **not** it's value
    pub fn get_data(&self) -> *const ::std::os::raw::c_char{
        unsafe{crate::binds::mono_field_get_data(self.get_ptr())}
    }
    /// Returns [`Class`] this field is attached to.
    /// # Example
    ///```rust
    /// let some_field = some_class.get_field_from_name(some_field_name).expect("Could not find field!");
    /// let some_field_class = some_field.get_parent();
    /// assert!(some_field_class == some_class);
    ///```
    pub fn get_parent(&self)->Class{
        unsafe{Class::from_ptr(crate::binds:: mono_field_get_parent(self.get_ptr()))}.expect("Could not get ClassFiled of Class")
    }
    /// Gets value of a field on [`Object`] *obj*. For boxable types this value is in boxed form. 
    /// In this case call [`Object`].unbox() to retrive pointer to unboxed version of this value.
    /// # Example
    /// ## C#
    ///```csharp
    /// class SomeClass{
    ///     int someField;    
    /// }
    ///```
    /// ## Rust
    ///```rust
    /// let some_field_value_object = some_field.get_value_object(&instance_of_some_class);
    /// // Retrived value *some_field_value_object* is a boxed int, so we must unbox it.
    /// let some_field_value = some_field_value_object.unbox::<i32>()
    /// ```
    pub fn get_value_object(&self,obj:&Object)->Option<Object>{
        use crate::object::ObjectTrait;
        let dom = obj.get_domain();
        unsafe{Object::from_ptr(
            crate::binds::mono_field_get_value_object(dom.get_ptr(),self.get_ptr(),obj.get_ptr())
        )}
    }
    /// Sets value of the object field on [`Object`] to value pointed to by *value_ptr*
    /// # Example
    /// ## C#
    ///```csharp
    /// class SomeClass{
    ///     int someField;    
    /// }
    ///```
    /// ## Rust
    ///```rust
    /// let value_to_set:i32 = 11;
    /// let some_field_value_object = some_field.set_value_unsafe(&instance_of_some_class,&mut value_to_set as *mut i32 as *mut  std::os::raw::c_void);
    /// ```
    /// # Safety
    /// *value_ptr* pointer must be valid and have correct type.
    pub unsafe fn set_value_unsafe(&self,obj:&Object,value_ptr:*mut std::os::raw::c_void){
        crate::binds::mono_field_set_value(obj.get_ptr(),self.get_ptr(),value_ptr);
    }
}
use crate::interop::{InteropBox,InteropClass};
impl ClassField{
    /// Sets value of a boxable type. WARING: currently there are no checks to ensure value type and field type match.
    pub fn set_value<T:InteropBox>(&self,obj:&Object,mut val:T){
        #[cfg(not(feature = "unsafe_boxing"))]
        {
            //TODO:add safety checks
        }
        unsafe{crate::binds::mono_field_set_value(obj.get_ptr(),self.get_ptr(),&mut val as *mut T as *mut c_void)};
    }
    /// Gets value of a boxable type.
    pub fn get_value<T:InteropBox + std::marker::Copy + InteropClass>(&self,obj:&Object)->T{
        use crate::object::ObjectTrait;
        let dom = obj.get_domain();
        let obj = unsafe{Object::from_ptr(
            crate::binds::mono_field_get_value_object(dom.get_ptr(),self.get_ptr(),obj.get_ptr())
        )}.expect("Cant unbox null as value type");
        #[cfg(not(feature = "unsafe_boxing"))]
        {
            let oclass =  obj.get_class();
            let tclass = <T as InteropClass>::get_mono_class();
            if oclass != tclass{
                panic!("Tried getting value of field of type `{}` as `{}` type!",&oclass.get_name(),&tclass.get_name());
            } 
        }
        obj.unbox::<T>()
    }
    /// Sets value of field *self* on *object* to *value*
    pub fn set_value_object(&self,obj:&Object,value:&Object){
        unsafe{crate::binds::mono_field_set_value(obj.get_ptr(),self.get_ptr(),value.get_ptr() as *mut c_void)};
    }
}
use crate::binds::MonoProperty;
use crate::Exception;
use core::ptr::null_mut;
/// Representation of class property(getters,setters) *not a class field!*
pub struct ClassProperity{
    prop_ptr:*mut MonoProperty,
}
impl ClassProperity{
    /// Creates new [`ClassProperity`] from a *mut [`MonoProperty`].
    /// # Safety
    /// The *ptr* must be either null or a valid pointer to *mut [`MonoProperty`]  or null.
    pub unsafe fn from_ptr(ptr:*mut MonoProperty)->Option<ClassProperity>{
        if ptr.is_null(){
            None
        }
        else {
            Some(Self{prop_ptr:ptr})
        }
    }
    pub fn get_ptr(&self)->*mut MonoProperty{
        self.prop_ptr
    }
    ///Gets value of property *self* of *object*(pass [`None`] if static), with parmateres *params*(only for Indexers,otherwise pass empty vec)
    /// # Safety
    ///Pointers in *params* must be a valid.
    pub unsafe fn get(&self,obj:Option<Object>,params:Vec<*mut c_void>)->Result<Option<Object>,Exception>{
        let param_ptr = params.as_ptr() as *mut *mut c_void;
        let obj_ptr = match obj{
            Some(obj)=>obj.get_ptr(),
            None=>null_mut(),
        } as *mut c_void;
        use crate::binds::{MonoException,MonoObject};
        let mut exec:*mut MonoException = null_mut();
        let exec_ptr = &mut exec as *mut *mut MonoException;
        let res = crate::binds::mono_property_get_value(self.get_ptr(),obj_ptr,param_ptr,exec_ptr as *mut *mut MonoObject);
        if !exec.is_null(){
            let e = Exception::from_ptr(exec).expect("Impossible condition reached. Pointer null and not null at the same time!");
            Err(e)
        }
        else{
            Ok(Object::from_ptr(res))
        }
    }
    //TODO: consider removing get and set functions, in favour of using methods(safer and more convenient)
    /// Sets value of property *self* of *object*(pass [`None`] if static), with value at beginning of *params*, and pass any other arguments after it(only for Indexers,otherwise pass only the set value)
    ///Pointers in *params* must be a valid.
    /// # Safety
    /// Params must be a list of valid pointers and must match arguments of set method.
    pub unsafe fn set(&self,obj:Option<Object>,params:Vec<*mut c_void>)->Result<(),Exception>{
        let param_ptr = params.as_ptr() as *mut *mut c_void;
        let obj_ptr = match obj{
            Some(obj)=>obj.get_ptr(),
            None=>null_mut(),
        } as *mut c_void;
        use crate::binds::{MonoException,MonoObject};
        let mut exec:*mut MonoException = null_mut();
        let exec_ptr = &mut exec as *mut *mut MonoException;
        crate::binds::mono_property_set_value(self.get_ptr(),obj_ptr,param_ptr,exec_ptr as *mut *mut MonoObject);
        if !exec.is_null(){
            let e = Exception::from_ptr(exec).expect("Impossible condition reached. Pointer null and not null at the same time!");
            Err(e)
        }
        else{
            Ok(())
        }
    }
    /// Gets getter method of this property.
    pub fn get_get_method<T:InteropSend>(&self)->Option<Method<T>>{
        unsafe{Method::from_ptr(crate::binds::mono_property_get_get_method(self.prop_ptr))}
    }
    /// Gets setter method of this property.
    pub fn get_set_method<T:InteropSend>(&self)->Option<Method<T>>{
        unsafe{Method::from_ptr(crate::binds::mono_property_get_get_method(self.prop_ptr))}
    }
    /// Gets class this property is attached to 
    pub fn get_parent(&self)->Class{
        unsafe{Class::from_ptr(crate::binds::mono_property_get_parent(self.prop_ptr))}.expect("Cold not get class this properity is attached to")
    }
    //TODO:mono_property_get_name
}
use crate::assembly::Assembly;
use lazy_static::lazy_static;
lazy_static!{
    static ref DELEGATE:Class = {
        let img = Assembly::assembly_loaded("mscorlib").expect("Assembly mscorlib not loaded, could not get System.Delegate class!").get_image();
        Class::from_name_case(&img,"System","Delegate").expect("Could not get System.Delegate class form mscorlib!")
    };
}
// Sharing Classes between thread is safe
unsafe impl Sync for Class{} 
