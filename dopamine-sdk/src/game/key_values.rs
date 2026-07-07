use std::ptr::NonNull;

use crate::cstr;
use crate::interfaces::mem_alloc;
use crate::utils::Patterns;

#[repr(C)]
pub struct KeyValues {
  pad: [u8; 68],
}

impl KeyValues {
  /// # Safety:
  /// Shall panic if an underlying allocation was failed.
  pub fn new(shader: &str) -> &'static mut KeyValues {
    let this = mem_alloc()
      .alloc(size_of::<KeyValues>())
      .map(|ptr| ptr.cast::<KeyValues>().as_ptr())
      .unwrap();

    (Patterns::get().key_values_new)(this, cstr!(shader));

    unsafe { this.as_mut().unwrap() }
  }

  pub fn set(&mut self, key: &str, value: &str) {
    (Patterns::get().key_values_set_string)(self, cstr!(key), cstr!(value));
  }
}

impl Drop for KeyValues {
  fn drop(&mut self) {
    mem_alloc().free(NonNull::from_mut(self).cast::<u8>());
  }
}
