mod interfaces;
mod netvars;
mod patterns;

pub use interfaces::*;
pub use netvars::*;
pub use patterns::*;

use std::ffi::c_void;

// TODO: Find a better place for this
pub unsafe fn rip_offset_value(inst_addr: *mut c_void) -> *mut c_void {
  let rip_offset = std::ptr::read_unaligned(inst_addr.byte_add(3).cast::<i32>());
  *(inst_addr
    .byte_offset(rip_offset as isize + 7 /* size of "mov rcx, cs:g_pSomething" */)
    .cast::<*mut c_void>())
}
