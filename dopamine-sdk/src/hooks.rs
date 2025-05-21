pub use minhook::MH_STATUS;

use minhook::MinHook;

use windows::Win32::System::Memory::{
  PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS, VirtualProtect,
};
use windows::core::{Error as WindowsError, Result as WindowsResult};

use std::ffi::c_void;
use std::marker::FnPtr;
use std::{mem, ptr};

#[derive(Debug)]
pub enum HookError {
  Trampoline(MH_STATUS),
  Vmt(WindowsError),
}

pub type HookResult<T> = std::result::Result<T, HookError>;

pub trait Hook<F: FnPtr> {
  unsafe fn detour_to(&mut self, hook: F) -> HookResult<()>;
  unsafe fn remove(&self) -> HookResult<()>;
}

pub struct TrampolineHook<F: FnPtr> {
  target: *mut c_void,
  pub original: F,
}

impl<F: FnPtr> TrampolineHook<F> {
  pub unsafe fn new(target: *mut c_void) -> Self {
    Self {
      target,
      original: unsafe { mem::transmute_copy(&ptr::null::<c_void>()) },
    }
  }
}

impl<F: FnPtr> Hook<F> for TrampolineHook<F> {
  unsafe fn detour_to(&mut self, hook: F) -> HookResult<()> {
    unsafe {
      self.original = mem::transmute_copy(
        &(MinHook::create_hook(self.target, mem::transmute_copy(&hook))
          .map_err(HookError::Trampoline)?),
      );
    }
    Ok(())
  }

  unsafe fn remove(&self) -> HookResult<()> {
    unsafe { MinHook::remove_hook(self.target).map_err(HookError::Trampoline) }
  }
}

pub struct VmtHook<F: FnPtr> {
  ptr_to_target: *mut *mut c_void,
  pub original: F,
}

impl<F: FnPtr> VmtHook<F> {
  pub unsafe fn new<T>(base: &T, index: usize) -> Self {
    let base = base as *const T as *mut c_void;

    unsafe {
      let vtable = *base.cast::<*mut *mut c_void>();
      let ptr_to_target = vtable.add(index);

      Self {
        ptr_to_target,
        original: mem::transmute_copy(&(*ptr_to_target)),
      }
    }
  }

  unsafe fn swap_target_to(&self, callback: F) -> WindowsResult<()> {
    let mut old = PAGE_PROTECTION_FLAGS::default();

    unsafe {
      VirtualProtect(
        self.ptr_to_target as *mut c_void,
        size_of::<usize>(),
        PAGE_EXECUTE_READWRITE,
        &mut old,
      )?;

      *self.ptr_to_target = mem::transmute_copy(&callback);

      VirtualProtect(
        self.ptr_to_target as *mut c_void,
        size_of::<usize>(),
        old,
        ptr::null_mut(),
      )
    }
  }
}

impl<F: FnPtr> Hook<F> for VmtHook<F> {
  unsafe fn detour_to(&mut self, hook: F) -> HookResult<()> {
    unsafe { self.swap_target_to(hook).map_err(HookError::Vmt) }
  }

  unsafe fn remove(&self) -> HookResult<()> {
    unsafe { self.swap_target_to(self.original).map_err(HookError::Vmt) }
  }
}

pub unsafe fn enable_all_hooks() -> HookResult<()> {
  unsafe { MinHook::enable_all_hooks().map_err(HookError::Trampoline) }
}

pub unsafe fn disable_all_hooks() -> HookResult<()> {
  unsafe { MinHook::disable_all_hooks().map_err(HookError::Trampoline) }
}
