#![allow(
    clippy::module_name_repetitions,
    clippy::missing_const_for_fn,
    clippy::as_ptr_cast_mut
)]
//! `wrapped_mono` is a safe, lightweight wrapper around the mono library. It allows embedding of the mono runtime inside a rust project. Inside this embedded runtime code written in languages supporting the .NET framework, such as C\# and F\#, can be run. This allows usage of libraries written in those languages, and using them as a scripting language. The mono runtime is used by many game engines, and this wrapper allows using it with projects written in Rust too.
//! # Safety
//! Most functions are safe and when invalid data is passed will fail in a controlled way with an error message. There are still some pitfalls, because not all errors can be caught without substantial overhead. Those errors are hard to come by, and should be always clearly
//! marked in the documentation(for example accessing an object after deleting it by deleting domain it is in), and easy to spot.
//! # Definitions of certain words used in documentation:
//!
//! **Managed Code** - code which runs in the runtime(e.g. C# code)
//!
//! **Unmanaged code** - code which runs outside runtime(in this case Rust code)
//!
//! More precise <a href = "https://docs.microsoft.com/en-us/dotnet/standard/managed-code">explanation</a>
//! ## Feature flags
#![doc = document_features::document_features!()]
//! Example
//! ```no_run
//! use wrapped_mono::*;
//! fn main(){
//!     // Initialise the runtime with default version(`None`), and root domian named "main_domain"
//!     let domain = jit::init("main_domain",None);
//!
//!     // Load assembly "SomeAssembly.dll"
//!     let assembly = domain.assembly_open("SomeAssembly.dll").expect("Could not load assembly!");
//!     // Get the image, the part of assembly containing executable code(classes,methods, etc.)
//!     let image = assembly.get_image();
//!     // Get class named SomeClass in SomeNamespace
//!     let class = Class::from_name(&image,"SomeNamespace","SomeClass").expect("Could not find SomeClass!");
//!     // Create an instance of this class
//!     let instance = Object::new(&domain,&class);
//!     // Creating an instance of a class DOES NOT CALL ITS CONSTRUCTOR. The constructor is a method named '.ctor', that has to be called separately
//!
//!     // Get a constructor method of SomeClass accepting an integer and a string (2 parameters)
//!     let ctor:Method<(i32,String)> = Method::get_from_name(&class,".ctor(int,System.String)",2).expect("Could not find the constructor!");
//!     // Call the constructor
//!     ctor.invoke(Some(instance.clone()),(12,"SomeString".to_owned())).expect("Got an exception while calling the constructor!");
//!     // Get a method "DoABackflip" form SomeClass with 1 parameter of type int returning a byte
//!     let met:Method<(i32,String)> = Method::get_from_name(&class,"DoABackflip",1).expect("Could not find method \"DoABackFlip\"!");
//!     // Call "DoABackflip" method on an instance
//!     let res_obj = met.invoke(Some(instance),(32,"Message".to_owned())).expect("Got an exception while calling DoABackflip!").expect("Got null from DoABackFlip");
//!     // Unbox the result to get a raw integer from a boxed integer
//!     let res = res_obj.unbox::<u8>();
//!     // Create a function with the special "invokable" attribute
//!     #[invokable]
//!     fn sqrt(input:f32)->f32{
//!         if input < 0.0{
//!             // can't get sqrt of a negative number, so create a managed exception and throw it.
//!             unsafe{Exception::arithmetic().raise()};
//!         }
//!         input.sqrt()
//!     }
//!     // Replace a method with "[MethodImplAttribute(MethodImplOptions.InternalCall)]" atribute with a rust function
//!     add_internal_call!("SomeClass::SqrtInternalCall",sqrt);
//!     // This supports all types with `InteropRecive` trait
//!     #[invokable]
//!     fn avg(input:Array<Dim1D,f32>)->f32{
//!         let mut avg = 0.0;
//!         for i in 0..input.len(){
//!             let curr = input.get([i]);// get the element at index i
//!             avg += curr/(input.len() as f32);
//!           }
//!         avg
//!     }
//!     // Replace a method with "[MethodImplAttribute(MethodImplOptions.InternalCall)]" attribute with a rust function
//!     add_internal_call!("SomeClass::AvgInternalCall",sqrt);
//!}
//! ```
pub mod bindgen;
pub mod dimensions;
pub use dimensions::*;
/// Utilities related to managed arrays.
pub mod array;
/// Functions and types related to `MonoAssembly` type.
pub mod assembly;
/// Autognerated, unsafe binds to mono library
#[doc(hidden)]
pub mod binds;
/// Representation of managed classes and utilities related to them.
pub mod class;
/// Safe representation of a delegate.
// pub mod delegate;
/// Functions and types related to `MonoDomain` type.
pub mod domain;
///Utilities related to Exceptions.
pub mod exception;
/// Functions related to garbage collection.
pub mod gc;
/// Part of assembly holding the executable code.
pub mod image;
/// Traits related to passing data between managed and unmanaged classes.
pub mod interop;
/// Functions related to Mono JIT Runtime
pub mod jit;
/// Utilities related to metadata. Bare bones and experimental.
pub mod metadata;
/// Safe representation of Methods(functions) form managed code an utilities related to managing and calling them.
pub mod method;
/// Managed string utilities.
pub mod mstring;
/// Utilities related to managed objects.
pub mod object;
/// Experimental Profiler API. Bare bones and may contain bugs.
#[cfg(feature = "profiler_api")]
#[allow(dead_code)]
pub mod profiler;
/// Safe representation of the `System.Type` type.
pub mod reflection_type;
///Functions related to getting data about and configuring mono runtime.
pub mod runtime;

mod testing;
mod tupleutilis; // Some utility traits used internally.

#[doc(inline)]
pub use array::Array;
#[doc(inline)]
pub use assembly::Assembly;
#[doc(inline)]
pub use class::{Class, ClassField, ClassProperity};
#[doc(inline)]
// pub use delegate::{Delegate, DelegateTrait};
#[doc(inline)]
pub use domain::Domain;
#[doc(inline)]
pub use exception::Exception;
#[doc(inline)]
pub use image::Image;
#[doc(inline)]
pub use interop::{InteropBox, InteropClass, InteropRecive, InteropSend};
#[doc(inline)]
pub use method::Method;
#[doc(inline)]
pub use mstring::MString;
#[doc(inline)]
pub use object::{Object, ObjectTrait};
#[doc(inline)]
pub use reflection_type::ReflectionType;
#[doc(inline)]
/// Custom macros used by `wrapped_mono`
pub use wrapped_mono_macros; // Custom macros
#[doc(inline)]
pub use wrapped_mono_macros::{add_internal_call, invokable, InteropRecive, InteropSend};
static STR2CSTR_ERR: &str = "Cold not create CString!";
static CSTR2STR_ERR: &str = "Could not convert CString to String";
