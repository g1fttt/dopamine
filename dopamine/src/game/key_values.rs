use crate::cstr;
use crate::patterns::Patterns;

#[repr(C)]
pub struct KeyValues {
  pad: [u8; 40],
}

impl KeyValues {
  fn new_boxed(shader: &str) -> Box<Self> {
    let mut this = Box::new_uninit();
    (Patterns::get().key_values_new)(this.as_mut_ptr(), cstr!(shader));
    unsafe { this.assume_init() }
  }

  // FIXME: It's better to store it somewhere and then free on unload
  pub fn new_leaked(shader: &str) -> &'static mut Self {
    Box::leak(Self::new_boxed(shader))
  }

  pub fn set(&mut self, key: &str, value: &str) {
    (Patterns::get().key_values_set_string)(self, cstr!(key), cstr!(value));
  }
}
