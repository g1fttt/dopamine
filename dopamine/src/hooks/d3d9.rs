use crate::App;
use crate::features::visuals;
use crate::ui::ImGuiContext;

use dopamine_sdk::utils::Interfaces;

use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Direct3D9::*;
use windows::Win32::Graphics::Gdi::RGNDATA;

use windows::core::{HRESULT, Result as WindowsResult};

use imgui::Ui;
use imgui_dx9_renderer::Renderer;

pub type ResetFn = extern "stdcall" fn(IDirect3DDevice9, &D3DPRESENT_PARAMETERS) -> HRESULT;

pub extern "stdcall" fn reset(device: IDirect3DDevice9, params: &D3DPRESENT_PARAMETERS) -> HRESULT {
  App::with_mut(move |app| {
    unsafe { app.blur_effect.clear_textures() };

    let result = (app.hooks.reset.original)(device.clone(), params);

    if let Some((fore_ctx, back_ctx)) = ImGuiContext::get_mut() {
      let _ = fore_ctx.reset(device.clone());
      let _ = back_ctx.reset(device.clone());
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
    if let Ok(state_block) = device.CreateStateBlock(D3DSBT_ALL) {
      let mut params = D3DDEVICE_CREATION_PARAMETERS::default();
      let _ = device.GetCreationParameters(&mut params);

      let (fore_ctx, back_ctx) = ImGuiContext::get_mut_or_init(device.clone(), params.hFocusWindow);

      // Fix menu doesn't render without `net_graph` or `cl_showfps`
      let _ = device.SetRenderState(D3DRS_COLORWRITEENABLE, u32::MAX);

      let _ = render_imgui(back_ctx, &device, |ui, renderer| {
        let interfaces = Interfaces::get();
        let should_draw_visuals =
          interfaces.engine.is_in_game() && !interfaces.surface.is_cursor_visible();

        let io = ui.io();

        app.menu.update_animation(io);

        if !app.menu.is_fully_closed() {
          app.blur_effect.render(
            &device,
            renderer,
            io,
            ui.get_background_draw_list(),
            app.menu.transparency(),
          )?;
        }

        if should_draw_visuals {
          visuals::draw_better_crosshair(
            app.capture_context(&app.config.visuals.better_crosshair),
            io,
            ui.get_background_draw_list(),
          )
        }

        Ok(())
      });

      let _ = render_imgui(fore_ctx, &device, |ui, _renderer| {
        // Fix broken ImGui menu colors with Source engine gamma correction
        device.SetRenderState(D3DRS_SRGBWRITEENABLE, 0)?;

        if app.menu.is_open() {
          app.menu.render(ui, &mut app.config);
        }
        Ok(())
      });

      let _ = state_block.Apply();
    }
    (app.hooks.present.original)(device.clone(), src, dest, window_override, dirty_region)
  })
}

fn render_imgui<F>(
  imgui_ctx: &mut ImGuiContext,
  device: &IDirect3DDevice9,
  mut f: F,
) -> WindowsResult<()>
where
  F: FnMut(&mut Ui, &mut Renderer) -> WindowsResult<()>,
{
  f(imgui_ctx.new_frame(), imgui_ctx.renderer_mut())?;

  unsafe {
    if device.BeginScene().is_ok() {
      imgui_ctx.render()?;
      device.EndScene()?;
    }
  }
  Ok(())
}
