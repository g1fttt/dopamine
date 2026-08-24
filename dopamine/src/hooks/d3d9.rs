use crate::app::App;
use crate::features::visuals;
use crate::ui::{BlurEffect, Context as ImGuiContext};

use windows::core::{HRESULT, Interface, Result as WindowsResult};

use dopamine_sdk::Hook;
use dopamine_sdk::interfaces::{engine, surface};

use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Direct3D9::*;
use windows::Win32::Graphics::Gdi::RGNDATA;

use std::ffi::c_void;
use std::ptr::NonNull;

pub type ResetFn = extern "C" fn(NonNull<c_void>, &D3DPRESENT_PARAMETERS) -> HRESULT;

pub extern "C" fn reset(this: NonNull<c_void>, params: &D3DPRESENT_PARAMETERS) -> HRESULT {
  App::with_mut(move |app| {
    let this_raw_ptr = this.as_ptr();
    let device = unsafe { IDirect3DDevice9::from_raw_borrowed(&this_raw_ptr).unwrap() };

    if let Some(blur_effect) = app.blur_effect.get_mut() {
      blur_effect.clear_textures();
    }

    let result = (app.hooks.reset.original())(this, params);

    if let Some(context) = app.imgui_context.get_mut()
      && let Err(err) = context.reset(device)
    {
      log::error!("Failed to reset ImGui context: {err}");
    }
    result
  })
}

pub type PresentFn =
  extern "C" fn(NonNull<c_void>, Option<&RECT>, Option<&RECT>, HWND, Option<&RGNDATA>) -> HRESULT;

pub extern "C" fn present(
  this: NonNull<c_void>,
  src: Option<&RECT>,
  dest: Option<&RECT>,
  window_override: HWND,
  dirty_region: Option<&RGNDATA>,
) -> HRESULT {
  App::with_mut(move |app| unsafe {
    let this_raw_ptr = this.as_ptr();
    let device = IDirect3DDevice9::from_raw_borrowed(&this_raw_ptr).unwrap();

    let mut params = D3DDEVICE_CREATION_PARAMETERS::default();
    let _ = device.GetCreationParameters(&mut params);

    if let Err(err) = draw_imgui_context(app, device, params.hFocusWindow) {
      log::error!("Failed to draw ImGui context: {err}");
    }
    (app.hooks.present.original())(this, src, dest, window_override, dirty_region)
  })
}

fn draw_imgui_context(app: &mut App, device: &IDirect3DDevice9, hwnd: HWND) -> WindowsResult<()> {
  let context = app.imgui_context.get_mut_or_init(|| ImGuiContext::new(device, hwnd));

  unsafe {
    // Fix menu doesn't render without `net_graph` or `cl_showfps`
    device.SetRenderState(D3DRS_COLORWRITEENABLE, u32::MAX)?;

    // Fix broken ImGui menu colors with Source engine gamma correction
    device.SetRenderState(D3DRS_SRGBWRITEENABLE, 0)?;
  };

  context.draw_with_frame(device, || {
    app.menu.update_animation();

    let bg_draw_list = imgui::background_draw_list();

    let blur_effect = app.blur_effect.get_mut_or_init(|| BlurEffect::new(device));
    let should_draw_blur_effect = app.config.blur_enabled && !app.menu.is_fully_closed();

    if should_draw_blur_effect {
      blur_effect.draw(bg_draw_list, app.menu.transparency())?;
    }

    let should_draw_visuals = engine().is_in_game() && !surface().is_cursor_visible();

    if should_draw_visuals {
      visuals::draw_better_crosshair(&app.config.visuals.better_crosshair, bg_draw_list);
    }

    if app.menu.is_open() {
      app.menu.draw(&mut app.config);
    }
    Ok(())
  })?;

  unsafe {
    device.SetRenderState(
      D3DRS_COLORWRITEENABLE,
      0xF, // Default value according to winapi docs
    )?;
    device.SetRenderState(D3DRS_SRGBWRITEENABLE, 1)
  }
}
