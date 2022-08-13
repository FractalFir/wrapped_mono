//! Rust wrapper around mono runtime
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
#[cfg(test)]
mod testing;
pub use macros;
pub use object::{Object,ObjectTrait};
pub use domain::Domain;
pub use invokable::*;
pub use array::Array;
pub use class::Class;
pub use image::Image;
pub use method::Method;
