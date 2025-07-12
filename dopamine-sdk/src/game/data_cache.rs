use crate::{cstr_path, rstr};

use dopamine_macros::virtual_method;

use std::ffi::c_char;
use std::path::Path;

#[repr(C)]
pub struct MdlCache;

impl MdlCache {
  #[inline]
  pub fn find_mdl(&self, relative_path: impl AsRef<Path>) -> ModelHandle {
    self.find_mdl_raw(cstr_path!(relative_path.as_ref()))
  }

  #[inline]
  pub fn get_model_name(&self, handle: ModelHandle) -> &'static str {
    unsafe { rstr!(self.get_model_name_raw(handle)) }
  }
}

impl MdlCache {
  virtual_method!(fn find_mdl_raw[6](&self, relative_path: *const c_char) -> ModelHandle);
  virtual_method!(fn get_model_name_raw[23](&self, handle: ModelHandle) -> *const c_char);
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct ModelHandle(u16);

impl ModelHandle {
  #[inline]
  pub fn invalid() -> Self {
    Self(u16::MAX)
  }

  #[inline]
  pub fn is_invalid(self) -> bool {
    self.0 == u16::MAX
  }
}
