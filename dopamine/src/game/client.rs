use super::{ClientClass, Entity};

use dopamine_macros::virtual_method;

#[repr(C)]
pub struct Client;

impl Client {
  #[virtual_method(index = 8)]
  fn all_classes(&self) -> Option<&ClientClass>;
}

#[repr(C)]
pub struct ClientMode;

#[repr(C)]
pub struct EntityList;

impl EntityList {
  #[virtual_method(index = 3)]
  fn get_entity_by_index(&self, index: i32) -> Option<&Entity>;

  #[virtual_method(index = 4)]
  fn get_entity_from_handle(&self, handle: i32) -> Option<&Entity>;
}
