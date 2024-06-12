use crate::interfaces::Interfaces;
use crate::ui::ImGuiContext;
use crate::App;

use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::CallWindowProcW;

use imgui::Key;
use imgui_win32_support::{imgui_win32_window_proc, ProcResponse};

pub unsafe extern "stdcall" fn wnd_proc(
  window: HWND,
  msg: u32,
  wparam: WPARAM,
  lparam: LPARAM,
) -> LRESULT {
  if let Ok(resp) = imgui_win32_window_proc(window, msg, wparam, lparam)
    && resp == ProcResponse::ActionTaken
  {
    return LRESULT(1);
  }

  App::with_mut(move |app| {
    if let Some(imgui_ctx) = ImGuiContext::get_mut()
      && let Some(ui) = imgui_ctx.ui()
    {
      if ui.is_key_down(Key::End) {
        app.unload().expect("Failed to unload application");
      }

      if ui.is_key_down(Key::Insert) {
        app.menu.handle_toggle(Interfaces::get().input_system);
        app.menu.update_mouse_cursor(imgui_ctx.io_mut());
      }
    }
    CallWindowProcW(app.hooks.wnd_proc, window, msg, wparam, lparam)
  })
}
