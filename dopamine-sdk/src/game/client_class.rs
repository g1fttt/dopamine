use super::RecvTable;

use dopamine_utils::rstr;
use strum::FromRepr;

use std::ffi::c_char;

#[repr(C)]
pub struct ClientClass<'a> {
  pad: [u8; 8],
  name: *const c_char,
  pub recv_table: &'a RecvTable<'a>,
  pub next: Option<&'a ClientClass<'a>>,
  pub id: i32,
}

impl ClientClass<'_> {
  #[inline]
  pub fn name(&self) -> &str {
    rstr!(self.name)
  }
}

#[derive(FromRepr)]
#[repr(C)]
pub enum WeaponClassId {
  Ak47 = 1,
  C4 = 23,
  DEagle = 31,
  Aug = 162,
  AWP,
  Elite = 167,
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
