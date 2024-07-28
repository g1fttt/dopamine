#![feature(let_chains, new_uninit, maybe_uninit_uninit_array, maybe_uninit_array_assume_init)]

pub mod game;
mod interfaces;
mod netvar_manager;
mod patterns;

pub use interfaces::*;
pub use netvar_manager::*;
pub use patterns::*;
