use imgui_dx9_renderer::Renderer;
use imgui_win32_support::Win32;

use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::Graphics::Direct3D9::IDirect3DDevice9;

use windows::core::Result as WindowsResult;

pub struct Context {
  ctx: imgui::Context,
  renderer: Renderer,
  win32: Win32,
}

impl Context {
  pub fn new(device: &IDirect3DDevice9, hwnd: HWND) -> Self {
    let ctx = imgui::Context::new();
    ctx.set_current();

    let renderer = Renderer::new(device)
      .inspect_err(|err| log::error!("Failed to create ImGui DX9 renderer: {err}"))
      .unwrap();

    let win32 = Win32::new(hwnd);

    imgui::style_colors_dark();

    let io = imgui::io_mut();
    io.set_ini_filename(None);
    io.set_log_filename(None);

    io.config_flags |= imgui::ConfigFlags::NO_MOUSE_CURSOR_CHANGE;

    io.font_atlas().add_font_default();

    Self { ctx, renderer, win32 }
  }
}

impl Context {
  pub fn new_frame(&mut self) -> imgui::Frame {
    self.ctx.set_current();
    self.win32.new_frame();
    self.ctx.new_frame()
  }

  pub fn handle_window_proc(
    &mut self,
    hwnd: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
  ) -> WindowsResult<isize> {
    self.ctx.set_current();
    self.win32.handle_window_proc(hwnd, msg, w_param, l_param, imgui::io_mut())
  }

  #[inline]
  pub fn reset(&mut self, device: &IDirect3DDevice9) -> WindowsResult<()> {
    self.renderer = Renderer::new(device)?;

    Ok(())
  }

  pub fn render(&mut self, device: &IDirect3DDevice9) -> WindowsResult<()> {
    self.ctx.render();

    unsafe {
      if device.BeginScene().is_ok() {
        self.renderer.render(imgui::draw_data())?;
      }
      device.EndScene()
    }
  }
}
