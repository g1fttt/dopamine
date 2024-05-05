#![allow(clippy::missing_transmute_annotations)]

mod app;
mod game;
mod hacks;
mod hooks;
mod interfaces;
mod macros;
mod utils;

use windows::Win32::Foundation::{BOOL, HMODULE, TRUE};
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

use std::ffi::c_void;
use std::mem;

use app::App;

#[no_mangle]
extern "system" fn DllMain(module: HMODULE, reason: u32, _reserved: *mut c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            App::init_and_setup(module).expect("Failed to create and setup application")
        }
        DLL_PROCESS_DETACH => mem::drop(App::get()),
        _ => (),
    }
    TRUE
}
