use crate::game::{ClientClass, Entity};

use dopamine_proc_macro::virtual_method;

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
    fn get_entity_by_index(&self, idx: i32) -> Option<&Entity>;
}
