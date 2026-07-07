use crate::virtual_method;

use std::ptr::NonNull;

#[repr(C)]
pub struct MemAlloc;

impl MemAlloc {
  virtual_method!(pub fn alloc[1](&self, size: usize) -> Option<NonNull<u8>>);
  virtual_method!(pub fn realloc[2](&self, ptr: NonNull<u8>, size: usize) -> Option<NonNull<u8>>);
  virtual_method!(pub fn free[3](&self, ptr: NonNull<u8>));
}
