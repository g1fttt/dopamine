use winapi::um::memoryapi::VirtualProtect;
use winapi::um::winnt::PAGE_EXECUTE_READWRITE;

use std::ffi::c_void;
use std::{mem, ptr};

pub struct VMTHook {
    base: *mut c_void,
    original: *mut c_void,
    ptr_to_target: *mut *mut c_void,
}

impl VMTHook {
    pub fn from_base<T>(base: &T) -> Self {
        Self {
            base: base as *const T as _,
            original: ptr::null_mut(),
            ptr_to_target: ptr::null_mut(),
        }
    }

    pub unsafe fn init_and_hook(&mut self, idx: usize, hook: *const ()) {
        let vtable = *self.base.cast::<*mut *mut c_void>();
        self.ptr_to_target = vtable.add(idx);
        self.original = *self.ptr_to_target;

        let mut old = 0;
        if VirtualProtect(self.ptr_to_target as _, 4, PAGE_EXECUTE_READWRITE, &mut old) != 0 {
            *self.ptr_to_target = hook as _;
            VirtualProtect(self.ptr_to_target as _, 4, old, ptr::null_mut());
        }
    }

    pub unsafe fn unhook(&self) {
        let mut old = 0;
        if VirtualProtect(self.ptr_to_target as _, 4, PAGE_EXECUTE_READWRITE, &mut old) != 0 {
            *self.ptr_to_target = self.original;
            VirtualProtect(self.ptr_to_target as _, 4, old, ptr::null_mut());
        }
    }

    pub fn original<T>(&self) -> &T {
        unsafe { mem::transmute(&self.original) }
    }
}
