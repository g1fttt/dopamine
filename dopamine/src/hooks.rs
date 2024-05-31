mod client;
mod client_mode;
mod d3d9;
mod model_render;
mod winapi;

use crate::interfaces::Interfaces;
use crate::patterns::Patterns;
use crate::pcstr;
use crate::utils::VMTHook;

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
  FindWindowA, SetWindowLongPtrW, GWLP_WNDPROC, WNDPROC,
};

use d3d9::{PresentFn, ResetFn};

use std::ffi::c_void;
use std::mem;

pub struct Hooks {
  window: HWND,
  pub(self) wnd_proc: WNDPROC,

  pub(self) create_move: VMTHook,
  pub(self) do_post_screen_space_effects: VMTHook,

  pub(self) level_init_post_entity: VMTHook,
  pub(self) level_shutdown: VMTHook,

  pub(self) draw_model_execute: VMTHook,

  reset_raw: *mut c_void,
  present_raw: *mut c_void,
  pub(self) reset: ResetFn,
  pub(self) present: PresentFn,
}

impl Hooks {
  pub unsafe fn create() -> Self {
    let interfaces = Interfaces::get();
    let patterns = Patterns::get();

    let reset = **patterns.d3d9_reset.cast::<*const ResetFn>();
    let present = **patterns.d3d9_present.cast::<*const PresentFn>();

    Self {
      window: FindWindowA(pcstr!("Valve001"), pcstr!()),
      wnd_proc: None,

      create_move: VMTHook::new(interfaces.client_mode, 21),
      do_post_screen_space_effects: VMTHook::new(interfaces.client_mode, 39),

      level_init_post_entity: VMTHook::new(interfaces.client, 6),
      level_shutdown: VMTHook::new(interfaces.client, 7),

      draw_model_execute: VMTHook::new(interfaces.model_render, 19),

      reset_raw: patterns.d3d9_reset,
      present_raw: patterns.d3d9_present,
      reset,
      present,
    }
  }

  pub unsafe fn hook_all(&mut self) -> windows::core::Result<()> {
    self.wnd_proc = {
      #[allow(clippy::fn_to_numeric_cast)]
      mem::transmute(SetWindowLongPtrW(
        self.window,
        GWLP_WNDPROC,
        winapi::wnd_proc as _,
      ))
    };

    self.create_move.hook(client_mode::create_move as _)?;
    self
      .do_post_screen_space_effects
      .hook(client_mode::do_post_screen_space_effects as _)?;

    self
      .level_init_post_entity
      .hook(client::level_init_post_entity as _)?;
    self.level_shutdown.hook(client::level_shutdown as _)?;

    self
      .draw_model_execute
      .hook(model_render::draw_model_execute as _)?;

    **self.reset_raw.cast::<*mut ResetFn>() = d3d9::reset;
    **self.present_raw.cast::<*mut PresentFn>() = d3d9::present;

    Ok(())
  }

  pub unsafe fn unhook_all(&self) -> windows::core::Result<()> {
    **self.reset_raw.cast::<*mut ResetFn>() = self.reset;
    **self.present_raw.cast::<*mut PresentFn>() = self.present;

    self.create_move.unhook()?;
    self.do_post_screen_space_effects.unhook()?;

    self.level_init_post_entity.unhook()?;
    self.level_shutdown.unhook()?;

    self.draw_model_execute.unhook()?;

    SetWindowLongPtrW(self.window, GWLP_WNDPROC, mem::transmute(self.wnd_proc));

    Ok(())
  }
}
