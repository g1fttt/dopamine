pub use minhook::{MinHook, MH_STATUS};

use crate::virtual_method;

use std::ffi::c_void;
use std::marker::FnPtr;
use std::{mem, ptr};

pub type HookResult<T> = Result<T, MH_STATUS>;

pub struct Hook<F: FnPtr> {
  target: *mut c_void,
  pub original: F,
}

impl<F: FnPtr> Hook<F> {
  pub unsafe fn new_virtual<T>(base: &T, index: usize) -> Self {
    Self {
      target: virtual_method!(base, index),
      original: mem::transmute_copy(&ptr::null::<c_void>()),
    }
  }

  pub unsafe fn detour_to(&mut self, hook: F) -> HookResult<()> {
    self.original =
      mem::transmute_copy(&MinHook::create_hook(self.target, mem::transmute_copy(&hook))?);
    Ok(())
  }

  #[inline]
  pub unsafe fn remove(&self) -> HookResult<()> {
    MinHook::remove_hook(self.target)
  }
}
