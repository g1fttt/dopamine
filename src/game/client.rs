use crate::call_vmethod;
use crate::game::{ClientClass, Entity};

#[repr(C)]
pub struct Client;

impl Client {
    pub fn all_classes(&self) -> Option<&ClientClass> {
        call_vmethod!(self, Option<&ClientClass>, 8, (&Self), (self))
    }
}

#[repr(C)]
pub struct ClientMode;

#[repr(C)]
pub struct EntityList;

impl EntityList {
    pub fn get_entity_by_index(&self, idx: i32) -> Option<&Entity> {
        call_vmethod!(self, Option<&Entity>, 3, (&Self, i32), (self, idx))
    }
}
