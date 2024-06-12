use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct3D9::IDirect3DDevice9;

use imgui::{Context, Io, Ui};
use imgui_dx9_renderer::Renderer;
use imgui_win32_support::Win32;

use std::cell::UnsafeCell;
use std::mem;
use std::sync::OnceLock;

static mut IMGUI_CONTEXT: OnceLock<ImGuiContext> = OnceLock::new();

// TODO: Deinitialization upon unloading
pub struct ImGuiContext<'a> {
  ctx: Context,
  renderer: Renderer,
  win32: Win32,
  ui: Option<&'a mut UnsafeCell<Ui>>,
}

impl ImGuiContext<'_> {
  pub fn get_mut_or_init(device: IDirect3DDevice9, hwnd: HWND) -> &'static mut Self {
    unsafe { IMGUI_CONTEXT.get_mut_or_init(|| ImGuiContext::new(device, hwnd)) }
  }

  pub fn get_mut() -> Option<&'static mut Self> {
    unsafe { IMGUI_CONTEXT.get_mut() }
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
      ui: None,
    }
  }
}

impl ImGuiContext<'_> {
  #[inline(always)]
  pub fn prepare_frame(&mut self) {
    let _ = unsafe { self.win32.prepare_frame(&mut self.ctx) };
  }

  pub fn new_frame(&mut self) -> &mut Ui {
    let ui = self.ctx.new_frame();
    self.ui.replace(unsafe { mem::transmute_copy(&ui) });
    ui
  }

  #[inline(always)]
  pub fn render(&mut self) {
    let _ = self.renderer.render(self.ctx.render());
  }

  #[inline(always)]
  pub fn io_mut(&mut self) -> &mut Io {
    self.ctx.io_mut()
  }

  #[inline(always)]
  pub fn ui(&mut self) -> Option<&mut Ui> {
    self.ui.as_mut().map(|ui_cell| ui_cell.get_mut())
  }
}

unsafe impl Send for ImGuiContext<'_> {}
unsafe impl Sync for ImGuiContext<'_> {}
