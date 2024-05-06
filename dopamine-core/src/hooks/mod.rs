mod client;
mod client_mode;
mod winapi;

use crate::interfaces::Interfaces;
use crate::pcstr;
use crate::utils::VMTHook;

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowA, SetWindowLongPtrW, GWLP_WNDPROC, WNDPROC,
};

use std::mem;

pub struct Hooks {
    window: HWND,
    pub wnd_proc: WNDPROC,
    pub create_move: VMTHook,
    pub level_init_post_entity: VMTHook,
    pub level_shutdown: VMTHook,
}

impl Hooks {
    pub unsafe fn create(interfaces: &Interfaces) -> Self {
        Self {
            window: FindWindowA(pcstr!("Valve001"), pcstr!()),
            wnd_proc: None,
            create_move: VMTHook::new(interfaces.client_mode, 21),
            level_init_post_entity: VMTHook::new(interfaces.client, 6),
            level_shutdown: VMTHook::new(interfaces.client, 7),
        }
    }

    pub unsafe fn hook_all(&mut self) -> windows::core::Result<()> {
        self.wnd_proc = {
            #[allow(clippy::fn_to_numeric_cast)]
            mem::transmute(SetWindowLongPtrW(
                self.window,
                GWLP_WNDPROC,
                winapi::wnd_proc as _,
            ))
        };

        self.create_move.hook(client_mode::create_move as _)?;

        self.level_init_post_entity
            .hook(client::level_init_post_entity as _)?;
        self.level_shutdown.hook(client::level_shutdown as _)?;

        Ok(())
    }

    pub unsafe fn unhook_all(&self) -> windows::core::Result<()> {
        self.create_move.unhook()?;

        self.level_init_post_entity.unhook()?;
        self.level_shutdown.unhook()?;

        SetWindowLongPtrW(self.window, GWLP_WNDPROC, mem::transmute(self.wnd_proc));

        Ok(())
    }
}
