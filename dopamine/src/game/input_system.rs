use dopamine_macros::virtual_method;

#[repr(C)]
pub struct InputSystem;

impl InputSystem {
  #[virtual_method(index = 7)]
  fn enable_input(&self, state: bool);

  #[virtual_method(index = 25)]
  fn reset_input_state(&self);
}
