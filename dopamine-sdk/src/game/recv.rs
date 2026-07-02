use crate::rstr;

use open_enum::open_enum;

use std::ffi::{c_char, c_void};

#[open_enum]
#[repr(C)]
pub enum SendPropKind {
  NumSendPropKinds = 6,
}

#[repr(C)]
pub struct RecvProp<'a> {
  name: *const c_char,
  pub kind: SendPropKind,
  pad1: [u8; 33],
  proxy: *mut c_void,
  pad2: [u8; 8],
  pub table: Option<&'a RecvTable<'a>>,
  pub offset: i32,
  pad3: [u8; 16],
}

impl RecvProp<'_> {
  pub fn name(&self) -> &'static str {
    unsafe { rstr!(self.name) }
  }
}

#[repr(C)]
pub union DataTableVariant {
  pub float: f32,
  pub int: i32,

  // TODO: Safe wrappers?
  string: *const c_char,
  data: *mut c_void,

  pub vector: [f32; 3],
  pub int64: i64,
}

#[repr(C)]
pub struct RecvTable<'a> {
  props: *const RecvProp<'a>,
  len: i32,
  pad1: [u8; 8],
  name: *const c_char,
}

impl<'a> RecvTable<'a> {
  pub fn name(&self) -> &'static str {
    unsafe { rstr!(self.name) }
  }

  pub fn props(&self) -> impl Iterator<Item = &RecvProp<'a>> {
    unsafe { std::slice::from_raw_parts(self.props, self.len as usize) }.iter()
  }
}
