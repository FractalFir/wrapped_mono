//Experimental features:
// Necesary for proper work of Method, usage rather simple, but bugs possible when changes to compiler are made.
#![feature(specialization)]
//used only for array sizes, in a very simple, limited manner. Should not cause trubles when updating.
#![feature(generic_const_exprs)]
//doctest are dissabled, because they do not work with rusty_fork! whcich is required for testing mono runtime
#![cfg(not(doctest))] 
//! `wrapped_mono` is a lightweight wrapper around the mono runtime, allowing emmbeding code from lagnages from the .NET frameawork into rust code.
//! Besides simple warppers around most functions, this crate also contains copule tratis and macros allowing easy interop between managed and unmanged code.
//! #Safety 
//! Most functions are safe and when invalid data is passed will fail in a controled way with an error message. Some error checks hoewer have ceratin, small preformance inpact,
//! and can be disabled. There are still some pitfalls, because not all errors can be caught without substanntial overhead. Those errors are hard to come by, and are almost always clearly 
//! marked in the documentaion(for example accesing an object after delting it by delting domain it is in). 
//! # Definitions of ceartain words used in documentation:
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
/// Functions realted to Mono JIT Runtime
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
/// Representaion of managed classes and utilities related to them.
pub mod class;
/// Part of assembly holding the executable code.
pub mod image;
/// Safe representaion of Methods(functions) form managed code an utilities related to managing and calling them.
pub mod method;
/// Managed string utilities.
pub mod mstring;
///Functions related to getting data about and configuring mono runtime.
pub mod runtime;
///Utilities realted to Exceptions. 
pub mod exception;
/// Funcrions related to garbage collection.
pub mod gc;
/// Utilities related to metadata.
pub mod metadata;

mod tupleutilis;
pub mod profiler;
mod testing;
pub use macros;
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
pub use exception::Exception;
#[doc(inline)]
pub use mstring::MString;
#[doc(inline)]
pub use macros::{add_internal_call,invokable,method_invoke,InteropRecive,InteropSend};
//for 0.2 TODO:create event object functionalites
//for 0.2 TODO:create delegate related functionalites
//for 0.2 TODO:create wrapper around MonoType. It is not necesary for basic functionalities, but is nice to have.
/*
    TODO: Memory leak prevention.
    Memory leaks are serius issues which would seriously reduce usability of wrapped-mono, and memory is mostly related propely.
    Some issues may arise from wrong interpretaion of documentaion of mono runtime. Certain assumptions were made, which may not be true.
    (for example const char* pointer being pointers to internal memory buffers within runtime which should not be freed).
    While running tests using valgrind shows no memory leaks, this info may be wrong for 2 reasons
    1: tests are run using rusty-fork - they are separate proceses an thus may or may not be superivsed by valgrind(depending on how spawning another process is handled - is it checked to or not?)
    2: memory may be freed by mono when runtime stops? But that depends on mono runtime "sensing" that application is closing and automaticaly cleaning-up
*/
static STR2CSTR_ERR:&str = "Cold not create CString!";
static CSTR2STR_ERR:&str = "Could not convert CString to String";
fn hold<T>(_:&T){}