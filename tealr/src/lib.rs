#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

///traits and types specific to rlua
#[cfg(feature = "rlua")]
pub mod rlu;

///traits and types specific to mlua
#[cfg(feature = "mlua")]
pub mod mlu;

mod exported_function;
mod teal_multivalue;
mod type_generator;
mod type_representation;
mod type_walker;

pub use exported_function::ExportedFunction;
pub use teal_multivalue::{TealMultiValue, TealType};
pub use type_generator::TypeGenerator;
pub use type_representation::{Direction, TypeBody, TypeName};
pub use type_walker::TypeWalker;

#[cfg(feature = "derive")]
pub use tealr_derive::{MluaTealDerive, MluaUserData, RluaTealDerive, RluaUserData, TypeName};

#[cfg(feature = "compile")]
pub use tealr_derive::compile_inline_teal;

#[cfg(any(
    feature = "embed_compiler_from_local",
    feature = "embed_compiler_from_download"
))]
pub use tealr_derive::embed_compiler;
