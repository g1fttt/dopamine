use dopamine_misc::rstr;

use std::ffi::c_char;

#[derive(PartialEq)]
#[repr(C)]
pub enum SendPropKind {
  NumSendPropKinds = 6,
}

#[repr(C)]
pub struct RecvProp<'a> {
  name: *const c_char,
  pub kind: SendPropKind,
  pad1: [u8; 29],
  pub table: Option<&'a RecvTable<'a>>,
  pub offset: i32,
  pad2: [u8; 12],
}

impl RecvProp<'_> {
  #[inline]
  pub fn name(&self) -> &str {
    rstr!(self.name)
  }
}

#[repr(C)]
pub struct RecvTable<'a> {
  pub props: *const RecvProp<'a>,
  pub len: i32,
  pad: [u8; 4],
  name: *const c_char,
}

impl RecvTable<'_> {
  #[inline]
  pub fn name(&self) -> &str {
    rstr!(self.name)
  }
}
