use crate::app::App;

use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{CallWindowProcW, VK_END, WM_KEYUP};

pub unsafe extern "stdcall" fn wnd_proc(
    window: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    App::with(move |app| {
        if msg == WM_KEYUP && wparam == VK_END as _ {
            app.unload();
        }
        CallWindowProcW(app.hooks.wnd_proc, window, msg, wparam, lparam)
    })
    .unwrap_or_default()
}
