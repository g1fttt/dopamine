use crate::empty_err;

use windows::Win32::System::Memory::{
    VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS,
};

use std::ffi::c_void;
use std::{mem, ptr};

pub struct VMTHook {
    original: *mut c_void,
    ptr_to_target: *mut *mut c_void,
}

impl VMTHook {
    pub unsafe fn new<T>(base: &T, idx: usize) -> Self {
        let base = base as *const T as *mut c_void;
        let vtable = *base.cast::<*mut *mut c_void>();
        let ptr_to_target = vtable.add(idx);

        Self {
            ptr_to_target,
            original: *ptr_to_target,
        }
    }

    pub unsafe fn hook(&self, hook: *const ()) -> windows::core::Result<()> {
        let mut old = PAGE_PROTECTION_FLAGS::default();
        if VirtualProtect(self.ptr_to_target as _, 4, PAGE_EXECUTE_READWRITE, &mut old).is_ok() {
            *self.ptr_to_target = hook as _;
            VirtualProtect(self.ptr_to_target as _, 4, old, ptr::null_mut())
        } else {
            Err(empty_err!())
        }
    }

    pub unsafe fn unhook(&self) -> windows::core::Result<()> {
        let mut old = PAGE_PROTECTION_FLAGS::default();
        if VirtualProtect(self.ptr_to_target as _, 4, PAGE_EXECUTE_READWRITE, &mut old).is_ok() {
            *self.ptr_to_target = self.original;
            VirtualProtect(self.ptr_to_target as _, 4, old, ptr::null_mut())
        } else {
            Err(empty_err!())
        }
    }

    pub fn original<T>(&self) -> &T {
        unsafe { mem::transmute(&self.original) }
    }
}
