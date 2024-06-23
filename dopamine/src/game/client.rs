use super::{ClientClass, Entity};

use dopamine_macros::virtual_method;

#[repr(C)]
pub struct Client;

impl Client {
  virtual_method!(pub fn all_classes(&self) -> Option<&ClientClass> [8]);
}

#[repr(C)]
pub struct ClientMode;

#[repr(C)]
pub struct EntityList;

impl EntityList {
  virtual_method!(pub fn get_entity_by_index(&self, index: i32) -> Option<&Entity> [3]);
  virtual_method!(pub fn get_entity_from_handle(&self, handle: i32) -> Option<&Entity> [4]);
}
