use crate::game::{Entity, KeyValues};

use crate::math::Vector3D;
use crate::pcstr;
use crate::utils::rip_offset_value;

use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::ProcessStatus::{GetModuleInformation, MODULEINFO};
use windows::Win32::System::Threading::GetCurrentProcess;
use windows::core::Result as WindowsResult;

use std::ffi::{c_char, c_void};
use std::sync::LazyLock;

pub struct Patterns {
  pub(crate) key_values_new:
    extern "fastcall" fn(*mut KeyValues, shader: *const c_char) -> *mut KeyValues,
  pub(crate) key_values_set_string:
    extern "fastcall" fn(&mut KeyValues, key: *const c_char, value: *const c_char),

  pub(crate) create_entity_by_name:
    extern "C" fn(class_name: *const c_char) -> Option<&'static Entity>,

  pub(crate) is_local_player: extern "fastcall" fn(&Entity) -> bool,
  pub(crate) set_model_index: extern "fastcall" fn(&Entity, index: i32),
  pub(crate) follow_entity: extern "fastcall" fn(&Entity, parent: &Entity, bone_merge: bool),
  pub(crate) set_abs_origin: extern "fastcall" fn(&Entity, origin: &Vector3D),
  pub(crate) lookup_sequence: extern "fastcall" fn(&Entity, label: *const c_char) -> i32,
  pub(crate) get_sequence_activity: extern "fastcall" fn(&Entity, sequence: i32) -> i32,

  pub calc_viewmodel_view: *mut c_void,
  pub should_flip_viewmodel: *mut c_void,

  pub d3d9_reset: *mut c_void,
  pub d3d9_present: *mut c_void,
}

impl Patterns {
  pub fn get() -> &'static Self {
    static PATTERNS: LazyLock<Patterns> = LazyLock::new(Patterns::find);
    &PATTERNS
  }

  #[rustfmt::skip]
  fn find() -> Self {
    unsafe {
      let key_values_new =
        find_by_pattern("studiorender.dll", b"\x40\x53\x48\x83\xEC?\x48\x8B\xD9\xC7\x01")
        .unwrap();
      let key_values_set_string =
        find_by_pattern("client.dll", b"\x48\x89\x5C\x24?\x55\x48\x83\xEC?\x49\x8B\xD8")
        .unwrap();

      let create_entity_by_name =
        find_by_pattern("client.dll", b"\x40\x53\x48\x83\xEC?\x48\x8B\xD9\xE8????\x48\x8B\xD3")
        .unwrap();

      let is_local_player =
        find_by_pattern("client.dll", b"\x48\x39\x0D????\x0F\x94\xC0\xC3\xCC")
        .unwrap();
      let set_model_index =
        find_by_pattern("client.dll", b"\x48\x89\x5C\x24?\x57\x48\x83\xEC?\x66\x89\x91")
        .unwrap();
      let follow_entity =
        find_by_pattern(
          "client.dll",
          b"\x48\x89\x5C\x24?\x57\x48\x83\xEC?\x41\x0F\xB6\xF8\x48\x8B\xD9",
        )
        .unwrap();
      let set_abs_origin =
        find_by_pattern(
          "client.dll",
          b"\x48\x89\x5C\x24?\x57\x48\x83\xEC?\x48\x8B\xFA\x48\x8B\xD9\xE8????\xF3\x0F\x10\x83",
        )
        .unwrap();
      let lookup_sequence =
        find_by_pattern(
          "client.dll",
          b"\x40\x53\x48\x83\xEC?\x48\x8B\xDA\xE8????\x48\x8B\xC8\x48\x8B\xD3\x48\x83\xC4?\x5B\xE9????\xCC\xCC\x4C\x89\x4C\x24",
        )
        .unwrap();

      let get_sequence_activity =
        find_by_pattern(
          "client.dll",
          b"\x48\x89\x5C\x24?\x57\x48\x83\xEC?\x8B\xDA\x48\x8B\xF9\x83\xFA?\x74",
        )
        .unwrap();

      let calc_viewmodel_view =
        find_by_pattern("client.dll", b"\x48\x89\x5C\x24?\x56\x48\x83\xEC?\xF2\x41\x0F\x10\x01")
        .unwrap();
      let should_flip_viewmodel =
        find_by_pattern(
          "client.dll",
          b"\x48\x83\xEC?\x8B\x91????\x85\xD2\x74?\xB8????\x83\xFA?\x74?\x0F\xB7\xC2\x4C\x8B\x05",
        )
        .unwrap();

      let d3d9_reset = rip_offset_value(
        find_by_pattern(
          "GameOverlayRenderer64.dll",
          b"\x48\x8B\x05????\x48\x8B\xD6\x48\x8B\xCF\xFF\xD0\x8B\xF8",
        )
        .unwrap(),
      );
      let d3d9_present = rip_offset_value(
        find_by_pattern(
          "GameOverlayRenderer64.dll",
          b"\x48\x8B\x05????\x4D\x8B\xCE\x4C\x8B\xC5",
        )
        .unwrap(),
      );

      Self {
        key_values_new,
        key_values_set_string,

        create_entity_by_name,

        is_local_player,
        set_model_index,
        follow_entity,
        set_abs_origin,
        lookup_sequence,

        get_sequence_activity,

        calc_viewmodel_view,
        should_flip_viewmodel,

        d3d9_reset,
        d3d9_present,
      }
    }
  }
}

unsafe impl Send for Patterns {}
unsafe impl Sync for Patterns {}

fn gen_bad_char_table(pattern: &[u8]) -> [usize; 256] {
  let last_wildcard = pattern.iter().rposition(|&b| b == b'?').unwrap_or(0);
  let default_shift = 1.max(pattern.len() - 1 - last_wildcard);

  let mut table = [default_shift; 256];

  for i in last_wildcard..pattern.len() - 1 {
    table[pattern[i] as usize] = pattern.len() - 1 - i;
  }
  table
}

fn find_by_pattern<T>(module_name: &str, pattern: &[u8]) -> Option<T> {
  let (module_base, module_size) = module_data(module_name).ok()?;

  let last_index = pattern.len() - 1;
  let bad_char_table = gen_bad_char_table(pattern);

  let mut current_addr = module_base;

  unsafe {
    let end = current_addr.add(module_size).sub(pattern.len());

    while current_addr <= end {
      let mut i = last_index;

      loop {
        if pattern[i] != b'?' && *current_addr.add(i) != pattern[i] {
          break;
        }

        i = match i.checked_sub(1) {
          Some(n) => n,
          None => return Some(std::mem::transmute_copy(&&*current_addr)),
        };
      }

      let char_index = *current_addr.add(last_index) as usize;
      current_addr = current_addr.add(bad_char_table[char_index]);
    }
  }

  log::error!("Failed to find {pattern:?} in {module_name}");

  None
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
