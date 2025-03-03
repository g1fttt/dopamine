use super::RecvTable;
use crate::rstr;

use open_enum::open_enum;

use std::ffi::c_char;

#[repr(C)]
pub struct ClientClass<'a> {
  pad: [u8; 16],
  name: *const c_char,
  pub recv_table: &'a RecvTable<'a>,
  pub next: Option<&'a ClientClass<'a>>,
  pub id: ClassId,
}

impl ClientClass<'_> {
  pub fn name(&self) -> &str {
    unsafe { rstr!(self.name) }
  }
}

#[derive(Clone, Copy)]
#[open_enum]
#[repr(C)]
pub enum ClassId {
  Ak47 = 1,
  C4 = 23,
  DEagle = 31,
  PredictedViewModel = 89,
  Aug = 163,
  AWP,
  Elite = 168,
  Famas,
  FiveSeven,
  G3SG1,
  Galil,
  Glock,
  M249,
  M3,
  M4A1,
  Mac10,
  Mp5N,
  P228,
  P90,
  Scout,
  Sg550,
  Sg552,
  Tmp,
  Ump45,
  Usp,
  Xm1014,
}
