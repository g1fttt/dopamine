use dopamine_macros::virtual_method;

#[repr(C)]
pub struct InputSystem;

impl InputSystem {
  virtual_method!(pub fn enable_input(&self, state: bool) [7]);
  virtual_method!(pub fn reset_input_state(&self) [25]);
}
