#![allow(clippy::missing_transmute_annotations)]
#![feature(once_cell_get_mut, let_chains)]

mod app;
mod config;
mod game;
mod hacks;
mod hooks;
mod interfaces;
mod macros;
mod netvar_manager;
mod utils;

use windows::Win32::Foundation::{BOOL, HMODULE, TRUE};
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

use std::ffi::c_void;

use app::App;

#[no_mangle]
extern "system" fn DllMain(module: HMODULE, reason: u32, _reserved: *mut c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            App::init_and_setup(module).expect("Failed to create and setup application")
        }
        DLL_PROCESS_DETACH => App::make_final_config_save(),
        _ => (),
    }
    TRUE
}
