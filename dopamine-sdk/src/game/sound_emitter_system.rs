use dopamine_macros::virtual_method;

use std::ffi::c_char;
use std::path::Path;

use crate::cstr_path;

#[repr(C)]
pub struct SoundEmitterSystem;

impl SoundEmitterSystem {
  #[inline]
  pub fn add_sound_overrides(&self, script_path: impl AsRef<Path>) {
    self.add_sound_overrides_raw(cstr_path!(script_path.as_ref()), false);
  }
}

impl SoundEmitterSystem {
  virtual_method!(pub fn clear_sound_overrides[40](&self));
}

impl SoundEmitterSystem {
  virtual_method!(fn add_sound_overrides_raw[39]
    (&self, script_path: *const c_char, preload: bool));
}
