mod client;
mod client_mode;
mod d3d9;
mod model_render;
mod surface;
mod viewmodel;
mod winapi;

use d3d9::{PresentFn, ResetFn};

use dopamine_sdk::math::{Angles, Vector};
use dopamine_sdk::utils::{Interfaces, Patterns};
use dopamine_sdk::{pcstr, Hook, HookResult, TrampolineHook, VmtHook};

use dopamine_sdk::client::{Client, ClientMode};
use dopamine_sdk::engine::{ModelRender, ModelRenderInfo};
use dopamine_sdk::render_view::ViewSetup;
use dopamine_sdk::surface::Surface;
use dopamine_sdk::{Entity, UserCommand};

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
  FindWindowA, SetWindowLongPtrW, GWLP_WNDPROC, WNDPROC,
};

use std::ffi::c_void;
use std::mem;

pub struct Hooks {
  window: HWND,
  pub(self) wnd_proc: WNDPROC,

  pub(self) override_view: VmtHook<extern "thiscall" fn(&ClientMode, &mut ViewSetup)>,
  pub(self) create_move: VmtHook<extern "thiscall" fn(&ClientMode, f32, &mut UserCommand) -> bool>,
  pub(self) do_post_screen_space_effects:
    VmtHook<extern "thiscall" fn(&ClientMode, &ViewSetup) -> bool>,

  pub(self) level_init_post_entity: VmtHook<extern "thiscall" fn(&Client)>,
  pub(self) level_shutdown: VmtHook<extern "thiscall" fn(&Client)>,

  pub(self) draw_model_execute:
    VmtHook<extern "thiscall" fn(&ModelRender, *mut c_void, &ModelRenderInfo, *mut c_void)>,

  pub(self) is_cursor_visible: VmtHook<extern "thiscall" fn(&Surface) -> bool>,
  pub(self) lock_cursor: VmtHook<extern "thiscall" fn(&Surface)>,

  pub(self) calc_viewmodel_view:
    TrampolineHook<extern "thiscall" fn(&Entity, &Entity, &Vector, &Angles)>,

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

    let window = FindWindowA(pcstr!("Valve001"), pcstr!())
      .inspect_err(|err| log::error!("Failed to find game window: {}", err))
      .unwrap();

    Self {
      window,
      wnd_proc: None,

      override_view: VmtHook::new(interfaces.client_mode, 16),
      create_move: VmtHook::new(interfaces.client_mode, 21),
      do_post_screen_space_effects: VmtHook::new(interfaces.client_mode, 39),

      level_init_post_entity: VmtHook::new(interfaces.client, 6),
      level_shutdown: VmtHook::new(interfaces.client, 7),

      draw_model_execute: VmtHook::new(interfaces.model_render, 19),

      is_cursor_visible: VmtHook::new(interfaces.surface, 53),
      lock_cursor: VmtHook::new(interfaces.surface, 62),

      calc_viewmodel_view: TrampolineHook::new(patterns.calc_viewmodel_view),

      reset_raw: patterns.d3d9_reset,
      present_raw: patterns.d3d9_present,
      reset,
      present,
    }
  }

  pub unsafe fn hook_all(&mut self) -> HookResult<()> {
    self.wnd_proc = {
      mem::transmute::<i32, WNDPROC>(SetWindowLongPtrW(
        self.window,
        GWLP_WNDPROC,
        winapi::wnd_proc as *const () as _,
      ))
    };

    self.override_view.detour_to(client_mode::override_view)?;
    self.create_move.detour_to(client_mode::create_move)?;
    self.do_post_screen_space_effects.detour_to(client_mode::do_post_screen_space_effects)?;

    self.level_init_post_entity.detour_to(client::level_init_post_entity)?;
    self.level_shutdown.detour_to(client::level_shutdown)?;

    self.draw_model_execute.detour_to(model_render::draw_model_execute)?;

    self.is_cursor_visible.detour_to(surface::is_cursor_visible)?;
    self.lock_cursor.detour_to(surface::lock_cursor)?;

    self.calc_viewmodel_view.detour_to(viewmodel::calc_viewmodel_view)?;

    dopamine_sdk::enable_all_hooks()?;

    **self.reset_raw.cast::<*mut ResetFn>() = d3d9::reset;
    **self.present_raw.cast::<*mut PresentFn>() = d3d9::present;

    Ok(())
  }

  pub unsafe fn unhook_all(&self) -> HookResult<()> {
    **self.reset_raw.cast::<*mut ResetFn>() = self.reset;
    **self.present_raw.cast::<*mut PresentFn>() = self.present;

    dopamine_sdk::disable_all_hooks()?;

    self.override_view.remove()?;
    self.create_move.remove()?;
    self.do_post_screen_space_effects.remove()?;

    self.level_init_post_entity.remove()?;
    self.level_shutdown.remove()?;

    self.draw_model_execute.remove()?;

    self.is_cursor_visible.remove()?;
    self.lock_cursor.remove()?;

    self.calc_viewmodel_view.remove()?;

    SetWindowLongPtrW(self.window, GWLP_WNDPROC, mem::transmute::<WNDPROC, i32>(self.wnd_proc));

    Ok(())
  }
}
