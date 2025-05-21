use crate::game::{Entity, KeyValues};

use super::rip_offset_value;
use crate::pcstr;

use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::ProcessStatus::{GetModuleInformation, MODULEINFO};
use windows::Win32::System::Threading::GetCurrentProcess;
use windows::core::{Error as WindowsError, Result as WindowsResult};

use std::ffi::{c_char, c_void};
use std::sync::LazyLock;

pub struct Patterns {
  pub key_values_new: extern "fastcall" fn(*mut KeyValues, shader: *const c_char) -> *mut KeyValues,
  pub key_values_set_string: extern "fastcall" fn(&mut KeyValues, *const c_char, *const c_char),

  pub is_local_player: extern "fastcall" fn(&Entity) -> bool,

  pub calc_viewmodel_view: *mut c_void,

  pub d3d9_reset: *mut c_void,
  pub d3d9_present: *mut c_void,
}

impl Patterns {
  pub fn get() -> &'static Self {
    static PATTERNS: LazyLock<Patterns> = LazyLock::new(|| unsafe {
      Patterns::find()
        .inspect_err(|err| log::error!("Failed to find patterns: {}", err))
        .unwrap()
    });
    &PATTERNS
  }

  unsafe fn find() -> WindowsResult<Self> {
    unsafe {
      let key_values_new = find_pattern("studiorender.dll", b"\x40\x53\x48\x83\xEC?\x48\x8B\xD9\xC7\x01")?;
      let key_values_set_string = find_pattern("client.dll", b"\x48\x89\x5C\x24?\x55\x48\x83\xEC?\x49\x8B\xD8")?;

      let is_local_player = find_pattern("client.dll", b"\x48\x39\x0D????\x0F\x94\xC0\xC3\xCC")?;

      let calc_viewmodel_view = find_pattern("client.dll", b"\x48\x89\x5C\x24?\x56\x48\x83\xEC?\xF2\x41\x0F\x10\x01")?;

      let d3d9_reset = rip_offset_value(find_pattern::<*mut c_void>(
        "GameOverlayRenderer64.dll",
        b"\x48\x8B\x05????\x48\x8B\xD6\x48\x8B\xCF\xFF\xD0\x8B\xF8",
      )?);

      let d3d9_present = rip_offset_value(find_pattern::<*mut c_void>(
        "GameOverlayRenderer64.dll",
        b"\x48\x8B\x05????\x4D\x8B\xCE\x4C\x8B\xC5",
      )?);

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

  unsafe {
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
  }

  log::error!("Failed to find {:?} in {}", pattern, module_name);

  Err(WindowsError::empty())
}

fn module_data(module_name: &str) -> WindowsResult<(*const u8, usize)> {
  let module = unsafe { GetModuleHandleA(pcstr!(module_name))? };

  let mut module_info = MODULEINFO::default();
  unsafe { GetModuleInformation(GetCurrentProcess(), module, &mut module_info, size_of::<MODULEINFO>() as _)? };

  let base = module_info.lpBaseOfDll as *const u8;
  let size = module_info.SizeOfImage as usize;

  Ok((base, size))
}
