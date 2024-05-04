mod client;
mod client_mode;

use crate::interfaces::Interfaces;
use crate::utils::VMTHook;

pub struct Hooks {
    pub create_move: VMTHook,
    pub level_init_post_entity: VMTHook,
    pub level_shutdown: VMTHook,
}

impl Hooks {
    pub fn create(interfaces: &Interfaces) -> Self {
        Self {
            create_move: VMTHook::from_base(interfaces.client_mode),
            level_init_post_entity: VMTHook::from_base(interfaces.client),
            level_shutdown: VMTHook::from_base(interfaces.client),
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
}
