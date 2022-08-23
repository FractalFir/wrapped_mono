//doctest are dissabled, because they do not work with rusty_fork! whcich is required for testing mono runtime
#![cfg(not(doctest))] 
//! Lightweight Rust wrapper around mono runtime. Allows embedding mono runtime in rust projects using safe Rust API, with minimal overhead.
//! # Reqiurements
//! `wrapped_mono` is, as the name suggest, a wrapper around the mono library. This means, that it needs this library to be installed in order to be compiled.
//! If you do not have mono installed yet, you can download it from <a href="https://www.mono-project.com/download/stable/">here</a>
//! # W.I.P
//! This crate is work in progress. While it support most common features, some more obscure ones do not have a safe API yet.
//! ## Finished fetures
//! * Runtime initialization/shutdown
//Some methods of domain creation do not have wrappers yet
//! * Creating multpile domains
//! * Loading assemblies from disk
//! * Searching for a specific class in assembly
//! * Creating new object of a given type
//! * Getting method of a class
//! * Calling a method - both static and not static
//! * Utilities related to objects (getting their size, converting them to strings)
//! * Boxing and unboxing values
//! * Getting/Setting object field
//! * Cloning objects
//! * Managed string support
//! * Array creation
//! * Getting/Setting array at index
//! * Exception creating
//! * Raising Exceptions
//! * Catching Exceptions
//! * Getting common type classes
//! * Loading config files
//! * Signal chaining
//!
//! TODO:finnish filling this list 
//! ## Potential issues
//! Even tough a lot of tests are run to ensure the API works as expected, some functions may have hidden issues uncaught by the tests.
//! As this crate matures, those should be caught and fixed over time. 
//!
//! API exposed by this crate sholud not have major changes, because it tries to stay close to C mono API.
//! ## Lacking features
//! There are not a lot of features missing. Large portion of those lacking features are niche and their documentaion is sparse,
//! making developing them challenging and time consuming. Because of that, they have been postponed.
//! Those features are:
//! * Debugging API
//! * Dynamic code generation
//! * Certain fetures of mono JIT(mostly debugging)
//! * Reading of assembly meatdata
//! * Profilier (Data about preformance)
//! * Seciurity API
//! * Features related to threads
//! ## Future features
//! There are certain features that are not **missing** but could be greatly improved upon in the future. For example, there is an API for calling methods,
//! but it cold be made more safe and convinient to use. Currently, it requires creating and passing to it pointers. A macro could abstract that away,
//! allowing for less hard and error-prone usage of that feature, but it is a hard, not necesary feature, and there is little harm in comming latter.
//! 
//! Another planned but not implemented feature is ability for structs to derive `InvokePass` and `InvokeReturn` traits. Those traits allowing for easy
//! passing of values between managed and unamanged code currently have to be implemented manualy, but could be automated in the futute.
//! # Definition of ceartain words used in documentation:<br>
//! **Managed Code** - code which runs in the runtime(e.g. C# code)<br>
//! **Unmanaged code** - code which runs outside runtime(in this case Rust code).<br>
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
mod testing;
pub use macros;
#[doc(inline)]
pub use object::{Object,ObjectTrait};
#[doc(inline)]
pub use domain::Domain;
#[doc(inline)]
pub use interop::{InteropRecive,InteropSend};
#[doc(inline)]
pub use array::Array;
#[doc(inline)]
pub use class::{Class,ClassField};
#[doc(inline)]
pub use image::Image;
#[doc(inline)]
pub use method::Method;
#[doc(inline)]
pub use exception::Exception;
#[doc(inline)]
pub use mstring::MString;
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
#[doc(hide)]
static STR2CSTR_ERR:&str = "Cold not create CString!";
#[doc(hide)]
static CSTR2STR_ERR:&str = "Could not convert CString to String";