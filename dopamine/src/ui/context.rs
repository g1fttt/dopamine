use imgui::sys::{igGetCurrentContext, igGetIO, igSetCurrentContext};
use imgui::{Context, DrawData, Io, Ui};

use imgui_dx9_renderer::Renderer;
use imgui_win32_support::Win32;

use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct3D9::IDirect3DDevice9;

use windows::core::Result as WindowsResult;

use std::ptr;
use std::sync::OnceLock;

static mut FORE_IMGUI_CONTEXT: OnceLock<ImGuiContext> = OnceLock::new();
static mut BACK_IMGUI_CONTEXT: OnceLock<ImGuiContext> = OnceLock::new();

pub struct ImGuiContext {
  ctx: Context,
  raw_ctx: *mut imgui::sys::ImGuiContext,
  renderer: Renderer,
  win32: Win32,
  draw_data: *const DrawData,
  ui: *mut Ui,
}

impl<'s: 'static> ImGuiContext {
  pub fn get_mut_or_init(device: IDirect3DDevice9, hwnd: HWND) -> (&'s mut Self, &'s mut Self) {
    unsafe {
      (
        FORE_IMGUI_CONTEXT.get_mut_or_init(|| ImGuiContext::new(device.clone(), hwnd)),
        BACK_IMGUI_CONTEXT.get_mut_or_init(|| ImGuiContext::new(device, hwnd)),
      )
    }
  }

  #[inline]
  pub fn get_mut() -> Option<(&'s mut Self, &'s mut Self)> {
    unsafe { FORE_IMGUI_CONTEXT.get_mut().zip(BACK_IMGUI_CONTEXT.get_mut()) }
  }

  #[inline]
  pub unsafe fn destroy() {
    unsafe {
      FORE_IMGUI_CONTEXT.take();
      BACK_IMGUI_CONTEXT.take();
    }
  }

  fn new(device: IDirect3DDevice9, hwnd: HWND) -> Self {
    let mut ctx = Context::create();
    ctx.set_ini_filename(None);

    let renderer = unsafe { Renderer::new(&mut ctx, device).unwrap() };
    let win32 = Win32::new(&mut ctx, hwnd);

    let raw_ctx = unsafe {
      let r = igGetCurrentContext();
      igSetCurrentContext(ptr::null_mut());
      r
    };

    Self { ctx, raw_ctx, renderer, win32, draw_data: ptr::null_mut(), ui: ptr::null_mut() }
  }
}

impl ImGuiContext {
  pub fn new_frame(&mut self) -> &'static mut Ui {
    self.set_current();

    let option_ui = self.ui();
    self.win32.prepare_frame(&mut self.ctx, option_ui);

    self.ui = self.ctx.new_frame() as *mut Ui;
    unsafe { self.ui.as_mut_unchecked() }
  }

  pub fn reset(&mut self, device: IDirect3DDevice9) -> WindowsResult<()> {
    self.renderer = unsafe { Renderer::new(&mut self.ctx, device)? };

    Ok(())
  }

  pub fn reset_render_state(&mut self) -> WindowsResult<()> {
    unsafe { self.renderer.set_render_state(self.draw_data.as_ref_unchecked()) }
  }

  pub fn render(&mut self) -> WindowsResult<()> {
    let draw_data = self.ctx.render();

    self.draw_data = draw_data as *const DrawData;

    self.renderer.render(draw_data)?;

    Ok(())
  }

  #[inline]
  pub fn renderer_mut(&mut self) -> &mut Renderer {
    &mut self.renderer
  }

  #[inline]
  pub fn io_mut(&self) -> &'static mut Io {
    unsafe { &mut *(igGetIO() as *mut Io) }
  }

  #[inline]
  pub fn ui(&self) -> Option<&'static mut Ui> {
    unsafe { self.ui.as_mut() }
  }

  fn set_current(&mut self) {
    unsafe { igSetCurrentContext(self.raw_ctx) };
  }
}
