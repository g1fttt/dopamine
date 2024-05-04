#![allow(clippy::missing_transmute_annotations)]

mod app;
mod game;
mod hacks;
mod hooks;
mod interfaces;
mod macros;
mod utils;

use winapi::shared::minwindef::{BOOL, DWORD, HMODULE, LPVOID, TRUE};
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

use std::mem;

use app::App;

#[no_mangle]
extern "system" fn DllMain(module: HMODULE, reason: DWORD, _reserved: LPVOID) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => App::create_and_setup(module),
        DLL_PROCESS_DETACH => mem::drop(App::get()),
        _ => (),
    }
    TRUE
}
