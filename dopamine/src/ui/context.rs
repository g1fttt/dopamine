// TODO: Full recode needed for DX9 and Win32 backends

use imgui::{Context, Io, Ui};
use imgui_dx9_renderer::Renderer;
use imgui_win32_support::Win32;

use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct3D9::IDirect3DDevice9;

use windows::core::Result as WindowsResult;

use std::cell::{Cell, RefCell};
use std::sync::OnceLock;

static mut IMGUI_CONTEXT: OnceLock<ImGuiContext> = OnceLock::new();

pub struct ImGuiContext {
  ctx: RefCell<Context>,
  renderer: RefCell<Renderer>,
  win32: RefCell<Win32>,
  ui: Cell<*mut Ui>,
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
    unsafe { IMGUI_CONTEXT.take() };
  }

  fn new(device: IDirect3DDevice9, hwnd: HWND) -> Self {
    let ctx = RefCell::new(Context::create());

    let (renderer, win32) = {
      let ctx_ref = &mut ctx.borrow_mut();

      ctx_ref.set_ini_filename(None);

      let renderer = unsafe { Renderer::new(ctx_ref, device).unwrap() };
      let win32 = Win32::new(ctx_ref, hwnd);

      (RefCell::new(renderer), RefCell::new(win32))
    };

    Self { ctx, renderer, win32, ui: Cell::new(std::ptr::null_mut()) }
  }
}

impl ImGuiContext {
  pub unsafe fn prepare_frame(&self) -> WindowsResult<()> {
    self.win32.borrow_mut().prepare_frame(&mut self.ctx.borrow_mut(), self.ui())
  }

  pub fn new_frame(&self) -> &'static mut Ui {
    self.ui.set(self.ctx.borrow_mut().new_frame() as *mut Ui);
    unsafe { self.ui().unwrap_unchecked() }
  }

  pub fn reset(&self, device: IDirect3DDevice9) {
    self.renderer.replace(unsafe { Renderer::new(&mut self.ctx.borrow_mut(), device) }.unwrap());
  }

  pub fn render(&self) -> WindowsResult<()> {
    self.renderer.borrow_mut().render(self.ctx.borrow_mut().render())
  }

  pub fn io_mut(&self) -> &'static mut Io {
    unsafe { (self.ctx.borrow_mut().io_mut() as *mut Io).as_mut_unchecked() }
  }

  #[inline]
  pub fn ui(&self) -> Option<&'static mut Ui> {
    unsafe { self.ui.get().as_mut() }
  }
}
