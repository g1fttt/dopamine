#![allow(clippy::missing_safety_doc)] // FIXME
#![feature(fn_ptr_trait, stmt_expr_attributes)]

mod color;
mod hook;
pub mod macros;

pub use color::*;
pub use hook::*;
