use crate::virtual_method;

#[repr(C)]
pub struct InputSystem;

impl InputSystem {
  virtual_method!(pub fn enable_input[7](&self, state: bool));
  virtual_method!(pub fn reset_input_state[25](&self));
}
