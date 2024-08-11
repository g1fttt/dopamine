#![allow(internal_features)]
#![feature(once_cell_get_mut, let_chains, ptr_as_ref_unchecked, is_none_or, core_intrinsics)]

mod app;
mod config;
mod entities;
mod features;
mod hooks;
mod ui;

use windows::Win32::Foundation::{BOOL, HMODULE, TRUE};
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

use app::App;

use std::ffi::c_void;

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
