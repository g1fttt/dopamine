use crate::call_vmethod;
use crate::game::Entity;

#[repr(C)]
pub struct Client;

#[repr(C)]
pub struct ClientMode;

#[repr(C)]
pub struct EntityList;

impl EntityList {
    pub fn get_entity_by_index(&self, idx: i32) -> Option<&Entity> {
        unsafe { call_vmethod!(self, *const Entity, 3, (&Self, i32), (self, idx)).as_ref() }
    }
}
