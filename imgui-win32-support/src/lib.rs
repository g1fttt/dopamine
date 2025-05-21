// https://github.com/super-continent

use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{ClientToScreen, ScreenToClient};
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use windows::core::Result as WindowsResult;

use std::ffi::c_void;
use std::time::Instant;

use imgui::Key;

pub type WindowProc = unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT;

#[derive(PartialEq)]
pub enum ProcResponse {
  NoAction,
  ActionTaken,
}

pub struct Win32 {
  hwnd: HWND,
  time: Instant,
  last_cursor: Option<imgui::MouseCursor>,
}

#[inline]
fn loword(l: u32) -> u16 {
  (l & 0xffff) as u16
}

#[inline]
fn hiword(l: u32) -> u16 {
  ((l >> 16) & 0xffff) as u16
}

fn get_wheel_delta_wparam(w_param: u32) -> u32 {
  hiword(w_param) as u32
}

impl Win32 {
  pub fn new(imgui: &mut imgui::Context, hwnd: HWND) -> Self {
    let time = Instant::now();
    let io = imgui.io_mut();

    io.backend_flags.insert(imgui::BackendFlags::HAS_MOUSE_CURSORS);
    io.backend_flags.insert(imgui::BackendFlags::HAS_SET_MOUSE_POS);

    io.key_map[Key::Tab as usize] = VK_TAB.0 as u32;
    io.key_map[Key::LeftArrow as usize] = VK_LEFT.0 as u32;
    io.key_map[Key::RightArrow as usize] = VK_RIGHT.0 as u32;
    io.key_map[Key::UpArrow as usize] = VK_UP.0 as u32;
    io.key_map[Key::DownArrow as usize] = VK_DOWN.0 as u32;
    io.key_map[Key::PageUp as usize] = VK_PRIOR.0 as u32;
    io.key_map[Key::PageDown as usize] = VK_NEXT.0 as u32;
    io.key_map[Key::Home as usize] = VK_HOME.0 as u32;
    io.key_map[Key::End as usize] = VK_END.0 as u32;
    io.key_map[Key::Insert as usize] = VK_INSERT.0 as u32;
    io.key_map[Key::Delete as usize] = VK_DELETE.0 as u32;
    io.key_map[Key::Backspace as usize] = VK_BACK.0 as u32;
    io.key_map[Key::Space as usize] = VK_SPACE.0 as u32;
    io.key_map[Key::KeypadEnter as usize] = VK_RETURN.0 as u32;
    io.key_map[Key::Escape as usize] = VK_ESCAPE.0 as u32;
    io.key_map[Key::KeypadEnter as usize] = VK_RETURN.0 as u32;
    io.key_map[Key::A as usize] = 'A' as u32;
    io.key_map[Key::C as usize] = 'C' as u32;
    io.key_map[Key::V as usize] = 'V' as u32;
    io.key_map[Key::X as usize] = 'X' as u32;
    io.key_map[Key::Y as usize] = 'Y' as u32;
    io.key_map[Key::Z as usize] = 'Z' as u32;

    imgui.set_platform_name(format!("imgui-win32 {}", env!("CARGO_PKG_VERSION")));

    Self { hwnd, time, last_cursor: None }
  }

  pub fn prepare_frame(
    &mut self,
    context: &mut imgui::Context,
    ui: Option<&mut imgui::Ui>,
  ) -> WindowsResult<()> {
    let current_cursor = context.mouse_cursor();

    let io = context.io_mut();

    // Set up display size every frame to handle resizing
    let mut rect = RECT::default();
    unsafe { GetClientRect(self.hwnd, &mut rect)? };

    let width = (rect.right - rect.left) as f32;
    let height = (rect.bottom - rect.top) as f32;
    io.display_size = [width, height];

    // Perform time step
    let current_time = Instant::now();

    let last_time = self.time;
    self.time = current_time;

    io.delta_time = current_time.duration_since(last_time).as_secs_f32();

    // Read key states
    unsafe {
      io.key_ctrl = (GetKeyState(VK_CONTROL.0 as i32) as u16 & 0x8000) != 0;
      io.key_shift = (GetKeyState(VK_SHIFT.0 as i32) as u16 & 0x8000) != 0;
      io.key_alt = (GetKeyState(VK_MENU.0 as i32) as u16 & 0x8000) != 0;
    }

    io.key_super = false;

    self.update_cursor_pos(io);

    if self.last_cursor != current_cursor {
      self.last_cursor = current_cursor;

      update_cursor(ui, io);
    }
    Ok(())
  }

  fn update_cursor_pos(&self, io: &mut imgui::Io) {
    if io.want_set_mouse_pos {
      let x = io.mouse_pos[0] as i32;
      let y = io.mouse_pos[1] as i32;

      let mut pos = POINT { x, y };

      unsafe {
        if ClientToScreen(self.hwnd, &mut pos).as_bool() {
          let _ = SetCursorPos(pos.x, pos.y);
        }
      }
    }

    io.mouse_pos = [-f32::MAX, -f32::MAX];

    let mut pos = POINT::default();

    unsafe {
      let foreground_hwnd = GetForegroundWindow();

      if (self.hwnd == foreground_hwnd || IsChild(foreground_hwnd, self.hwnd).as_bool())
        && GetCursorPos(&mut pos).is_ok()
        && ScreenToClient(self.hwnd, &mut pos).as_bool()
      {
        io.mouse_pos = [pos.x as f32, pos.y as f32];
      }
    }
  }
}

fn update_cursor(ui: Option<&mut imgui::Ui>, io: &mut imgui::Io) -> bool {
  if io.config_flags.contains(imgui::ConfigFlags::NO_MOUSE_CURSOR_CHANGE) {
    return false;
  };

  let win32_cursor = match ui.as_deref().and_then(imgui::Ui::mouse_cursor) {
    Some(cursor) => match cursor {
      imgui::MouseCursor::Arrow => IDC_ARROW,
      imgui::MouseCursor::TextInput => IDC_IBEAM,
      imgui::MouseCursor::ResizeAll => IDC_SIZEALL,
      imgui::MouseCursor::ResizeEW => IDC_SIZEWE,
      imgui::MouseCursor::ResizeNS => IDC_SIZENS,
      imgui::MouseCursor::ResizeNESW => IDC_SIZENESW,
      imgui::MouseCursor::ResizeNWSE => IDC_SIZENWSE,
      imgui::MouseCursor::Hand => IDC_HAND,
      imgui::MouseCursor::NotAllowed => IDC_NO,
    }
    .as_ptr()
    .cast_mut() as *mut c_void,
    None => std::ptr::null_mut(),
  };

  unsafe { SetCursor(HCURSOR(win32_cursor)) };

  true
}
pub fn imgui_win32_window_proc(
  window: HWND,
  msg: u32,
  w_param: WPARAM,
  l_param: LPARAM,
  ui: Option<&mut imgui::Ui>,
  io: &mut imgui::Io,
) -> WindowsResult<ProcResponse> {
  let w_param = w_param.0 as u32;

  let result = match msg {
    WM_LBUTTONDOWN | WM_LBUTTONDBLCLK | WM_RBUTTONDOWN | WM_RBUTTONDBLCLK | WM_MBUTTONDOWN
    | WM_MBUTTONDBLCLK => {
      let button = match msg {
        WM_LBUTTONDOWN | WM_LBUTTONDBLCLK => 0,
        WM_RBUTTONDOWN | WM_RBUTTONDBLCLK => 1,
        WM_MBUTTONDOWN | WM_MBUTTONDBLCLK => 2,
        WM_XBUTTONDOWN | WM_XBUTTONDBLCLK => 3,
        _ => 0,
      };

      unsafe {
        if !ui.as_deref().is_some_and(imgui::Ui::is_any_mouse_down) && GetCapture().is_invalid() {
          SetCapture(window);
        }
      }

      io.mouse_down[button] = true;

      ProcResponse::NoAction
    }

    WM_LBUTTONUP | WM_RBUTTONUP | WM_MBUTTONUP | WM_XBUTTONUP => {
      let button = match msg {
        WM_LBUTTONUP => 0,
        WM_RBUTTONUP => 1,
        WM_MBUTTONUP => 2,
        WM_XBUTTONUP => 3,
        _ => 0,
      };

      io.mouse_down[button] = false;

      unsafe {
        if !ui.as_deref().is_some_and(imgui::Ui::is_any_mouse_down) && GetCapture() == window {
          let _ = ReleaseCapture();
        }
      }
      ProcResponse::NoAction
    }
    WM_MOUSEWHEEL => {
      io.mouse_wheel += (get_wheel_delta_wparam(w_param) / WHEEL_DELTA) as f32;
      ProcResponse::NoAction
    }
    WM_MOUSEHWHEEL => {
      io.mouse_wheel_h += (get_wheel_delta_wparam(w_param) / WHEEL_DELTA) as f32;
      ProcResponse::NoAction
    }
    WM_KEYDOWN | WM_SYSKEYDOWN => {
      if w_param < 256 {
        io.keys_down[w_param as usize] = true;
      }
      ProcResponse::NoAction
    }
    WM_KEYUP | WM_SYSKEYUP => {
      if w_param < 256 {
        io.keys_down[w_param as usize] = false;
      }
      ProcResponse::NoAction
    }
    WM_CHAR => {
      if w_param > 0 && w_param < 0x10000 {
        io.add_input_character(unsafe { char::from_u32_unchecked(w_param) });
      }
      ProcResponse::NoAction
    }
    WM_SETCURSOR => {
      if loword(l_param.0 as u32) as u32 == HTCLIENT && update_cursor(ui, io) {
        ProcResponse::ActionTaken
      } else {
        ProcResponse::NoAction
      }
    }
    WM_DEVICECHANGE => ProcResponse::NoAction,
    _ => ProcResponse::NoAction,
  };
  Ok(result)
}
