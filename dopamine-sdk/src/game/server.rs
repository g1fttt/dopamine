use crate::GlobalVars;

use dopamine_macros::virtual_method;

#[repr(C)]
pub struct Server;

impl Server {
  virtual_method!(pub fn global_vars[1](&self) -> &GlobalVars);
}
