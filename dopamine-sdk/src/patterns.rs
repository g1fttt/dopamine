use crate::game::{Entity, KeyValues};

use dopamine_misc::pcstr;

use windows::core::{Error as WindowsError, Result as WindowsResult};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::ProcessStatus::{GetModuleInformation, MODULEINFO};
use windows::Win32::System::Threading::GetCurrentProcess;

use std::ffi::{c_char, c_void};
use std::sync::LazyLock;

pub struct Patterns {
  pub key_values_new: extern "thiscall" fn(*mut KeyValues, shader: *const c_char) -> *mut KeyValues,
  pub key_values_set_string: extern "thiscall" fn(&mut KeyValues, *const c_char, *const c_char),

  pub is_local_player: extern "thiscall" fn(&Entity) -> bool,

  pub calc_viewmodel_view: *mut c_void,

  pub d3d9_reset: *mut c_void,
  pub d3d9_present: *mut c_void,
}

impl Patterns {
  pub fn get() -> &'static Self {
    static PATTERNS: LazyLock<Patterns> =
      LazyLock::new(|| unsafe { Patterns::find().expect("Failed to find patterns") });
    &PATTERNS
  }

  #[rustfmt::skip]
  unsafe fn find() -> WindowsResult<Self> {
    let key_values_new = find_pattern("StudioRender.dll", b"\x55\x8B\xEC\x56\x8B\xF1\x6A")?;
    let key_values_set_string = find_pattern("client.dll", b"\x55\x8B\xEC\x57\x6A\x01\xFF\x75\x08\xE8????\x8B\xF8\x85\xFF\x74\x60")?;

    let is_local_player = find_pattern("client.dll", b"\x33\xC0\x39\x0D????\x0F")?;

    let calc_viewmodel_view = find_pattern("client.dll", b"\x55\x8B\xEC\x83\xEC?\x8B\x55?\x56\x57\x8B\xF9\x8B\x4D")?;

    let d3d9_reset = find_pattern::<*mut c_void>("GameOverlayRenderer.dll", b"\xA1????\x57\x53\xC7\x45\xFC\x00\x00\x00\x00")?
      .byte_add(1);
    let d3d9_present = find_pattern::<*mut c_void>("GameOverlayRenderer.dll", b"\xA1????\x51\xFF\x75\x14")?
      .byte_add(1);

    Ok(Self {
      key_values_new,
      key_values_set_string,

      is_local_player,

      calc_viewmodel_view,

      d3d9_reset,
      d3d9_present,
    })
  }
}

unsafe impl Send for Patterns {}
unsafe impl Sync for Patterns {}

fn gen_bad_char_table(pattern: &[u8]) -> [usize; 256] {
  let last_wildcard = pattern.iter().rposition(|&b| b == b'?').unwrap_or(0);
  let default_shift = 1.max(pattern.len() - 1 - last_wildcard);

  let mut table = [default_shift; _];
  for i in last_wildcard..pattern.len() - 1 {
    table[pattern[i] as usize] = pattern.len() - 1 - i;
  }
  table
}

unsafe fn find_pattern<T>(module_name: &str, pattern: &[u8]) -> WindowsResult<T> {
  let (module_base, module_size) = module_data(module_name)?;

  let last_index = pattern.len() - 1;
  let bad_char_table = gen_bad_char_table(pattern);

  let mut start = module_base;
  let end = start.add(module_size).sub(pattern.len());

  while start <= end {
    let mut i = last_index as isize;
    while i >= 0 && (pattern[i as usize] == b'?' || *start.add(i as usize) == pattern[i as usize]) {
      i -= 1;
    }

    if i < 0 {
      return Ok(std::mem::transmute_copy(&&*start));
    }

    let char_index = *start.add(last_index) as usize;
    start = start.add(bad_char_table[char_index]);
  }
  Err(WindowsError::empty())
}

fn module_data(module_name: &str) -> WindowsResult<(*const u8, usize)> {
  let module = unsafe { GetModuleHandleA(pcstr!(module_name))? };

  let mut module_info = MODULEINFO::default();
  unsafe {
    GetModuleInformation(
      GetCurrentProcess(),
      module,
      &mut module_info,
      size_of::<MODULEINFO>() as _,
    )?
  };

  let base = module_info.lpBaseOfDll as *const u8;
  let size = module_info.SizeOfImage as usize;

  Ok((base, size))
}
