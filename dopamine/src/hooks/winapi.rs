use crate::app::App;

use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::CallWindowProcW;

use imgui::Key;

pub unsafe extern "stdcall" fn wnd_proc(
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

      if imgui::is_key_down(Key::Home) {
        let _ = app
          .unload()
          .inspect_err(|err| log::error!("Failed to deinitialize and unload App instance: {err}"));
      }

      if imgui::is_key_down(Key::Insert) {
        app.menu.handle_toggle();
      }
    }
    unsafe { CallWindowProcW(app.hooks.wnd_proc, hwnd, msg, w_param, l_param) }
  })
}
