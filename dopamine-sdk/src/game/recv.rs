use crate::rstr;

use open_enum::open_enum;

use std::ffi::c_char;

#[open_enum]
#[repr(C)]
pub enum SendPropKind {
  NumSendPropKinds = 6,
}

#[repr(C)]
pub struct RecvProp<'a> {
  name: *const c_char,
  pub kind: SendPropKind,
  pad1: [u8; 49],
  pub table: Option<&'a RecvTable<'a>>,
  pub offset: i32,
  pad2: [u8; 16],
}

impl RecvProp<'_> {
  pub fn name(&self) -> &str {
    unsafe { rstr!(self.name) }
  }
}

#[repr(C)]
pub struct RecvTable<'a> {
  pub props: *const RecvProp<'a>,
  pub len: i32,
  pad1: [u8; 8],
  name: *const c_char,
}

impl RecvTable<'_> {
  pub fn name(&self) -> &str {
    unsafe { rstr!(self.name) }
  }
}
