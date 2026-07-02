use crate::{ClientClass, Entity, EntityHandle};

use dopamine_macros::virtual_method;

#[repr(C)]
pub struct Client;

impl Client {
  virtual_method!(pub fn all_classes[8](&self) -> Option<&ClientClass<'_>>);
}

#[repr(C)]
pub struct ClientMode;

#[repr(C)]
pub struct EntityList;

impl EntityList {
  virtual_method!(pub fn get_entity_by_index[3](&self, index: i32) -> Option<&Entity>);
  virtual_method!(pub fn get_entity_from_handle<'a>[4](&self, handle: &EntityHandle) -> Option<&'a Entity>);
  virtual_method!(pub fn highest_entity_index[6](&self) -> i32);
}
