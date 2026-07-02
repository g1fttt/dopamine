use crate::cstr;
use crate::utils::Patterns;

use bumpalo::Bump;

use std::alloc::Layout;

#[repr(C)]
pub struct KeyValues {
  pad: [u8; 68],
}

impl KeyValues {
  #[allow(clippy::mut_from_ref)]
  pub fn alloc_in_bump<'a>(shader: &str, bump: &'a Bump) -> &'a mut KeyValues {
    let layout = Layout::new::<KeyValues>();
    let mut this = bump.alloc_layout(layout).cast::<KeyValues>();

    (Patterns::get().key_values_new)(this.as_ptr(), cstr!(shader));

    unsafe { this.as_mut() }
  }

  pub fn set(&mut self, key: &str, value: &str) {
    (Patterns::get().key_values_set_string)(self, cstr!(key), cstr!(value));
  }
}
