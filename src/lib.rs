// Some experimental features simply need to be enabled to make wrapped_mono work. They are used rarely and with special caution.
#![allow(incomplete_features)]
// Necessary for proper work of Method, usage rather simple, but bugs possible when changes to compiler are made.
#![feature(specialization)]
// used only for array sizes, in a very simple, limited manner. Should not cause troubles when updating.
#![feature(generic_const_exprs)]
// used for benchmarking
#![feature(test)]
// doctests are disabled, because they do not work with rusty_fork! which is required for testing mono runtime
#![cfg(not(doctest))] 
//! `wrapped_mono` is a lightweight wrapper around the mono runtime, allowing embedding code from languages from the .NET frameawork into rust code.
//! Besides simple warppers around most functions, this crate also contains couple traits and macros allowing easy interop between managed and unmanaged code.
//! # Safety 
//! Most functions are safe and when invalid data is passed will fail in a controlled way with an error message. There are still some pitfalls, because not all errors can be caught without substantial overhead. Those errors are hard to come by, and should be always clearly 
//! marked in the documentation(for example accessing an object after deleting it by deleting domain it is in), and be generally obvious mistakes(deleting something and then accessing it). 
//! # Definitions of certain words used in documentation:
//!
//! **Managed Code** - code which runs in the runtime(e.g. C# code)
//!
//! **Unmanaged code** - code which runs outside runtime(in this case Rust code)
//!
//! More precise explanation: <a href = "https://docs.microsoft.com/en-us/dotnet/standard/managed-code">Explanation</a>
//! ## Feature flags
#![doc = document_features::document_features!()]

/// Autognerated, unsafe binds to mono library
pub mod binds;
/// Functions related to Mono JIT Runtime
pub mod jit;
/// Functions and types related to MonoDomain type.
pub mod domain;
/// Functions and types related to MonoAssemblt type.
pub mod assembly;
/// Traits related to passing data between managed and unmanaged classes.
pub mod interop;
/// Utilities related to managed arrays.
pub mod array;
/// Utilities related to managed objects.
pub mod object;
/// Representation of managed classes and utilities related to them.
pub mod class;
/// Part of assembly holding the executable code.
pub mod image;
/// Safe representation of Methods(functions) form managed code an utilities related to managing and calling them.
pub mod method;
/// Managed string utilities.
pub mod mstring;
///Functions related to getting data about and configuring mono runtime.
pub mod runtime;
///Utilities related to Exceptions. 
pub mod exception;
/// Functions related to garbage collection.
pub mod gc;
/// Utilities related to metadata. Bare bones and experimental.
pub mod metadata;
///
pub mod delegate;

mod tupleutilis;
/// Experimental Profiler API. Bare bones and may contain bugs.
#[allow(dead_code)]
pub mod profiler;
mod testing;
pub use wrapped_mono_macros;
#[doc(inline)]
pub use object::{Object,ObjectTrait};
#[doc(inline)]
pub use domain::Domain;
#[doc(inline)]
pub use interop::{InteropRecive,InteropSend,InteropBox,InteropClass,get_mono_rep_val,ref_to_cvoid_ptr};
#[doc(inline)]
pub use array::Array;
#[doc(inline)]
pub use class::{Class,ClassField,ClassProperity};
#[doc(inline)]
pub use image::Image;
#[doc(inline)]
pub use method::{Method,MethodTrait};
#[doc(inline)]
pub use exception::{Exception,except_managed};
#[doc(inline)]
pub use mstring::MString;
#[doc(inline)]
pub use wrapped_mono_macros::{add_internal_call,invokable,InteropRecive,InteropSend};
#[doc(inline)]
pub use delegate::{Delegate,DelegateTrait};
#[doc(inline)]
pub use assembly::Assembly;
//for 0.2 TODO:create event object functionalities
//for 0.2 TODO:create delegate related functionalities
//for 0.2 TODO:create wrapper around MonoType. It is not necessary for basic functionalities, but is nice to have.
/*
    TODO: Memory leak prevention.
    Memory leaks are serious issues which would seriously reduce usability of wrapped-mono.
    Some issues may arise from wrong interpretation of documentation of mono runtime. Certain assumptions were made, which may not be true.
    (for example const char* pointer being pointers to internal memory buffers within runtime which should not be freed).
    While running tests using valgrind shows no memory leaks, this info may be wrong for 2 reasons
    1: tests are run using rusty-fork - they are separate processes an thus may or may not be supervised by valgrind(depending on how spawning another process is handled - is it checked to or not?)
    2: memory may be freed by mono when runtime stops? But that depends on mono runtime "sensing" that application is closing and automatically cleaning-up
*/
static STR2CSTR_ERR:&str = "Cold not create CString!";
static CSTR2STR_ERR:&str = "Could not convert CString to String";
#[doc(hidden)]
fn hold<T>(_:&T){}
