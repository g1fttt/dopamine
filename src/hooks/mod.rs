mod client;
mod client_mode;
mod winapi;

use crate::interfaces::Interfaces;
use crate::utils::VMTHook;

use ::winapi::shared::windef::HWND;
use ::winapi::um::winuser::{SetWindowLongPtrW, GWLP_WNDPROC, WNDPROC};

use std::mem;

pub struct Hooks {
    window: HWND,
    pub create_move: VMTHook,
    pub level_init_post_entity: VMTHook,
    pub level_shutdown: VMTHook,
    pub wnd_proc: WNDPROC,
}

impl Hooks {
    pub fn create(interfaces: &Interfaces, window: HWND) -> Self {
        let wnd_proc: WNDPROC = unsafe {
            #[allow(clippy::fn_to_numeric_cast)]
            mem::transmute(SetWindowLongPtrW(
                window,
                GWLP_WNDPROC,
                winapi::wnd_proc as _,
            ))
        };

        Self {
            window,
            create_move: VMTHook::from_base(interfaces.client_mode),
            level_init_post_entity: VMTHook::from_base(interfaces.client),
            level_shutdown: VMTHook::from_base(interfaces.client),
            wnd_proc,
        }
    }

    pub unsafe fn hook_all(&mut self) {
        self.create_move
            .init_and_hook(21, client_mode::create_move as _);

        self.level_init_post_entity
            .init_and_hook(6, client::level_init_post_entity as _);
        self.level_shutdown
            .init_and_hook(7, client::level_shutdown as _);
    }

    pub unsafe fn unhook_all(&self) {
        self.create_move.unhook();

        self.level_init_post_entity.unhook();
        self.level_shutdown.unhook();

        SetWindowLongPtrW(self.window, GWLP_WNDPROC, mem::transmute(self.wnd_proc));
    }
}
