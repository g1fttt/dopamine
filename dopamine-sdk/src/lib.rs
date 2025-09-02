#![feature(maybe_uninit_array_assume_init, fn_ptr_trait)]

mod color;
mod game;
mod hooks;
pub mod interfaces;
mod macros;
pub mod math;
pub mod utils;

pub use color::*;
pub use game::*;
pub use hooks::*;
