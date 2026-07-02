use crate::app::App;

use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::CallWindowProcW;

pub unsafe extern "system" fn wnd_proc(
  hwnd: HWND,
  msg: u32,
  w_param: WPARAM,
  l_param: LPARAM,
) -> LRESULT {
  App::with_mut(move |app| {
    if let Some(foreground) = app.foreground_imgui_context.get_mut() {
      if let Ok(code) = foreground.handle_window_proc(hwnd, msg, w_param, l_param)
        && code > 0
      {
        return LRESULT(code);
      }

      if is_key_down(VK_HOME) {
        let _ = app
          .unload()
          .inspect_err(|err| log::error!("Failed to deinitialize and unload App instance: {err}"));
      } else if is_key_down(VK_INSERT) {
        app.menu.handle_toggle();
      }
    }
    unsafe { CallWindowProcW(app.hooks.wnd_proc, hwnd, msg, w_param, l_param) }
  })
}

fn is_key_down(key: VIRTUAL_KEY) -> bool {
  unsafe { GetAsyncKeyState(key.0 as i32) & i16::MAX != 0 }
}
