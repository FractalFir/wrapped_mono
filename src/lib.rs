// Some experimental features simply need to be enabled to make wrapped_mono work. They are used rarely and with special caution.
#![allow(incomplete_features)]
// Necessary for proper work of Method, usage rather simple, but bugs possible when changes to compiler are made.
 #![feature(specialization)]
// used only for array sizes, in a very simple, limited manner. Should not cause troubles when updating.
// #![feature(generic_const_exprs)]
// used for benchmarking
// #![feature(test)]
// doctests are disabled, because they do not work with rusty_fork! which is required for testing mono runtime
#![cfg(not(doctest))]
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
pub mod dimensions;
#[doc(inline)]
pub use dimensions::*;
/// Utilities related to managed arrays.
pub mod array;
/// Functions and types related to MonoAssembly type.
pub mod assembly;
/// Autognerated, unsafe binds to mono library
pub mod binds;
/// Representation of managed classes and utilities related to them.
pub mod class;
/// Safe representation of a delegate.
pub mod delegate;
/// Functions and types related to MonoDomain type.
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
pub use delegate::{Delegate, DelegateTrait};
#[doc(inline)]
pub use domain::Domain;
#[doc(inline)]
pub use exception::{except_managed, Exception};
#[doc(inline)]
pub use image::Image;
#[doc(inline)]
pub use interop::{InteropBox, InteropClass, InteropRecive, InteropSend};
#[doc(inline)]
pub use method::{Method, MethodTrait};
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
static STR2CSTR_ERR: &str = "Cold not create CString!";
static CSTR2STR_ERR: &str = "Could not convert CString to String";
#[doc(hidden)]
fn hold<T>(_: &T) {}
