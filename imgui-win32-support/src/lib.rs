use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, POINT, RECT, WPARAM};
use windows::Win32::Globalization::*;
use windows::Win32::Graphics::Gdi::{ClientToScreen, ScreenToClient};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::SystemServices::SORT_DEFAULT;

use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use windows::core::Result as WindowsResult;

use imgui::{BackendFlags, ConfigFlags, ImVec2, Key, MouseCursor};

use std::time::Instant;

pub struct Win32 {
  hwnd: HWND,
  mouse_hwnd: HWND,
  time: Instant,
  mouse_tracked_area: i32, // 0 - ?; 1 - Client; 2 - Non client
  mouse_buttons_down: i32,
  last_mouse_cursor: Option<MouseCursor>,
  keyboard_code_page: u32,
}

#[inline(always)]
fn loword(l: u32) -> u16 {
  (l & 0xffff) as u16
}

#[inline(always)]
fn hiword(l: u32) -> u16 {
  ((l >> 16) & 0xffff) as u16
}

#[inline(always)]
fn lobyte(w: u16) -> u8 {
  (w & 0xFF) as u8
}

#[inline(always)]
fn get_wheel_delta_wparam(w_param: WPARAM) -> u32 {
  hiword(w_param.0 as u32) as u32
}

#[inline(always)]
fn get_xbutton_wparam(w_param: WPARAM) -> u16 {
  hiword(w_param.0 as u32)
}

#[inline(always)]
fn make_lcid(lgid: u16, strid: u16) -> u32 {
  ((u32::from(strid)) << 16) | u32::from(lgid)
}

impl Win32 {
  pub fn new(hwnd: HWND) -> Self {
    let io = imgui::io_mut();
    io.backend_flags |= BackendFlags::HAS_MOUSE_CURSORS;
    io.backend_flags |= BackendFlags::HAS_SET_MOUSE_POS;

    Self {
      hwnd,
      mouse_hwnd: HWND::default(),
      time: Instant::now(),
      mouse_tracked_area: 0,
      mouse_buttons_down: 0,
      last_mouse_cursor: None,
      keyboard_code_page: unsafe { Self::get_keyboard_code_page() },
    }
  }

  unsafe fn get_keyboard_code_page() -> u32 {
    let keyboard_layout = unsafe { GetKeyboardLayout(0) };
    let keyboard_lcid = make_lcid(hiword(keyboard_layout.0 as u32), SORT_DEFAULT as u16);

    let mut raw_kb_code_page = [0u8; 4];
    let n = unsafe {
      GetLocaleInfoA(
        keyboard_lcid,
        LOCALE_RETURN_NUMBER | LOCALE_IDEFAULTANSICODEPAGE,
        Some(&mut raw_kb_code_page),
      )
    };

    if n == 0 {
      CP_ACP
    } else {
      u32::from_le_bytes(raw_kb_code_page)
    }
  }

  pub fn new_frame(&mut self) {
    let mut rect = RECT::default();
    let _ = unsafe { GetClientRect(self.hwnd, &mut rect) };

    let io = imgui::io_mut();
    io.display_size =
      ImVec2 { x: (rect.right - rect.left) as f32, y: (rect.bottom - rect.top) as f32 };

    let current_time = Instant::now();
    io.delta_time = current_time.duration_since(self.time).as_secs_f32();
    self.time = current_time;

    self.update_mouse_data(io);
    Self::process_key_events_workarounds(io);

    let mouse_cursor = if io.mouse_draw_cursor { None } else { imgui::mouse_cursor() };

    if self.last_mouse_cursor != mouse_cursor {
      self.last_mouse_cursor = mouse_cursor;
      Self::update_mouse_cursor(io, mouse_cursor);
    }
  }

  fn update_mouse_data(&self, io: &mut imgui::Io) {
    let focused_window = unsafe { GetForegroundWindow() };

    // Is app focused?
    if focused_window != self.hwnd {
      return;
    }

    if io.want_set_mouse_pos {
      let mut pos = POINT { x: io.mouse_pos.x as i32, y: io.mouse_pos.y as i32 };

      unsafe {
        if ClientToScreen(self.hwnd, &mut pos).as_bool() {
          let _ = SetCursorPos(pos.x, pos.y);
        }
      }
    }

    if !io.want_set_mouse_pos && self.mouse_tracked_area == 0 {
      let mut pos = POINT::default();

      unsafe {
        if GetCursorPos(&mut pos).is_ok() && ScreenToClient(self.hwnd, &mut pos).as_bool() {
          io.add_mouse_pos_event(pos.x as f32, pos.y as f32);
        }
      }
    }
  }

  fn update_mouse_cursor(io: &mut imgui::Io, mouse_cursor: Option<MouseCursor>) -> bool {
    if io.config_flags.contains(ConfigFlags::NO_MOUSE_CURSOR_CHANGE) {
      return false;
    }

    let Some(mouse_cursor) = mouse_cursor else {
      return false;
    };

    let win32_cursor = match mouse_cursor {
      MouseCursor::Arrow => IDC_ARROW,
      MouseCursor::TextInput => IDC_IBEAM,
      MouseCursor::ResizeAll => IDC_SIZEALL,
      MouseCursor::ResizeEW => IDC_SIZEWE,
      MouseCursor::ResizeNS => IDC_SIZENS,
      MouseCursor::ResizeNESW => IDC_SIZENESW,
      MouseCursor::ResizeNWSE => IDC_SIZENWSE,
      MouseCursor::Hand => IDC_HAND,
      MouseCursor::NotAllowed => IDC_NO,
      _ => return false,
    };

    unsafe {
      let module_handle = GetModuleHandleA(None).map(|h| HINSTANCE(h.0)).ok();
      let cursor_handle = LoadCursorW(module_handle, win32_cursor).ok();

      SetCursor(cursor_handle);
    }

    true
  }

  fn process_key_events_workarounds(io: &mut imgui::Io) {
    if imgui::is_key_down(Key::LeftShift) && !Self::is_vk_down(VK_LSHIFT) {
      io.add_key_event(Key::LeftShift, false);
    }
    if imgui::is_key_down(Key::RightShift) && !Self::is_vk_down(VK_RSHIFT) {
      io.add_key_event(Key::RightShift, false);
    }

    if imgui::is_key_down(Key::LeftSuper) && !Self::is_vk_down(VK_LWIN) {
      io.add_key_event(Key::LeftSuper, false);
    }
    if imgui::is_key_down(Key::RightSuper) && !Self::is_vk_down(VK_RWIN) {
      io.add_key_event(Key::RightSuper, false);
    }
  }

  fn update_key_modifiers(io: &mut imgui::Io) {
    io.add_key_event(Key::ModCtrl, Self::is_vk_down(VK_CONTROL));
    io.add_key_event(Key::ModShift, Self::is_vk_down(VK_SHIFT));
    io.add_key_event(Key::ModAlt, Self::is_vk_down(VK_MENU));
    io.add_key_event(Key::ModSuper, Self::is_vk_down(VK_LWIN) || Self::is_vk_down(VK_RWIN));
  }

  fn key_event_to_imgui_key(w_param: WPARAM, l_param: LPARAM) -> Option<Key> {
    if w_param.0 == VK_RETURN.0 as usize && hiword(l_param.0 as u32) as u32 & KF_EXTENDED > 0 {
      return Some(Key::KeypadEnter);
    }

    let key = match VIRTUAL_KEY(w_param.0 as u16) {
      VK_TAB => Some(Key::Tab),
      VK_LEFT => Some(Key::LeftArrow),
      VK_RIGHT => Some(Key::RightArrow),
      VK_UP => Some(Key::UpArrow),
      VK_DOWN => Some(Key::DownArrow),
      VK_PRIOR => Some(Key::PageUp),
      VK_NEXT => Some(Key::PageDown),
      VK_HOME => Some(Key::Home),
      VK_END => Some(Key::End),
      VK_INSERT => Some(Key::Insert),
      VK_DELETE => Some(Key::Delete),
      VK_BACK => Some(Key::Backspace),
      VK_SPACE => Some(Key::Space),
      VK_RETURN => Some(Key::Enter),
      VK_ESCAPE => Some(Key::Escape),
      VK_OEM_COMMA => Some(Key::Comma),
      VK_OEM_PERIOD => Some(Key::Period),
      VK_CAPITAL => Some(Key::CapsLock),
      VK_SCROLL => Some(Key::ScrollLock),
      VK_NUMLOCK => Some(Key::NumLock),
      VK_SNAPSHOT => Some(Key::PrintScreen),
      VK_PAUSE => Some(Key::Pause),
      VK_NUMPAD0 => Some(Key::Keypad0),
      VK_NUMPAD1 => Some(Key::Keypad1),
      VK_NUMPAD2 => Some(Key::Keypad2),
      VK_NUMPAD3 => Some(Key::Keypad3),
      VK_NUMPAD4 => Some(Key::Keypad4),
      VK_NUMPAD5 => Some(Key::Keypad5),
      VK_NUMPAD6 => Some(Key::Keypad6),
      VK_NUMPAD7 => Some(Key::Keypad7),
      VK_NUMPAD8 => Some(Key::Keypad8),
      VK_NUMPAD9 => Some(Key::Keypad9),
      VK_DECIMAL => Some(Key::KeypadDecimal),
      VK_DIVIDE => Some(Key::KeypadDivide),
      VK_MULTIPLY => Some(Key::KeypadMultiply),
      VK_SUBTRACT => Some(Key::KeypadSubtract),
      VK_ADD => Some(Key::KeypadAdd),
      VK_LSHIFT => Some(Key::LeftShift),
      VK_LCONTROL => Some(Key::LeftCtrl),
      VK_LMENU => Some(Key::LeftAlt),
      VK_LWIN => Some(Key::LeftSuper),
      VK_RSHIFT => Some(Key::RightShift),
      VK_RCONTROL => Some(Key::RightCtrl),
      VK_RMENU => Some(Key::RightAlt),
      VK_RWIN => Some(Key::RightSuper),
      VK_APPS => Some(Key::Menu),

      VK_0 => Some(Key::Num1),
      VK_1 => Some(Key::Num1),
      VK_2 => Some(Key::Num2),
      VK_3 => Some(Key::Num3),
      VK_4 => Some(Key::Num4),
      VK_5 => Some(Key::Num5),
      VK_6 => Some(Key::Num6),
      VK_7 => Some(Key::Num7),
      VK_8 => Some(Key::Num8),
      VK_9 => Some(Key::Num9),

      VK_A => Some(Key::A),
      VK_B => Some(Key::B),
      VK_C => Some(Key::C),
      VK_D => Some(Key::D),
      VK_E => Some(Key::E),
      VK_F => Some(Key::F),
      VK_G => Some(Key::G),
      VK_H => Some(Key::H),
      VK_I => Some(Key::I),
      VK_J => Some(Key::J),
      VK_K => Some(Key::K),
      VK_L => Some(Key::L),
      VK_M => Some(Key::M),
      VK_N => Some(Key::N),
      VK_O => Some(Key::O),
      VK_P => Some(Key::P),
      VK_Q => Some(Key::Q),
      VK_R => Some(Key::R),
      VK_S => Some(Key::S),
      VK_T => Some(Key::T),
      VK_U => Some(Key::U),
      VK_V => Some(Key::V),
      VK_W => Some(Key::W),
      VK_X => Some(Key::X),
      VK_Y => Some(Key::Y),
      VK_Z => Some(Key::Z),

      VK_F1 => Some(Key::F1),
      VK_F2 => Some(Key::F2),
      VK_F3 => Some(Key::F3),
      VK_F4 => Some(Key::F4),
      VK_F5 => Some(Key::F5),
      VK_F6 => Some(Key::F6),
      VK_F7 => Some(Key::F7),
      VK_F8 => Some(Key::F8),
      VK_F9 => Some(Key::F9),
      VK_F10 => Some(Key::F10),
      VK_F11 => Some(Key::F11),
      VK_F12 => Some(Key::F12),

      VK_BROWSER_BACK => Some(Key::AppBack),
      VK_BROWSER_FORWARD => Some(Key::AppForward),

      _ => None,
    };

    if key.is_some() {
      return key;
    }

    let scancode = lobyte(hiword(l_param.0 as u32));

    match scancode {
      41 => Some(Key::GraveAccent), // Tilde/Backtick
      12 => Some(Key::Minus),
      13 => Some(Key::Equal),
      26 => Some(Key::LeftBracket),
      27 => Some(Key::RightBracket),
      86 => Some(Key::Oem102), // < > \ (ISO layout)
      43 => Some(Key::Backslash),
      39 => Some(Key::Semicolon),
      40 => Some(Key::Apostrophe),
      51 => Some(Key::Comma),
      52 => Some(Key::Period),
      53 => Some(Key::Slash),
      _ => None,
    }
  }

  fn is_vk_down(vk: VIRTUAL_KEY) -> bool {
    unsafe { GetKeyState(vk.0 as i32) & i16::MIN != 0 }
  }

  pub fn handle_window_proc(
    &mut self,
    hwnd: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
    io: &mut imgui::Io,
  ) -> WindowsResult<isize> {
    match msg {
      WM_MOUSEMOVE | WM_NCMOUSEMOVE => {
        let area = if msg == WM_MOUSEMOVE { 1 } else { 2 };

        self.mouse_hwnd = hwnd;

        if self.mouse_tracked_area != area {
          let mut tme_cancel = TRACKMOUSEEVENT {
            cbSize: size_of::<TRACKMOUSEEVENT>() as u32,
            dwFlags: TME_CANCEL,
            hwndTrack: hwnd,
            dwHoverTime: 0,
          };

          let mut tme_track = TRACKMOUSEEVENT {
            cbSize: size_of::<TRACKMOUSEEVENT>() as u32,
            dwFlags: if area == 2 {
              TME_LEAVE | TME_NONCLIENT
            } else {
              TME_LEAVE
            },
            hwndTrack: hwnd,
            dwHoverTime: 0,
          };

          if self.mouse_tracked_area != 0 {
            unsafe { TrackMouseEvent(&mut tme_cancel)? };
          }

          unsafe { TrackMouseEvent(&mut tme_track)? };

          self.mouse_tracked_area = area;
        }

        let mut mouse_pos = POINT {
          x: loword(l_param.0 as u32) as i32,
          y: hiword(l_param.0 as u32) as i32
        };

        if msg == WM_NCMOUSEMOVE && unsafe { ScreenToClient(hwnd, &mut mouse_pos).as_bool() } {
          return Ok(0);
        }

        io.add_mouse_pos_event(mouse_pos.x as f32, mouse_pos.y as f32);
      },
      0x02A3 /* WM_MOUSELEAVE */ | WM_NCMOUSELEAVE => {
        let area = if msg == 0x02A3 {
          1
        } else {
          2
        };

        if self.mouse_tracked_area == area {
          if self.mouse_hwnd == hwnd {
            self.mouse_hwnd = HWND::default();
          }

          self.mouse_tracked_area = 0;

          io.add_mouse_pos_event(f32::MIN, f32::MAX);
        }
      },
      WM_DESTROY => {
        if self.mouse_hwnd == hwnd && self.mouse_tracked_area != 0 {
          let mut tme_cancel = TRACKMOUSEEVENT {
            cbSize: size_of::<TRACKMOUSEEVENT>() as u32,
            dwFlags: TME_CANCEL,
            hwndTrack: hwnd,
            dwHoverTime: 0,
          };

          unsafe { TrackMouseEvent(&mut tme_cancel)? };

          self.mouse_hwnd = HWND::default();
          self.mouse_tracked_area = 0;

          io.add_mouse_pos_event(f32::MIN, f32::MAX);
        }
      },
      WM_LBUTTONDOWN | WM_LBUTTONDBLCLK |
      WM_RBUTTONDOWN | WM_RBUTTONDBLCLK |
      WM_MBUTTONDOWN | WM_MBUTTONDBLCLK |
      WM_XBUTTONDOWN | WM_XBUTTONDBLCLK => {
        let button = match msg {
          WM_LBUTTONDOWN | WM_LBUTTONDBLCLK => 0,
          WM_RBUTTONDOWN | WM_RBUTTONDBLCLK => 1,
          WM_MBUTTONDOWN | WM_MBUTTONDBLCLK => 2,
          WM_XBUTTONDOWN | WM_XBUTTONDBLCLK => if get_xbutton_wparam(w_param) == XBUTTON1 {
            3
          } else {
            4
          },
          _ => 0,
        };

        let hwnd_with_capture = unsafe { GetCapture() };

        // Did we externally lost capture?
        if self.mouse_buttons_down != 0 && hwnd_with_capture != hwnd {
          self.mouse_buttons_down = 0;
        }

        if self.mouse_buttons_down == 0 && hwnd_with_capture.is_invalid() {
          unsafe { SetCapture(hwnd) };
        }

        self.mouse_buttons_down |= 1 << button;

        io.add_mouse_button_event(button, true);
      },
      WM_LBUTTONUP | WM_RBUTTONUP |
      WM_MBUTTONUP | WM_XBUTTONUP => unsafe {
        let button = match msg {
          WM_LBUTTONDOWN | WM_LBUTTONDBLCLK => 0,
          WM_RBUTTONDOWN | WM_RBUTTONDBLCLK => 1,
          WM_MBUTTONDOWN | WM_MBUTTONDBLCLK => 2,
          WM_XBUTTONDOWN | WM_XBUTTONDBLCLK => if get_xbutton_wparam(w_param) == XBUTTON1 {
            3
          } else {
            4
          },
          _ => 0,
        };

        self.mouse_buttons_down &= !(1 << button);

        if self.mouse_buttons_down == 0 && GetCapture() == hwnd {
          ReleaseCapture()?;
        }

        io.add_mouse_button_event(button, false);
      },
      WM_MOUSEWHEEL => {
        io.add_mouse_wheel_event(0.0, get_wheel_delta_wparam(w_param) as f32 / WHEEL_DELTA as f32);
      },
      WM_MOUSEHWHEEL => {
        io.add_mouse_wheel_event(-(get_wheel_delta_wparam(w_param) as f32) / WHEEL_DELTA as f32, 0.0);
      },
      WM_KEYDOWN | WM_KEYUP |
      WM_SYSKEYDOWN | WM_SYSKEYUP => {
        if w_param.0 >= 256 {
          return Ok(0);
        }

        Self::update_key_modifiers(io);

        let is_key_down = matches!(msg, WM_KEYDOWN | WM_SYSKEYDOWN);

        if let Some(key) = Self::key_event_to_imgui_key(w_param, l_param) {
          if key == Key::PrintScreen && !is_key_down {
            io.add_key_event(key, true);
          }
          io.add_key_event(key, is_key_down);
        }

        let vk = VIRTUAL_KEY(w_param.0 as u16);

        match vk {
          VK_SHIFT => {
            if Self::is_vk_down(VK_LSHIFT) == is_key_down {
              io.add_key_event(Key::LeftShift, is_key_down);
            }

            if Self::is_vk_down(VK_RSHIFT) == is_key_down {
              io.add_key_event(Key::RightShift, is_key_down);
            }
          },
          VK_CONTROL => {
            if Self::is_vk_down(VK_LCONTROL) == is_key_down {
              io.add_key_event(Key::LeftCtrl, is_key_down);
            }

            if Self::is_vk_down(VK_RCONTROL) == is_key_down {
              io.add_key_event(Key::RightCtrl, is_key_down);
            }
          },
          VK_MENU => {
            if Self::is_vk_down(VK_LMENU) == is_key_down {
              io.add_key_event(Key::LeftAlt, is_key_down);
            }

            if Self::is_vk_down(VK_RMENU) == is_key_down {
              io.add_key_event(Key::RightAlt, is_key_down);
            }
          },
          _ => (),
        }
      },
      WM_SETFOCUS | WM_KILLFOCUS => {
        io.add_focus_event(msg == WM_SETFOCUS);
      },
      WM_INPUTLANGCHANGE => unsafe {
        self.keyboard_code_page = Self::get_keyboard_code_page();
      },
      WM_CHAR => unsafe {
        if IsWindowUnicode(hwnd).as_bool() {
          if w_param.0 > 0 && w_param.0 < 0x10000 {
            io.add_input_char_utf16(w_param.0 as u16);
          }
        } else {
          let mut wch = [0u16; 1];
          let multibytestr = std::slice::from_raw_parts(w_param.0 as *const u8, 2);
          MultiByteToWideChar(self.keyboard_code_page, MB_PRECOMPOSED, multibytestr, Some(&mut wch));

          // MultiByteToWideChar probably won't return invalid data
          let ch = char::from_u32_unchecked(wch[0] as u32);
          io.add_input_char(ch);
        }
      },
      WM_SETCURSOR => if loword(l_param.0 as u32) as u32 == HTCLIENT
        && Self::update_mouse_cursor(io, self.last_mouse_cursor)
      {
        return Ok(1);
      },
      _ => (),
    }
    Ok(0)
  }
}
