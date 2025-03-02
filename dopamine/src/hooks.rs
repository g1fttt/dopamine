mod client;
mod client_mode;
mod d3d9;
mod model_render;
mod surface;
mod viewmodel;
mod winapi;

use d3d9::{PresentFn, ResetFn};

use dopamine_sdk::math::{Angles, Vector3D};
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

  pub(self) reset: TrampolineHook<ResetFn>,
  pub(self) present: TrampolineHook<PresentFn>,

  pub(self) override_view: VmtHook<extern "fastcall" fn(&ClientMode, &mut ViewSetup)>,
  pub(self) create_move: VmtHook<extern "fastcall" fn(&ClientMode, f32, &mut UserCommand) -> bool>,
  pub(self) do_post_screen_space_effects:
    VmtHook<extern "fastcall" fn(&ClientMode, &ViewSetup) -> bool>,

  pub(self) level_init_post_entity: VmtHook<extern "fastcall" fn(&Client)>,
  pub(self) level_shutdown: VmtHook<extern "fastcall" fn(&Client)>,

  pub(self) draw_model_execute:
    VmtHook<extern "fastcall" fn(&ModelRender, *mut c_void, &ModelRenderInfo, *mut c_void)>,

  pub(self) is_cursor_visible: VmtHook<extern "fastcall" fn(&Surface) -> bool>,
  pub(self) lock_cursor: VmtHook<extern "fastcall" fn(&Surface)>,

  pub(self) calc_viewmodel_view:
    TrampolineHook<extern "fastcall" fn(&Entity, &Entity, &Vector3D, &Angles)>,
}

impl Hooks {
  pub unsafe fn create() -> Self {
    let interfaces = Interfaces::get();
    let patterns = Patterns::get();

    let window = FindWindowA(pcstr!("Valve001"), pcstr!())
      .inspect_err(|err| log::error!("Failed to find game window: {}", err))
      .unwrap();

    Self {
      window,
      wnd_proc: None,

      reset: TrampolineHook::new(patterns.d3d9_reset),
      present: TrampolineHook::new(patterns.d3d9_present),

      override_view: VmtHook::new(interfaces.client_mode, 16),
      create_move: VmtHook::new(interfaces.client_mode, 21),
      do_post_screen_space_effects: VmtHook::new(interfaces.client_mode, 39),

      level_init_post_entity: VmtHook::new(interfaces.client, 6),
      level_shutdown: VmtHook::new(interfaces.client, 7),

      draw_model_execute: VmtHook::new(interfaces.model_render, 19),

      is_cursor_visible: VmtHook::new(interfaces.surface, 53),
      lock_cursor: VmtHook::new(interfaces.surface, 62),

      calc_viewmodel_view: TrampolineHook::new(patterns.calc_viewmodel_view),
    }
  }

  pub unsafe fn hook_all(&mut self) -> HookResult<()> {
    self.wnd_proc = {
      mem::transmute::<isize, WNDPROC>(SetWindowLongPtrW(
        self.window,
        GWLP_WNDPROC,
        winapi::wnd_proc as *const () as _,
      ))
    };

    self.reset.detour_to(d3d9::reset)?;
    self.present.detour_to(d3d9::present)?;

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

    Ok(())
  }

  pub unsafe fn unhook_all(&self) -> HookResult<()> {
    dopamine_sdk::disable_all_hooks()?;

    self.reset.remove()?;
    self.present.remove()?;

    self.override_view.remove()?;
    self.create_move.remove()?;
    self.do_post_screen_space_effects.remove()?;

    self.level_init_post_entity.remove()?;
    self.level_shutdown.remove()?;

    self.draw_model_execute.remove()?;

    self.is_cursor_visible.remove()?;
    self.lock_cursor.remove()?;

    self.calc_viewmodel_view.remove()?;

    SetWindowLongPtrW(self.window, GWLP_WNDPROC, mem::transmute::<WNDPROC, isize>(self.wnd_proc));

    Ok(())
  }
}
