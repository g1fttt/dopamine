use crate::App;

use windows::core::HRESULT;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Direct3D9::{IDirect3DDevice9, D3DPRESENT_PARAMETERS};
use windows::Win32::Graphics::Gdi::RGNDATA;

// TODO: Own GUI library 🤪

pub(super) type ResetFn = extern "stdcall" fn(&IDirect3DDevice9, &D3DPRESENT_PARAMETERS) -> HRESULT;

pub(super) extern "stdcall" fn reset(
    device: &IDirect3DDevice9,
    params: &D3DPRESENT_PARAMETERS,
) -> HRESULT {
    App::with(move |app| (app.hooks.reset)(device, params))
}

pub(super) type PresentFn = extern "stdcall" fn(
    &IDirect3DDevice9,
    Option<&RECT>,
    Option<&RECT>,
    HWND,
    Option<&RGNDATA>,
) -> HRESULT;

pub(super) extern "stdcall" fn present(
    device: &IDirect3DDevice9,
    src: Option<&RECT>,
    dest: Option<&RECT>,
    window_override: HWND,
    dirty_region: Option<&RGNDATA>,
) -> HRESULT {
    App::with(move |app| (app.hooks.present)(device, src, dest, window_override, dirty_region))
}
