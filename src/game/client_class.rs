use crate::game::RecvTable;

use std::ffi::c_char;

#[repr(C)]
pub struct ClientClass {
    pad: [u8; 8],
    pub name: *const c_char,
    pub recv_table: &'static RecvTable,
    pub next: Option<&'static ClientClass>,
}
