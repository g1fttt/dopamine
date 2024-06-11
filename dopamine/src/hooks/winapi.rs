use crate::interfaces::Interfaces;
use crate::ui::ImGuiContext;
use crate::App;

use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_END, VK_INSERT};
use windows::Win32::UI::WindowsAndMessaging::{CallWindowProcW, WM_KEYUP};

use imgui_win32_support::imgui_win32_window_proc;

pub unsafe extern "stdcall" fn wnd_proc(
  window: HWND,
  msg: u32,
  wparam: WPARAM,
  lparam: LPARAM,
) -> LRESULT {
  let _ = imgui_win32_window_proc(window, msg, wparam, lparam);

  App::with_mut(move |app| {
    // TODO: Think of how to use `ui.is_key_pressed` here
    if msg == WM_KEYUP {
      if wparam.0 == VK_END.0 as usize {
        app.unload().expect("Failed to unload application");
      }

      if wparam.0 == VK_INSERT.0 as usize {
        app.menu.handle_toggle(Interfaces::get().input_system);

        if let Some(imgui_ctx) = ImGuiContext::get_mut() {
          app.menu.update_mouse_cursor(imgui_ctx.io_mut());
        }
      }
    }
    CallWindowProcW(app.hooks.wnd_proc, window, msg, wparam, lparam)
  })
}
