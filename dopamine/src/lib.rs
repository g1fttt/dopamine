#![allow(internal_features)]
#![feature(once_cell_get_mut, str_from_raw_parts, core_intrinsics /* reserved for debug purpose */)]

mod app;
mod config;
mod entities;
mod features;
mod hooks;
mod logger;
mod ui;

use windows::Win32::Foundation::{HMODULE, TRUE};
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

use windows::core::BOOL;

use app::App;
use logger::Logger;

use std::ffi::c_void;

#[unsafe(no_mangle)]
extern "system" fn DllMain(module: HMODULE, reason: u32, _reserved: *mut c_void) -> BOOL {
  match reason {
    DLL_PROCESS_ATTACH => {
      let _ = std::fs::create_dir("dopamine");

      logger::init(Logger::PATH).unwrap();

      let _ = App::on_process_attach(module)
        .inspect_err(|err| log::error!("Failed to create and setup App instance: {err}"));
    }
    DLL_PROCESS_DETACH => App::on_process_detach(),
    _ => (),
  }
  TRUE
}
