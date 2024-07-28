use dopamine_macros::virtual_method;

#[repr(C)]
pub struct Surface;

impl Surface {
  virtual_method!(pub fn is_cursor_visible[53](&self) -> bool);
  virtual_method!(pub fn unlock_cursor[61](&self));
}
