use crate::app::App;
use crate::features::visuals;
use crate::ui::{BlurEffect, Context as ImGuiContext};

use dopamine_sdk::utils::Interfaces;

use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Direct3D9::*;
use windows::Win32::Graphics::Gdi::RGNDATA;

use windows::core::{HRESULT, Interface, Result as WindowsResult};

use std::ffi::c_void;
use std::ptr::NonNull;

pub type ResetFn = extern "stdcall" fn(NonNull<c_void>, &D3DPRESENT_PARAMETERS) -> HRESULT;

pub extern "stdcall" fn reset(this: NonNull<c_void>, params: &D3DPRESENT_PARAMETERS) -> HRESULT {
  App::with_mut(move |app| {
    let this_raw_ptr = this.as_ptr();
    let device = unsafe { IDirect3DDevice9::from_raw_borrowed(&this_raw_ptr).unwrap() };

    if let Some(blur_effect) = app.blur_effect.as_mut() {
      blur_effect.clear_textures();
    }

    let result = (app.hooks.reset.original)(this, params);

    if let Some((background, foreground)) =
      app.background_imgui_context.get_mut().zip(app.foreground_imgui_context.get_mut())
    {
      let _ = background.reset(device);
      let _ = foreground.reset(device);
    }
    result
  })
}

pub type PresentFn = extern "stdcall" fn(
  NonNull<c_void>,
  Option<&RECT>,
  Option<&RECT>,
  HWND,
  Option<&RGNDATA>,
) -> HRESULT;

pub extern "stdcall" fn present(
  this: NonNull<c_void>,
  src: Option<&RECT>,
  dest: Option<&RECT>,
  window_override: HWND,
  dirty_region: Option<&RGNDATA>,
) -> HRESULT {
  App::with_mut(move |app| {
    let this_raw_ptr = this.as_ptr();
    let device = unsafe { IDirect3DDevice9::from_raw_borrowed(&this_raw_ptr).unwrap() };

    if let Ok(state_block) = unsafe { device.CreateStateBlock(D3DSBT_ALL) } {
      let mut params = D3DDEVICE_CREATION_PARAMETERS::default();
      let _ = unsafe { device.GetCreationParameters(&mut params) };

      if app.blur_effect.is_none() {
        app.blur_effect = Some(BlurEffect::new(device));
      }

      // Fix menu doesn't render without `net_graph` or `cl_showfps`
      let _ = unsafe { device.SetRenderState(D3DRS_COLORWRITEENABLE, u32::MAX) };

      draw_background(app, device, params.hFocusWindow)
        .inspect_err(|err| log::error!("Failed to draw ImGui `background` context: {err}"))
        .unwrap();
      draw_foreground(app, device, params.hFocusWindow)
        .inspect_err(|err| log::error!("Failed to draw ImGui `foreground` context: {err}"))
        .unwrap();

      let _ = unsafe { state_block.Apply() };
    }
    (app.hooks.present.original)(this, src, dest, window_override, dirty_region)
  })
}

fn draw_background(app: &mut App, device: &IDirect3DDevice9, hwnd: HWND) -> WindowsResult<()> {
  let background = app.background_imgui_context.get_mut_or_init(|| ImGuiContext::new(device, hwnd));

  let frame = background.new_frame();

  app.menu.update_animation();

  let bg_draw_list = imgui::background_draw_list();

  if let Some(blur_effect) = app.blur_effect.as_mut()
    && app.config.blur_enabled
    && !app.menu.is_fully_closed()
  {
    blur_effect.render(bg_draw_list, app.menu.transparency())?;
  }

  let interfaces = Interfaces::get();
  let should_draw_visuals =
    interfaces.engine.is_in_game() && !interfaces.surface.is_cursor_visible();

  if should_draw_visuals {
    visuals::draw_better_crosshair(
      app.local_player,
      &app.config.visuals.better_crosshair,
      bg_draw_list,
    );
  }

  frame.end();

  background.render(device)
}

fn draw_foreground(app: &mut App, device: &IDirect3DDevice9, hwnd: HWND) -> WindowsResult<()> {
  let foreground = app.foreground_imgui_context.get_mut_or_init(|| ImGuiContext::new(device, hwnd));

  let frame = foreground.new_frame();

  // Fix broken ImGui menu colors with Source engine gamma correction
  unsafe { device.SetRenderState(D3DRS_SRGBWRITEENABLE, 0)? };

  if app.menu.is_open() {
    app.menu.render(&mut app.config);
  }

  frame.end();

  foreground.render(device)
}
