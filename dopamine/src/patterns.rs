use crate::game::{Entity, KeyValues};
use crate::{get_last_err, pcstr};

use regex::bytes::Regex;
use regex::Error;

use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::ProcessStatus::{GetModuleInformation, MODULEINFO};
use windows::Win32::System::Threading::GetCurrentProcess;

use std::ffi::{c_char, c_void};
use std::sync::LazyLock;
use std::{mem, slice};

pub struct Patterns {
  pub key_values_new: extern "thiscall" fn(*mut KeyValues, *const c_char) -> *mut KeyValues,
  pub key_values_set_string: extern "thiscall" fn(&mut KeyValues, *const c_char, *const c_char),

  pub is_local_player: extern "thiscall" fn(&Entity) -> bool,

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
  unsafe fn find() -> windows::core::Result<Self> {
    let key_values_new = find_pattern("StudioRender.dll", "55 8B EC 56 8B F1 6A")?;
    let key_values_set_string = find_pattern("client.dll", "55 8B EC 57 6A 01 FF 75 08 E8 ? ? ? ? 8B F8 85 FF 74 60")?;

    let is_local_player = find_pattern("client.dll", "33 C0 39 0D ? ? ? ? 0F")?;

    let d3d9_reset = find_pattern::<*mut c_void>("GameOverlayRenderer.dll", "A1 ? ? ? ? 57 53 C7 45 FC 00 00 00 00")?
      .byte_add(1);
    let d3d9_present = find_pattern::<*mut c_void>("GameOverlayRenderer.dll", "A1 ? ? ? ? 51 FF 75 14")?
      .byte_add(1);

    Ok(Self {
      key_values_new,
      key_values_set_string,

      is_local_player,

      d3d9_reset,
      d3d9_present,
    })
  }
}

unsafe impl Send for Patterns {}
unsafe impl Sync for Patterns {}

unsafe fn find_pattern<T>(module_name: &str, pattern: &str) -> windows::core::Result<T> {
  let (base, size) = module_data(module_name)?;
  let bytes = slice::from_raw_parts(base, size);
  let offset = regex_from_str(pattern)
    .ok()
    .and_then(|re| re.find(bytes))
    .map(|mat| mat.start())
    .ok_or(get_last_err!())?;
  Ok(mem::transmute_copy(&&*base.byte_add(offset)))
}

fn regex_from_str(s: &str) -> Result<Regex, Error> {
  let mut re = s
    .split_whitespace()
    .map(|b| match b {
      "?" => ".".to_owned(),
      b => format!("\\x{}", b),
    })
    .collect::<Vec<_>>()
    .join("");
  re.insert_str(0, "(?s-u)");
  Regex::new(&re)
}

fn module_data(module_name: &str) -> windows::core::Result<(*mut u8, usize)> {
  let module = unsafe { GetModuleHandleA(pcstr!(module_name))? };

  let mut module_info = MODULEINFO::default();
  unsafe {
    GetModuleInformation(
      GetCurrentProcess(),
      module,
      &mut module_info,
      mem::size_of::<MODULEINFO>() as _,
    )?
  };

  let base = module_info.lpBaseOfDll as *mut u8;
  let size = module_info.SizeOfImage as usize;

  Ok((base, size))
}
