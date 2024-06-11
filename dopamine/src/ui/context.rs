use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct3D9::IDirect3DDevice9;

use imgui::{Context, Ui};
use imgui_dx9_renderer::Renderer;
use imgui_win32_support::Win32;

use std::sync::OnceLock;

// TODO: Deinitialization upon unloading
pub struct ImGuiContext {
  ctx: Context,
  renderer: Renderer,
  win32: Win32,
}

impl ImGuiContext {
  pub unsafe fn get_mut_or_init(
    device: Option<IDirect3DDevice9>,
    hwnd: Option<HWND>,
  ) -> &'static mut Self {
    static mut IMGUI_CONTEXT: OnceLock<ImGuiContext> = OnceLock::new();
    IMGUI_CONTEXT.get_mut_or_init(|| ImGuiContext::new(device.unwrap(), hwnd.unwrap()))
  }

  fn new(device: IDirect3DDevice9, hwnd: HWND) -> Self {
    let mut ctx = Context::create();
    ctx.set_ini_filename(None);

    let renderer = unsafe { Renderer::new(&mut ctx, device).unwrap_unchecked() };
    let win32 = Win32::new(&mut ctx, hwnd);

    Self {
      ctx,
      renderer,
      win32,
    }
  }
}

impl ImGuiContext {
  #[inline(always)]
  pub fn prepare_frame(&mut self) {
    let _ = unsafe { self.win32.prepare_frame(&mut self.ctx) };
  }

  #[inline(always)]
  pub fn new_frame(&mut self) -> &mut Ui {
    self.ctx.new_frame()
  }

  #[inline(always)]
  pub fn render(&mut self) {
    let _ = self.renderer.render(self.ctx.render());
  }
}

unsafe impl Send for ImGuiContext {}
unsafe impl Sync for ImGuiContext {}
