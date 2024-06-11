use dopamine_macros::virtual_method;

#[repr(C)]
pub struct Surface;

impl Surface {
  #[virtual_method(index = 61)]
  fn unlock_cursor(&self);
}
