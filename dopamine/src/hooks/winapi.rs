use crate::app::App;

use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::VK_END;
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
    if msg == WM_KEYUP && wparam.0 == VK_END.0 as _ {
      app.unload().expect("Failed to unload application");
    }
    CallWindowProcW(app.hooks.wnd_proc, window, msg, wparam, lparam)
  })
}
