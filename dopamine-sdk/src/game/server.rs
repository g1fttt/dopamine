use crate::{GlobalVars, virtual_method};

#[repr(C)]
pub struct PlayerInfoManager;

impl PlayerInfoManager {
  virtual_method!(pub fn global_vars[1](&self) -> &GlobalVars);
}
