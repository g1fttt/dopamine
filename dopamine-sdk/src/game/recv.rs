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
  pub proxy: Option<RecvPropProxy>,
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

pub type RecvPropProxy =
  extern "C" fn(&mut RecvPropProxyData, r#struct: *mut c_void, out: *mut c_void);

#[repr(C)]
pub struct RecvPropProxyData<'a> {
  pub recv_prop: Option<&'a RecvProp<'a>>,
  pub value: DataTableVariant,
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
  pub props: *mut RecvProp<'a>,
  pub len: i32,
  pad1: [u8; 8],
  name: *const c_char,
}

impl RecvTable<'_> {
  pub fn name(&self) -> &'static str {
    unsafe { rstr!(self.name) }
  }
}
