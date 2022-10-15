use crate::binds::MonoException;
use crate::binds::MonoObject;
use std::ffi::CString;
use crate::Domain;
use crate::Image;
use crate::Class;
use crate::Object;
use crate::InteropClass;
use crate::gc::{GCHandle,gc_unsafe_exit,gc_unsafe_enter};
use crate::ObjectTrait;
/// Safe representation of MonoException
pub struct Exception{
    #[cfg(not(feature = "referneced_objects"))]
    exc_ptr:*mut MonoException,
    #[cfg(feature = "referneced_objects")]
    handle:GCHandle,
} 
impl Exception{
    /// Raise exception (it can be then cathed by cath clause in managed code)
    /// # Example
    /// ## C#
    ///```csharp
    /// using System.Runtime.CompilerServices;
    /// class SomeClass{
    ///     [MethodImplAttribute(MethodImplOptions.InternalCall)]   
    ///     void ExceptionThrower();
    ///     void SomeMethod(){
    ///         try{
    ///             ExceptionThrower();
    ///         }
    ///         catch(exception){
    ///             Console.WriteLine("This will always catch exceptions raised in ExceptionThrower");
    ///         }
    ///     }
    /// }
    ///```
    /// ## Rust
    ///```rust
    /// #[invokable]
    /// fn  exception_thrower(){
    ///     let exception = Exception::not_implemented("This function will just throw exceptions!");
    ///     exception.raise();
    /// }
    ///
    pub fn raise(&self){
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{crate::binds::mono_raise_exception(self.get_ptr())};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Creates [`Exception`] of type *name* in *namespace* from *image* in *domain*
    pub fn from_name_domain(domain:&Domain,image:&Image,namespace:&str,name:&str)->Option<Self>{
        let ns_cstr = CString::new(namespace).expect(crate::STR2CSTR_ERR);
        let nm_cstr = CString::new(name).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_exception_from_name_domain(domain.get_ptr(),image.get_ptr(),ns_cstr.as_ptr(),nm_cstr.as_ptr())
        )};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        drop(ns_cstr);
        drop(nm_cstr);
        res
    }
    /// Creates [`Exception`] of type *name* in *namespace* from *image*
    pub fn from_name(image:&Image,namespace:&str,name:&str)->Option<Self>{
        let ns_cstr = CString::new(namespace).expect(crate::STR2CSTR_ERR);
        let nm_cstr = CString::new(name).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_exception_from_name(image.get_ptr(),ns_cstr.as_ptr(),nm_cstr.as_ptr())
        )};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        drop(ns_cstr);
        drop(nm_cstr);
        res
    }
    /// Creates [`Exception`] of type *name* in *namespace* from *image* with message *msg*.
    pub fn from_name_msg(image:&Image,namespace:&str,name:&str,msg:&str)->Option<Self>{
        let ns_cstr = CString::new(namespace).expect(crate::STR2CSTR_ERR);
        let nm_cstr = CString::new(name).expect(crate::STR2CSTR_ERR);
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_exception_from_name_msg(image.get_ptr(),ns_cstr.as_ptr(),nm_cstr.as_ptr(),msg_cstr.as_ptr())
        )};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        drop(msg_cstr);
        drop(ns_cstr);
        drop(nm_cstr);
        res
    }
    ///Casts object to exception. Returns [`None`] if cast failed
    pub fn cast_from_object(object:&Object)->Option<Exception>{
        if !Class::get_exception_class().is_assignable_from(&object.get_class()){
            return None;
        }
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(object.get_ptr() as *mut MonoException)};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **System.ArgumentException**
    pub fn argument_exception(arg:&str,msg:&str)->Self{
        let arg_cstr = CString::new(arg).expect(crate::STR2CSTR_ERR);
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_get_exception_argument(arg_cstr.as_ptr(),msg_cstr.as_ptr())
        )};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        drop(arg_cstr);
        drop(msg_cstr);
        res.expect("Could not create ArgumentException!")
    }
    /// Returns [`Exception`] that is instance of **System.NotImplementedException**
    pub fn not_implemented(msg:&str)->Self{
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_get_exception_not_implemented(msg_cstr.as_ptr())
        )};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        drop(msg_cstr);
        res.expect("Could not create NotImplementedException!")
    }
    /// Returns [`Exception`] that is instance of **System.ArgumentNullException**
    pub fn argument_null(msg:&str)->Self{
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_get_exception_argument_null(msg_cstr.as_ptr())
        )};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        drop(msg_cstr);
        res.expect("Could not create ArgumentNullException!")
    }
    /// Returns [`Exception`] that is instance of **System.ArgumentOutOfRangeException**
    pub fn argument_out_of_range(msg:&str)->Self{
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_get_exception_argument_out_of_range(msg_cstr.as_ptr())
        )};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        drop(msg_cstr);
        res.expect("Could not create ArgumentOutOfRangeException!")
    }
    /// Returns [`Exception`] that is instance of **System.ArithmeticException**
    pub fn arithmetic()->Self{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(crate::binds::mono_get_exception_arithmetic()).expect("Could not create ArithmeticException!")};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **System.ArrayTypeMismatchException**
    pub fn array_type_mismatch()->Self{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(crate::binds::mono_get_exception_array_type_mismatch()).expect("Could not create ArrayTypeMismatchException!")};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **System.BadImageFormatException**
    pub fn bad_image_format(msg:&str)->Self{
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_get_exception_bad_image_format(msg_cstr.as_ptr())
        )};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        drop(msg_cstr);
        res.expect("Could not create BadImageFormatException!")
    }
    /// Returns [`Exception`] that is instance of **System.CannotUnloadAppdomain**
    pub fn cannot_unload_appdomain(msg:&str)->Self{
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_get_exception_cannot_unload_appdomain(msg_cstr.as_ptr())
        )};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        drop(msg_cstr);
        res.expect("Could not create CannotUnloadAppdomainException!")
    }
    /// Returns [`Exception`] that is instance of **System.AppDomainUnloadedException**
    pub fn domain_unloaded()->Self{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(crate::binds::mono_get_exception_appdomain_unloaded()).expect("Could not create AppDomainUnloadedException!")};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **System.DivideByZeroException**
    pub fn divide_by_zero()->Self{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(crate::binds::mono_get_exception_divide_by_zero()).expect("Could not create DivideByZeroException!")};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **System.ExecutionEngineException**
    pub fn execution_engine_exception(msg:&str)->Self{
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_get_exception_execution_engine(msg_cstr.as_ptr())
        )};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        drop(msg_cstr);
        res.expect("Could not create ExecutionEngineException!")
    }
    /// Returns [`Exception`] that is instance of **System.IO.FileNotFoundException**
    pub fn file_not_found(fname:&str)->Self{
        let fname_cstr = CString::new(fname).expect(crate::STR2CSTR_ERR);
        let ms = unsafe{crate::binds::mono_string_new_wrapper(fname_cstr.as_ptr())}; 
        drop(fname_cstr); 
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_get_exception_file_not_found(ms)
        )};
         #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res.expect("Could not create System.IO.FileNotFoundException!")
    }
    /// Returns [`Exception`] that is instance of **System.IndexOutOfRangeException**
    pub fn index_out_of_range()->Self{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_get_exception_index_out_of_range()
        )}.expect("Could not create IndexOutOfRangeException");
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **System.InvalidCastException**
    pub fn invald_cast()->Self{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(crate::binds::mono_get_exception_invalid_cast()).expect("Could not create InvalidCastException!")};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **System.IO.IOException**
    pub fn io_exception(msg:&str)->Self{
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_get_exception_io(msg_cstr.as_ptr())
        )};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        drop(msg_cstr);
        res.expect("Could not create System.IO.IOException!")
    }
    /// Returns [`Exception`] that is instance of **System.MissingMethodException**
    pub fn missing_method(class_name:&str,member_name:&str)->Self{
        let class_name_cstr = CString::new(class_name).expect(crate::STR2CSTR_ERR);
        let member_name_cstr = CString::new(member_name).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_get_exception_missing_method(class_name_cstr.as_ptr(),member_name_cstr.as_ptr())
        )};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        drop(class_name_cstr);
        drop(member_name_cstr);
        res.expect("Could not create MissingMethodException!")
    }
    /// Returns [`Exception`] that is instance of **System.NullReferenceException**
    pub fn null_reference()->Self{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(crate::binds::mono_get_exception_null_reference()).expect("Could not create NullReferenceException!")};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **System.OverflowException**
    pub fn overflow()->Self{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(crate::binds::mono_get_exception_overflow()).expect("Could not create OverflowException!")};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **System.Security.SecurityException**
    pub fn security()->Self{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(crate::binds::mono_get_exception_security()).expect("Could not create System.Security.SecurityException!")};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **System.Runtime.Serialization.SerializationException**
    pub fn serialization_exception(msg:&str)->Self{
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_get_exception_serialization(msg_cstr.as_ptr())
        )};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        drop(msg_cstr);
        res.expect("Could not create System.Runtime.Serialization.SerializationException!")
    }
    /// Returns [`Exception`] that is instance of **System.StackOverflowException**
    pub fn stack_overflow()->Self{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(crate::binds::mono_get_exception_stack_overflow()).expect("Could not create StackOverflowException!")};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **System.SynchronizationLockException**
    pub fn synchronization_lock(msg:&str)->Self{
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_get_exception_synchronization_lock(msg_cstr.as_ptr())
        )};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        drop(msg_cstr);
        res.expect("Could not create SynchronizationLockException!")
    }
    /// Returns [`Exception`] that is instance of **System.Threading.ThreadAbortException**
    pub fn thread_abort()->Self{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(crate::binds::mono_get_exception_thread_abort()).expect("Could not create System.Threading.ThreadAbortException!")};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **System.Threading.ThreadStateException**
    pub fn thread_sate(msg:&str)->Self{
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_get_exception_thread_state(msg_cstr.as_ptr())
        )};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        drop(msg_cstr);
        res.expect("Could not create System.Threading.ThreadStateException!")
    }
    /// Returns [`Exception`] that is instance of **System.TypeInitializationException** with *type_name* and inner exception *inner*.
    pub fn type_initialization(type_name:&str,inner:&Exception)->Self{
        let type_name_cstr = CString::new(type_name).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_get_exception_type_initialization(type_name_cstr.as_ptr(),inner.get_ptr())
        )};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        drop(type_name_cstr);
        res.expect("Could not create System.TypeInitializationException!")
    }
    /// Returns [`Exception`] that is instance of **System.TypeLoadException**
    pub fn type_load(class_name:&str,member_name:&str)->Self{
        let class_name_cstr = CString::new(class_name).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let cn_mono_string = unsafe{crate::binds::mono_string_new_wrapper(class_name_cstr.as_ptr())};
        drop(class_name_cstr);
        let member_name_cstr = CString::new(member_name).expect(crate::STR2CSTR_ERR);
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_get_exception_type_load(cn_mono_string,member_name_cstr.as_ptr() as *mut i8)
        )};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        drop(member_name_cstr);
        res.expect("Could not create System.TypeLoadException!")
    }
    /// Returns [`Exception`] that is instance of **System.InvalidOperationException**
    pub fn invalid_operation(msg:&str)->Self{
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_get_exception_invalid_operation(msg_cstr.as_ptr())
        )};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        drop(msg_cstr);
        res.expect("Could not create System.InvalidOperationException!")
    }
    /// Returns [`Exception`] that is instance of **System.MissingFieldException**
    pub fn missing_field(class_name:&str,member_name:&str)->Self{
        let class_name_cstr = CString::new(class_name).expect(crate::STR2CSTR_ERR);
        let member_name_cstr = CString::new(member_name).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_get_exception_missing_field(class_name_cstr.as_ptr(),member_name_cstr.as_ptr() as *mut i8)
        )};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        drop(member_name_cstr);
        drop(class_name_cstr);
        res.expect("Could not create System.MissingFieldException!")
    }
    /// Returns [`Exception`] that is instance of **System.NotSupportedException**
    pub fn not_supported(msg:&str)->Self{
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(
            crate::binds::mono_get_exception_not_supported(msg_cstr.as_ptr())
        )};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        drop(msg_cstr);
        res.expect("Could not create System.NotSupportedException!")
    }
    /// Returns [`Exception`] that is instance of **System.FieldAccessException**
    pub fn field_access()->Self{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(crate::binds::mono_get_exception_field_access()).expect("Could not create System.FieldAccessException!")};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **System.MethodAccessException**
    pub fn method_access()->Self{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(crate::binds::mono_get_exception_method_access()).expect("Could not create System.MethodAccessException!")};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **System.OutOfMemoryException**
    pub fn out_of_memory()->Self{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(crate::binds::mono_get_exception_out_of_memory()).expect("Could not create System.OutOfMemoryException!")};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Creates [`Exception`] with a wraped inner [`Exception`] *inner*.
    pub fn wrapped(inner:&Self)->Self{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(crate::binds::mono_get_exception_runtime_wrapped(inner.get_ptr() as *mut MonoObject))}.expect("Colud not create a wrapped exception");
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    //TODO: implement mono_get_exception_reflection_type_load
    /// Creates [`Exception`] from a [`MonoException`] pointer
    /// # Safety
    /// *exec_ptr* mus be either null, or a valid MonoException pointer.
    pub unsafe fn from_ptr(exc_ptr:*mut MonoException)->Option<Self>{
        #[cfg(not(feature = "referneced_objects"))]
        {
            if exc_ptr.is_null(){
                None
            }
            else {Some(Self{exc_ptr})}
        }
        #[cfg(feature = "referneced_objects")]
        {
            if exc_ptr.is_null(){
                None
            }
            else {Some(Self{handle:GCHandle::create_default(exc_ptr as *mut _)})}
        }

    }
    pub fn get_ptr(&self)->*mut MonoException{
        #[cfg(not(feature = "referneced_objects"))]
        {self.exc_ptr}
        #[cfg(feature = "referneced_objects")]
        {self.handle.get_target() as *mut MonoException}
    }
}
/// Variant of except which instead of panicking will raise a managed exception.
pub fn except_managed<T:Sized>(option:Option<T>,msg:&str)->T{
    match option{
        Some(t)=>t,
        None=>{
            let exc = Exception::argument_null(&format!("Value of type: \"{}\" was null!\"{}\"",std::any::type_name::<T>(),&msg));
            exc.raise();
            panic!("Impossible condition reached. Code executed after exception thrown.");
        }
    }
}
/// Variant of except which instead of panicking will raise a managed exception.
pub fn unwrap_managed<T:Sized>(option:Option<T>)->T{
    match option{
        Some(t)=>t,
        None=>{
            let exc = Exception::argument_null(&format!("Value of type: \"{}\" was null!",std::any::type_name::<T>()));
            exc.raise();
            panic!("Impossible condition reached. Code executed after exception thrown.");
        }
    }
}
use core::fmt::Formatter;
impl core::fmt::Debug for Exception{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        let mstr = self.to_mstring().expect("Got an exception while trying to convert an exception to string!").expect("Got null intead of a string while tring to convert an exception to a string!").to_string();
        write!(f,"Exception:\"{}\"",mstr)
    }
}
use crate::MString;
impl crate::object::ObjectTrait for Exception{
    fn hash(&self)->i32{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let hash = unsafe{crate::binds::mono_object_hash(self.get_ptr() as *mut MonoObject)};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        hash
    }
    fn get_domain(&self)->crate::domain::Domain{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let dom = unsafe{crate::domain::Domain::from_ptr(crate::binds::mono_object_get_domain(self.get_ptr() as *mut MonoObject))};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        dom
    }
    fn get_size(&self)->u32{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let size = unsafe{crate::binds:: mono_object_get_size(self.get_ptr() as *mut MonoObject)};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        size
    }
    fn reflection_get_token(&self)->u32{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let tok = unsafe{crate::binds::mono_reflection_get_token(self.get_ptr() as *mut MonoObject)};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        tok
    }
    fn get_class(&self)->crate::class::Class{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let class = unsafe{crate::class::Class::from_ptr(
            crate::binds::mono_object_get_class(self.get_ptr() as *mut MonoObject)
        ).expect("Could not get class of an object")};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        class
    }
    fn to_mstring(&self)->Result<Option<MString>,Exception>{
        let mut exc:*mut crate::binds::MonoException = core::ptr::null_mut();
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{MString::from_ptr(
            crate::binds::mono_object_to_string(self.get_ptr() as *mut crate::binds::MonoObject,&mut exc as *mut *mut crate::binds::MonoException as *mut *mut crate::binds::MonoObject)
        )};
        let exc = unsafe{Exception::from_ptr(exc)};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        match exc{
            Some(e)=>Err(e),
            None=>Ok(res),
        }
    }
    fn cast_to_object(&self)->Object{
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let obj = unsafe{Object::from_ptr(self.get_ptr() as *mut MonoObject)}.unwrap(); //impossible. If exception exists, then it can be cast to an object
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        obj
    }
    fn cast_from_object(obj:&Object)->Option<Self>{
        //TODO: adjust this after including GCHandles to speed things up.
        let cast = match obj.is_inst(&<Self as InteropClass>::get_mono_class()){
            Some(cast)=>cast,
            None=>return None,
        };
        #[cfg(feature = "referneced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe{Self::from_ptr(cast.get_ptr() as *mut _)};
        #[cfg(feature = "referneced_objects")]
        gc_unsafe_exit(marker);
        res
    }
}
impl InteropClass for Exception{
    fn get_mono_class()->Class{
        Class::get_exception_class()
    }
}
impl Clone for Exception{
    fn clone(&self)->Self{
        unsafe{Self::from_ptr(self.get_ptr()).unwrap()}//If exception exists then it can't be null
    }
}
impl<O:ObjectTrait> PartialEq<O> for Exception{
    fn eq(&self,other:&O)->bool{
        self.get_ptr() as *mut _ == other.cast_to_object().get_ptr()
    }
}
impl std::fmt::Display for Exception{
    fn fmt(&self,f:&mut Formatter<'_>)->Result<(), std::fmt::Error>{
        let mstr = self.to_mstring();
        write!(f,"{}",mstr.expect("Got an exception while converting an exception String!").expect("Got null from converting exception to string!").to_string())
    }
}
