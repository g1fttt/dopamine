use crate::features::visuals;
use crate::interfaces::Interfaces;
use crate::ui::ImGuiContext;
use crate::App;

use windows::core::HRESULT;

use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Direct3D9::*;
use windows::Win32::Graphics::Gdi::RGNDATA;

pub type ResetFn = extern "stdcall" fn(IDirect3DDevice9, &D3DPRESENT_PARAMETERS) -> HRESULT;

pub extern "stdcall" fn reset(device: IDirect3DDevice9, params: &D3DPRESENT_PARAMETERS) -> HRESULT {
  App::with(move |app| {
    let result = (app.hooks.reset)(device.clone(), params);

    if let Some(imgui_ctx) = ImGuiContext::get_mut() {
      imgui_ctx.reset(device.clone());
    }
    result
  })
}

pub type PresentFn = extern "stdcall" fn(
  IDirect3DDevice9,
  Option<&RECT>,
  Option<&RECT>,
  HWND,
  Option<&RGNDATA>,
) -> HRESULT;

pub extern "stdcall" fn present(
  device: IDirect3DDevice9,
  src: Option<&RECT>,
  dest: Option<&RECT>,
  window_override: HWND,
  dirty_region: Option<&RGNDATA>,
) -> HRESULT {
  App::with_mut(move |app| unsafe {
    let mut params = D3DDEVICE_CREATION_PARAMETERS::default();
    let _ = device.GetCreationParameters(&mut params);

    let imgui_ctx = ImGuiContext::get_mut_or_init(device.clone(), params.hFocusWindow);
    imgui_ctx.prepare_frame();

    // ImGui::NewFrame with Drop at the end of the block
    let ui = imgui_ctx.new_frame();

    let interfaces = Interfaces::get();
    let should_draw_visuals =
      interfaces.engine.is_in_game() && !interfaces.surface.is_cursor_visible();

    if should_draw_visuals {
      visuals::draw_sniper_crosshair(
        &app.config.visuals.no_scope_crosshair,
        app.local_player,
        ui.io(),
        ui.get_background_draw_list(),
      )
    }

    if app.menu.is_open() {
      app.menu.render(ui, &mut app.config);
    }

    if let Ok(state_block) = device.CreateStateBlock(D3DSBT_ALL) {
      let _ = state_block.Capture();

      // Fix menu not rendering without `net_graph` or `cl_showfps`
      let _ = device.SetRenderState(D3DRS_COLORWRITEENABLE, u32::MAX);

      // Fix broken ImGui menu colors with Source engine gamma correction
      let _ = device.SetRenderState(D3DRS_SRGBWRITEENABLE, 0);

      if device.BeginScene().is_ok() {
        imgui_ctx.render();
        let _ = device.EndScene();
      }
      let _ = state_block.Apply();
    }
    (app.hooks.present)(device.clone(), src, dest, window_override, dirty_region)
  })
}
