use dopamine_macros::virtual_method;

#[repr(C)]
pub struct Surface;

impl Surface {
  virtual_method!(pub fn is_cursor_visible(&self) -> bool [53]);
  virtual_method!(pub fn unlock_cursor(&self) [61]);
}
