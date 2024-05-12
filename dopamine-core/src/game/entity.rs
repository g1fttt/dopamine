use crate::App;

use dopamine_macros::{netvar, virtual_method};

#[repr(C)]
pub struct UserCommand {
    pad: [u8; 36],
    pub buttons: i32,
}

impl UserCommand {
    pub const IN_JUMP: i32 = 1 << 1;
}

#[repr(C)]
pub struct Entity;

impl Entity {
    const ON_GROUND: i32 = 1 << 0;

    pub fn is_on_ground(&self) -> bool {
        (self.flags() & Self::ON_GROUND) != 0
    }

    pub fn move_child(&self) -> Option<&Self> {
        let handle = unsafe { *(self as *const Self).byte_add(0x184).cast::<i32>() };
        App::interfaces().entity_list.get_entity_from_handle(handle)
    }

    pub fn move_peer(&self) -> Option<&Self> {
        let handle = unsafe { *(self as *const Self).byte_add(0x188).cast::<i32>() };
        App::interfaces().entity_list.get_entity_from_handle(handle)
    }

    pub fn is_local_player(&self) -> bool {
        (App::patterns().is_local_player)(self)
    }

    pub fn is_dormant(&self) -> bool {
        self.networkable().is_dormant()
    }

    pub fn should_draw(&self) -> bool {
        self.renderable().should_draw()
    }

    pub fn draw_model(&self) -> i32 {
        self.renderable().draw_model()
    }

    #[virtual_method(index = 4, private)]
    fn networkable(&self) -> &NetworkableEntity;

    #[virtual_method(index = 5, private)]
    fn renderable(&self) -> &RenderableEntity;

    #[virtual_method(index = 131)]
    fn is_player(&self) -> bool;

    #[netvar(path = "CBaseEntity->m_iTeamNum")]
    fn team(&self) -> i32;

    #[netvar(path = "CBasePlayer->m_fFlags")]
    fn flags(&self) -> i32;
}

#[repr(C)]
struct NetworkableEntity;

impl NetworkableEntity {
    #[virtual_method(index = 8, private)]
    fn is_dormant(&self) -> bool;
}

#[repr(C)]
struct RenderableEntity;

impl RenderableEntity {
    fn draw_model(&self) -> i32 {
        self.draw_model_private(1 /* StudioRender */)
    }
}

impl RenderableEntity {
    #[virtual_method(index = 3, private)]
    fn should_draw(&self) -> bool;

    #[virtual_method(index = 10, private)]
    fn draw_model_private(&self, flags: i32) -> i32;
}
