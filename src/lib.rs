#![cfg(not(doctest))] 
//! Rust wrapper around mono runtime. Allows embbeding mono runtime in rust projects using safe Rust API.
//! # Definition of ceartain words used in documentation:<br>
//! **Managed Code** - code which runs in the runtime(e.g. C# code)<br>
//! **Unmanaged code** - code which runs outside runtime(in this case Rust code).<br>
//! More precise explanation: <a href = "https://docs.microsoft.com/en-us/dotnet/standard/managed-code">Explanation</a>
//! # Features
//! ## Automatic wrappers around functions
//! Wrapped_mono provides automatic function wrapper creating functionality using #[[invokable]] and add_internall_call! macros. 
//! They allow easy exposing of rust functions to managed code.
//! For example function 
//! ```rust
//!#[invokable]
//!fn get_string_length(input:String)->i32{
//!     return input.len();
//!}
//! ```
//! Takes as it's input Rust's String type, which stores data in a diffrent way than MonoString type used by MonoRuntime.
//! It means that it needs converting. This is exacly what #[[invokable]] macro does. It creates a special wrapper function
//! which converts arguments from type used in manged code to type used by unmanged code(In this example, MonoString* to String)
//! This macro also converts return values of functions.
/// Autognerated, unsafe binds to mono library
pub mod binds;
/// Functions realted to Mono JIT Runtime
pub mod jit;
/// Functions and types related to MonoDomain type.
pub mod domain;
/// Functions and types related to MonoAssemblt type.
pub mod assembly;
/// Trait related to converting Rust's types and MonoRuntime's types when exposing rust functios to managed code
pub mod invokable;
/// Utilities related to arrays.
pub mod array;
///
pub mod object;
///
pub mod class;
///
pub mod image;
///
pub mod method;
///
pub mod mstring;
///
pub mod runtime;
///Utilities realted to Exceptions in managed code. 
pub mod exception;
mod testing;
pub use macros;
pub use object::{Object,ObjectTrait};
pub use domain::Domain;
pub use invokable::*;
pub use array::Array;
pub use class::Class;
pub use class::ClassField;
pub use image::Image;
pub use method::Method;
pub use exception::Exception;
