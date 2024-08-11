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
  AWP = 163,
  Elite = 167,
  Famas = 168,
  FiveSeven = 169,
  G3SG1 = 170,
  M249 = 173,
  Galil = 171,
  Glock = 172,
  M3 = 174,
  M4A1 = 175,
  Mac10 = 176,
  Mp5N = 177,
  P228 = 178,
  P90 = 179,
  Scout = 180,
  Sg550 = 181,
  Sg552 = 182,
  Tmp = 183,
  Ump45 = 184,
  Usp = 185,
  Xm1014 = 186,
}
