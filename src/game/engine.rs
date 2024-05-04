use crate::macros::call_vmethod;

#[repr(C)]
pub struct Engine;

impl Engine {
    pub fn local_player_index(&self) -> i32 {
        call_vmethod!(self, i32, 12, (&Self), (self))
    }
}
