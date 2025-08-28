use crate::app::App;
use crate::ui::ImGuiContext;

use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::CallWindowProcW;

use dopamine_sdk::utils::Interfaces;
use imgui::Key;

pub unsafe extern "stdcall" fn wnd_proc(
  hwnd: HWND,
  msg: u32,
  w_param: WPARAM,
  l_param: LPARAM,
) -> LRESULT {
  App::with_mut(move |app| {
    if let Some((fore_ctx, _)) = ImGuiContext::get_mut() {
      if let Ok(code) = fore_ctx.handle_window_proc(hwnd, msg, w_param, l_param)
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
        app.menu.handle_toggle(Interfaces::get().input_system);
        app.menu.update_mouse_cursor();
      }
    }
    unsafe { CallWindowProcW(app.hooks.wnd_proc, hwnd, msg, w_param, l_param) }
  })
}
