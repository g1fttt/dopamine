#![allow(clippy::missing_transmute_annotations, dead_code)]
#![feature(
    once_cell_get_mut,
    let_chains,
    new_uninit,
    maybe_uninit_uninit_array,
    maybe_uninit_array_assume_init
)]

mod app;
mod config;
mod game;
mod hacks;
mod hooks;
mod interfaces;
mod macros;
mod material_creator;
mod netvar_manager;
mod patterns;
mod utils;

use windows::Win32::Foundation::{BOOL, HMODULE, TRUE};
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

use std::ffi::c_void;

use app::App;

#[no_mangle]
extern "system" fn DllMain(module: HMODULE, reason: u32, _reserved: *mut c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            App::on_process_attach(module).expect("Failed to create and setup application")
        }
        DLL_PROCESS_DETACH => App::on_process_detach(),
        _ => (),
    }
    TRUE
}
