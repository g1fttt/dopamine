use std::ffi::c_void;

pub unsafe fn resolve_rip<T>(base: *mut c_void) -> *mut T {
  unsafe { rel32_target(base, 0x3) }
}

unsafe fn rel32_target<T>(base: *mut c_void, offset: isize) -> *mut T {
  unsafe {
    let target_offset = base.byte_offset(offset).cast::<i32>().read_unaligned();
    let instr_end = base.byte_offset(offset + size_of::<i32>() as isize);
    instr_end.byte_offset(target_offset as isize).cast::<*mut T>().read()
  }
}
