use imgui::{Context, Io, Ui};
use imgui_dx9_renderer::Renderer;
use imgui_win32_support::Win32;

use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct3D9::IDirect3DDevice9;

use std::sync::OnceLock;

static mut IMGUI_CONTEXT: OnceLock<ImGuiContext> = OnceLock::new();

pub struct ImGuiContext {
  ctx: Context,
  renderer: Renderer,
  win32: Win32,
  ui: *mut Ui,
}

impl ImGuiContext {
  pub fn get_mut_or_init(device: IDirect3DDevice9, hwnd: HWND) -> &'static mut Self {
    unsafe { IMGUI_CONTEXT.get_mut_or_init(|| ImGuiContext::new(device, hwnd)) }
  }

  #[inline]
  pub fn get_mut() -> Option<&'static mut Self> {
    unsafe { IMGUI_CONTEXT.get_mut() }
  }

  #[inline]
  pub unsafe fn destroy() {
    IMGUI_CONTEXT.take();
  }

  fn new(device: IDirect3DDevice9, hwnd: HWND) -> Self {
    let mut ctx = Context::create();
    ctx.set_ini_filename(None);

    let renderer = unsafe { Renderer::new(&mut ctx, device).unwrap() };
    let win32 = Win32::new(&mut ctx, hwnd);

    Self { ctx, renderer, win32, ui: std::ptr::null_mut() }
  }
}

impl ImGuiContext {
  #[inline]
  pub fn prepare_frame(&mut self) {
    let _ = unsafe { self.win32.prepare_frame(&mut self.ctx) };
  }

  pub fn new_frame(&mut self) -> &mut Ui {
    let ui = self.ctx.new_frame();
    self.ui = ui as *mut Ui;
    ui
  }

  pub fn reset(&mut self, device: IDirect3DDevice9) {
    self.renderer = unsafe { Renderer::new(&mut self.ctx, device) }.unwrap();
  }

  #[inline]
  pub fn render(&mut self) {
    let _ = self.renderer.render(self.ctx.render());
  }

  #[inline]
  pub fn io_mut(&mut self) -> &mut Io {
    self.ctx.io_mut()
  }

  #[inline]
  pub fn ui(&mut self) -> Option<&mut Ui> {
    unsafe { self.ui.as_mut() }
  }
}
