mod client;
mod client_mode;
mod d3d9;
mod model_render;
mod surface;
mod viewmodel;
mod winapi;

use d3d9::{PresentFn, ResetFn};

use dopamine_sdk::interfaces::*;
use dopamine_sdk::math::{Angles, Vector3D};
use dopamine_sdk::utils::Patterns;
use dopamine_sdk::{pcstr, Hook, HookResult, TrampolineHook, VmtHook};

use dopamine_sdk::client::{Client, ClientMode};
use dopamine_sdk::engine::{ModelRender, ModelRenderInfo};
use dopamine_sdk::render_view::ViewSetup;
use dopamine_sdk::surface::Surface;
use dopamine_sdk::{Entity, UserCommand};

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::*;

use std::ffi::c_void;
use std::mem;

pub struct Hooks {
  window: HWND,
  pub(self) wnd_proc: WNDPROC,

  pub(self) reset: TrampolineHook<ResetFn>,
  pub(self) present: TrampolineHook<PresentFn>,

  pub(self) override_view: VmtHook<extern "C" fn(&ClientMode, &mut ViewSetup)>,
  pub(self) create_move: VmtHook<extern "C" fn(&ClientMode, f32, &mut UserCommand) -> bool>,
  pub(self) should_draw_crosshair: VmtHook<extern "C" fn(&ClientMode) -> bool>,
  pub(self) do_post_screen_space_effects: VmtHook<extern "C" fn(&ClientMode, &ViewSetup) -> bool>,

  pub(self) level_init_post_entity: VmtHook<extern "C" fn(&Client)>,
  pub(self) level_shutdown: VmtHook<extern "C" fn(&Client)>,

  pub(self) draw_model_execute:
    VmtHook<extern "C" fn(&ModelRender, *mut c_void, &ModelRenderInfo, *mut c_void)>,

  pub(self) is_cursor_visible: VmtHook<extern "C" fn(&Surface) -> bool>,
  pub(self) lock_cursor: VmtHook<extern "C" fn(&Surface)>,

  pub(self) calc_viewmodel_view:
    TrampolineHook<extern "C" fn(&Entity, &Entity, &Vector3D, &Angles)>,
}

impl Hooks {
  pub unsafe fn create() -> Self {
    unsafe {
      let patterns = Patterns::get();

      let window = FindWindowA(pcstr!("Valve001"), pcstr!())
        .inspect_err(|err| log::error!("Failed to find game window: {err}"))
        .unwrap();

      Self {
        window,
        wnd_proc: None,

        reset: TrampolineHook::new(patterns.d3d9_reset),
        present: TrampolineHook::new(patterns.d3d9_present),

        override_view: VmtHook::new(client_mode(), 16),
        create_move: VmtHook::new(client_mode(), 21),
        should_draw_crosshair: VmtHook::new(client_mode(), 25),
        do_post_screen_space_effects: VmtHook::new(client_mode(), 39),

        level_init_post_entity: VmtHook::new(client(), 6),
        level_shutdown: VmtHook::new(client(), 7),

        draw_model_execute: VmtHook::new(model_render(), 19),

        is_cursor_visible: VmtHook::new(surface(), 53),
        lock_cursor: VmtHook::new(surface(), 62),

        calc_viewmodel_view: TrampolineHook::new(patterns.calc_viewmodel_view),
      }
    }
  }

  pub unsafe fn hook_all(&mut self) -> HookResult<()> {
    unsafe {
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
      self.should_draw_crosshair.detour_to(client_mode::should_draw_crosshair)?;
      self.do_post_screen_space_effects.detour_to(client_mode::do_post_screen_space_effects)?;

      self.level_init_post_entity.detour_to(client::level_init_post_entity)?;
      self.level_shutdown.detour_to(client::level_shutdown)?;

      self.draw_model_execute.detour_to(model_render::draw_model_execute)?;

      self.is_cursor_visible.detour_to(surface::is_cursor_visible)?;
      self.lock_cursor.detour_to(surface::lock_cursor)?;

      self.calc_viewmodel_view.detour_to(viewmodel::calc_viewmodel_view)?;

      dopamine_sdk::enable_all_hooks()?;
    }
    Ok(())
  }

  pub unsafe fn unhook_all(&self) -> HookResult<()> {
    unsafe {
      dopamine_sdk::disable_all_hooks()?;

      self.reset.remove()?;
      self.present.remove()?;

      self.override_view.remove()?;
      self.create_move.remove()?;
      self.should_draw_crosshair.remove()?;
      self.do_post_screen_space_effects.remove()?;

      self.level_init_post_entity.remove()?;
      self.level_shutdown.remove()?;

      self.draw_model_execute.remove()?;

      self.is_cursor_visible.remove()?;
      self.lock_cursor.remove()?;

      self.calc_viewmodel_view.remove()?;

      SetWindowLongPtrW(self.window, GWLP_WNDPROC, mem::transmute::<WNDPROC, isize>(self.wnd_proc));
    }
    Ok(())
  }
}
