use crate::binds::MonoObject;
use crate::gc::{gc_unsafe_enter, gc_unsafe_exit, GCHandle};
use crate::{Class, Domain, Image, InteropClass, ObjectTrait};
use std::ffi::CString;
/// Safe representation of `MonoException`.
pub struct Exception {
    #[cfg(not(feature = "referenced_objects"))]
    exc_ptr: *mut crate::binds::MonoException,
    #[cfg(feature = "referenced_objects")]
    handle: GCHandle,
}
impl Exception {
    /// Raise exception (it can be then caught by catch clause in managed code)
    /// # Safety
    /// This function is extremely unsafe, because when it is called, drop functions of local variables **are not** automatically  called.
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
    /// # use wrapped_mono::{invokable,Exception};
    /// #[invokable]
    /// fn exception_thrower(){
    ///     let some_local_data = vec![12,2,35,32];
    ///     let exception = Exception::not_implemented("This function will just throw exceptions!");
    ///     // Everything needs to be dropped before exception is thrown!
    ///     drop(some_local_data);
    ///     unsafe{exception.raise()};
    /// }
    #[allow(clippy::missing_panics_doc)] // This may never panic, because it is impossible to return from `raise`
    pub unsafe fn raise(&self) -> ! {
        unsafe { crate::binds::mono_raise_exception(self.get_ptr().cast()) };
        panic!("After an exception is thrown, nothing should happen.");
    }
    /// Creates [`Exception`] of type *name* in *namespace* from *image* in *domain*'
    #[must_use]
    pub fn from_name_domain(
        domain: &Domain,
        image: Image,
        namespace: &str,
        name: &str,
    ) -> Option<Self> {
        let namespace_cstr = CString::new(namespace).expect(crate::STR2CSTR_ERR);
        let name_cstr = CString::new(name).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr(
                crate::binds::mono_exception_from_name_domain(
                    domain.get_ptr(),
                    image.get_ptr(),
                    namespace_cstr.as_ptr(),
                    name_cstr.as_ptr(),
                )
                .cast(),
            )
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        let _ = namespace_cstr;
        let _ = name_cstr;
        res
    }
    /// Creates [`Exception`] of type *name* in *namespace* from *image*
    #[must_use]
    pub fn from_name(image: Image, namespace: &str, name: &str) -> Option<Self> {
        let namespace_cstr = CString::new(namespace).expect(crate::STR2CSTR_ERR);
        let name_cstr = CString::new(name).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr(
                crate::binds::mono_exception_from_name(
                    image.get_ptr(),
                    namespace_cstr.as_ptr(),
                    name_cstr.as_ptr(),
                )
                .cast(),
            )
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        let _ = namespace_cstr;
        let _ = name_cstr;
        res
    }
    /// Creates [`Exception`] of type *name* in *namespace* from *image* with message *msg*.
    #[must_use]
    pub fn from_name_msg(image: Image, namespace: &str, name: &str, msg: &str) -> Option<Self> {
        let namespace_cstr = CString::new(namespace).expect(crate::STR2CSTR_ERR);
        let name_cstr = CString::new(name).expect(crate::STR2CSTR_ERR);
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr(
                crate::binds::mono_exception_from_name_msg(
                    image.get_ptr(),
                    namespace_cstr.as_ptr(),
                    name_cstr.as_ptr(),
                    msg_cstr.as_ptr(),
                )
                .cast(),
            )
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        let _ = msg_cstr;
        let _ = namespace_cstr;
        let _ = name_cstr;
        res
    }
    /// Returns [`Exception`] that is instance of **`System.ArgumentException`**
    #[must_use]
    pub fn argument_exception(arg: &str, msg: &str) -> Self {
        let arg_cstr = CString::new(arg).expect(crate::STR2CSTR_ERR);
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(
                crate::binds::mono_get_exception_argument(arg_cstr.as_ptr(), msg_cstr.as_ptr())
                    .cast(),
            )
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        let _ = arg_cstr;
        let _ = msg_cstr;
        res
    }
    /// Returns [`Exception`] that is instance of **`System.NotImplementedException`**
    #[must_use]
    pub fn not_implemented(msg: &str) -> Self {
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(
                crate::binds::mono_get_exception_not_implemented(msg_cstr.as_ptr()).cast(),
            )
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        let _ = msg_cstr;
        res
    }
    /// Returns [`Exception`] that is instance of **`System.ArgumentNullException`**
    #[must_use]
    pub fn argument_null(msg: &str) -> Self {
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(
                crate::binds::mono_get_exception_argument_null(msg_cstr.as_ptr()).cast(),
            )
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        let _ = msg_cstr;
        res
    }
    /// Returns [`Exception`] that is instance of **`System.ArgumentOutOfRangeException`**
    #[must_use]
    pub fn argument_out_of_range(msg: &str) -> Self {
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(
                crate::binds::mono_get_exception_argument_out_of_range(msg_cstr.as_ptr()).cast(),
            )
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        let _ = msg_cstr;
        res
    }
    /// Returns [`Exception`] that is instance of **`System.ArithmeticException`**
    #[must_use]
    pub fn arithmetic() -> Self {
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(crate::binds::mono_get_exception_arithmetic().cast())
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **`System.ArrayTypeMismatchException`**
    #[must_use]
    pub fn array_type_mismatch() -> Self {
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(crate::binds::mono_get_exception_array_type_mismatch().cast())
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **`System.BadImageFormatException`**
    #[must_use]
    pub fn bad_image_format(msg: &str) -> Self {
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(
                crate::binds::mono_get_exception_bad_image_format(msg_cstr.as_ptr()).cast(),
            )
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        let _ = msg_cstr;
        res
    }
    /// Returns [`Exception`] that is instance of **`System.CannotUnloadAppdomain`**
    #[must_use]
    pub fn cannot_unload_appdomain(msg: &str) -> Self {
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(
                crate::binds::mono_get_exception_cannot_unload_appdomain(msg_cstr.as_ptr()).cast(),
            )
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        let _ = msg_cstr;
        res
    }
    /// Returns [`Exception`] that is instance of **`System.AppDomainUnloadedException`**
    #[must_use]
    pub fn domain_unloaded() -> Self {
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(crate::binds::mono_get_exception_appdomain_unloaded().cast())
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **`System.DivideByZeroException`**
    #[must_use]
    pub fn divide_by_zero() -> Self {
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(crate::binds::mono_get_exception_divide_by_zero().cast())
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **`System.ExecutionEngineException`**
    #[must_use]
    pub fn execution_engine_exception(msg: &str) -> Self {
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(
                crate::binds::mono_get_exception_execution_engine(msg_cstr.as_ptr()).cast(),
            )
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        let _ = msg_cstr;
        res
    }
    /// Returns [`Exception`] that is instance of **`System.IO.FileNotFoundException`**
    #[must_use]
    pub fn file_not_found(fname: &str) -> Self {
        let fname_cstr = CString::new(fname).expect(crate::STR2CSTR_ERR);
        let ms = unsafe { crate::binds::mono_string_new_wrapper(fname_cstr.as_ptr()) };
        let _ = fname_cstr;
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(crate::binds::mono_get_exception_file_not_found(ms).cast())
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **`System.IndexOutOfRangeException`**
    #[must_use]
    pub fn index_out_of_range() -> Self {
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(crate::binds::mono_get_exception_index_out_of_range().cast())
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **`System.InvalidCastException`**
    #[must_use]
    pub fn invalid_cast() -> Self {
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(crate::binds::mono_get_exception_invalid_cast().cast())
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **`System.IO.IOException`**
    #[must_use]
    pub fn io_exception(msg: &str) -> Self {
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(crate::binds::mono_get_exception_io(msg_cstr.as_ptr()).cast())
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        let _ = msg_cstr;
        res
    }
    /// Returns [`Exception`] that is instance of **`System.MissingMethodException`**
    #[must_use]
    pub fn missing_method(class_name: &str, member_name: &str) -> Self {
        let class_name_cstr = CString::new(class_name).expect(crate::STR2CSTR_ERR);
        let member_name_cstr = CString::new(member_name).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(
                crate::binds::mono_get_exception_missing_method(
                    class_name_cstr.as_ptr(),
                    member_name_cstr.as_ptr(),
                )
                .cast(),
            )
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        let _ = class_name_cstr;
        let _ = member_name_cstr;
        res
    }
    /// Returns [`Exception`] that is instance of **`System.NullReferenceException`**
    #[must_use]
    pub fn null_reference() -> Self {
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(crate::binds::mono_get_exception_null_reference().cast())
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **`System.OverflowException`**
    #[must_use]
    pub fn overflow() -> Self {
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res =
            unsafe { Self::from_ptr_unchecked(crate::binds::mono_get_exception_overflow().cast()) };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **`System.Security.SecurityException`**
    #[must_use]
    pub fn security() -> Self {
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res =
            unsafe { Self::from_ptr_unchecked(crate::binds::mono_get_exception_security().cast()) };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **`System.Runtime.Serialization.SerializationException`**
    #[must_use]
    pub fn serialization_exception(msg: &str) -> Self {
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(
                crate::binds::mono_get_exception_serialization(msg_cstr.as_ptr()).cast(),
            )
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        let _ = msg_cstr;
        res
    }
    /// Returns [`Exception`] that is instance of **`System.StackOverflowException`**
    #[must_use]
    pub fn stack_overflow() -> Self {
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(crate::binds::mono_get_exception_stack_overflow().cast())
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **`System.SynchronizationLockException`**
    #[must_use]
    pub fn synchronization_lock(msg: &str) -> Self {
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(
                crate::binds::mono_get_exception_synchronization_lock(msg_cstr.as_ptr()).cast(),
            )
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        let _ = msg_cstr;
        res
    }
    /// Returns [`Exception`] that is instance of **`System.Threading.ThreadAbortException`**
    #[must_use]
    pub fn thread_abort() -> Self {
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(crate::binds::mono_get_exception_thread_abort().cast())
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **`System.Threading.ThreadStateException`**
    #[must_use]
    pub fn thread_sate(msg: &str) -> Self {
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(
                crate::binds::mono_get_exception_thread_state(msg_cstr.as_ptr()).cast(),
            )
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        let _ = msg_cstr;
        res
    }
    /// Returns [`Exception`] that is instance of **`System.TypeInitializationException`** with *`type_name`* and inner exception *`inner`*.
    #[must_use]
    pub fn type_initialization(type_name: &str, inner: &Self) -> Self {
        let type_name_cstr = CString::new(type_name).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(
                crate::binds::mono_get_exception_type_initialization(
                    type_name_cstr.as_ptr(),
                    inner.get_ptr().cast(),
                )
                .cast(),
            )
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        let _ = type_name_cstr;
        res
    }
    /// Returns [`Exception`] that is instance of **`System.TypeLoadException`**
    #[must_use]
    pub fn type_load(class_name: &str, member_name: &str) -> Self {
        let class_name_cstr = CString::new(class_name).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let cn_mono_string =
            unsafe { crate::binds::mono_string_new_wrapper(class_name_cstr.as_ptr()) };
        let _ = class_name_cstr;
        let member_name_cstr = CString::new(member_name).expect(crate::STR2CSTR_ERR);
        let res = unsafe {
            Self::from_ptr_unchecked(
                crate::binds::mono_get_exception_type_load(
                    cn_mono_string,
                    member_name_cstr.as_ptr() as *mut i8,
                )
                .cast(),
            )
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        let _ = member_name_cstr;
        res
    }
    /// Returns [`Exception`] that is instance of **`System.InvalidOperationException`**
    #[must_use]
    pub fn invalid_operation(msg: &str) -> Self {
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(
                crate::binds::mono_get_exception_invalid_operation(msg_cstr.as_ptr()).cast(),
            )
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        let _ = msg_cstr;
        res
    }
    /// Returns [`Exception`] that is instance of **`System.MissingFieldException`**
    #[must_use]
    pub fn missing_field(class_name: &str, member_name: &str) -> Self {
        let class_name_cstr = CString::new(class_name).expect(crate::STR2CSTR_ERR);
        let member_name_cstr = CString::new(member_name).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(
                crate::binds::mono_get_exception_missing_field(
                    class_name_cstr.as_ptr(),
                    member_name_cstr.as_ptr() as *mut i8,
                )
                .cast(),
            )
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        let _ = member_name_cstr;
        let _ = class_name_cstr;
        res
    }
    /// Returns [`Exception`] that is instance of **`System.NotSupportedException`**
    #[must_use]
    pub fn not_supported(msg: &str) -> Self {
        let msg_cstr = CString::new(msg).expect(crate::STR2CSTR_ERR);
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(
                crate::binds::mono_get_exception_not_supported(msg_cstr.as_ptr()).cast(),
            )
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        let _ = msg_cstr;
        res
    }
    /// Returns [`Exception`] that is instance of **`System.FieldAccessException`**
    #[must_use]
    pub fn field_access() -> Self {
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(crate::binds::mono_get_exception_field_access().cast())
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **`System.MethodAccessException`**
    #[must_use]
    pub fn method_access() -> Self {
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(crate::binds::mono_get_exception_method_access().cast())
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Returns [`Exception`] that is instance of **`System.OutOfMemoryException`**
    #[must_use]
    pub fn out_of_memory() -> Self {
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(crate::binds::mono_get_exception_out_of_memory().cast())
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        res
    }
    /// Creates [`Exception`] with a wrapped inner [`Exception`] *inner*.
    #[must_use]
    pub fn wrapped(inner: &Self) -> Self {
        #[cfg(feature = "referenced_objects")]
        let marker = gc_unsafe_enter();
        let res = unsafe {
            Self::from_ptr_unchecked(
                crate::binds::mono_get_exception_runtime_wrapped(inner.get_ptr().cast()).cast(),
            )
        };
        #[cfg(feature = "referenced_objects")]
        gc_unsafe_exit(marker);
        res
    }
}
/// Variant of except which instead of panicking will raise a managed exception.
pub(crate) fn except_managed<T: Sized>(option: Option<T>, msg: &str) -> T {
    option.map_or_else(
        || {
            let exc = Exception::argument_null(&format!(
                "Value of type: \"{}\" was null!\"{}\"",
                std::any::type_name::<T>(),
                &msg
            ));
            unsafe {
                exc.raise();
            }
        },
        |t| t,
    )
}
/*
/// Variant of except which instead of panicking will raise a managed exception.
pub(crate) fn unwrap_managed<T: Sized>(option: Option<T>) -> T {
    if let Some(t) = option {
        t
    } else {
        let exc = Exception::argument_null(&format!(
            "Value of type: \"{}\" was null!",
            std::any::type_name::<T>()
        ));
        unsafe{exc.raise();}
    }
}*/
use core::fmt::Formatter;
impl core::fmt::Debug for Exception {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mstr = self
            .to_mstring()
            .expect("Got an exception while trying to convert an exception to string!")
            .expect(
                "Got null instead of a string while trying to convert an exception to a string!",
            )
            .to_string();
        write!(f, "Exception:\"{mstr}\"")
    }
}
impl crate::object::ObjectTrait for Exception {
    #[must_use]
    unsafe fn from_ptr_unchecked(exc_ptr: *mut MonoObject) -> Self {
        #[cfg(not(feature = "referenced_objects"))]
        {
            Self {
                exc_ptr: exc_ptr.cast(),
            }
        }
        #[cfg(feature = "referenced_objects")]
        {
            Self {
                handle: GCHandle::create_default(exc_ptr.cast()),
            }
        }
    }
    #[must_use]
    fn get_ptr(&self) -> *mut MonoObject {
        #[cfg(not(feature = "referenced_objects"))]
        {
            self.exc_ptr.cast()
        }
        #[cfg(feature = "referenced_objects")]
        {
            self.handle.get_target().cast()
        }
    }
}
impl InteropClass for Exception {
    fn get_mono_class() -> Class {
        Class::get_exception_class()
    }
}
impl Clone for Exception {
    fn clone(&self) -> Self {
        unsafe { Self::from_ptr_unchecked(self.get_ptr().cast()) } //If exception exists then it can't be null
    }
}
impl<O: ObjectTrait> PartialEq<O> for Exception {
    fn eq(&self, other: &O) -> bool {
        self.get_ptr().cast() == other.get_ptr()
    }
}
impl std::fmt::Display for Exception {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mstr = self.to_mstring();
        write!(
            f,
            "{}",
            mstr.expect("Got an exception while converting an exception String!")
                .expect("Got null from converting exception to string!")
                .to_string()
        )
    }
}
