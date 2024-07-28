use super::RecvTable;

use std::ffi::c_char;

#[repr(C)]
pub struct ClientClass<'a> {
  pad: [u8; 8],
  pub name: *const c_char,
  pub recv_table: &'a RecvTable<'a>,
  pub next: Option<&'a ClientClass<'a>>,
  pub class_id: i32,
}
