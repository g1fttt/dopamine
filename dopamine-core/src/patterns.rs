use crate::game::{Entity, KeyValues};
use crate::{get_last_err, pcstr};

use regex::bytes::Regex;
use regex::Error;

use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::ProcessStatus::{GetModuleInformation, MODULEINFO};
use windows::Win32::System::Threading::GetCurrentProcess;

use std::ffi::c_char;
use std::mem::{self, transmute as t};
use std::slice;

pub struct Patterns {
    pub key_values_new:
        extern "thiscall" fn(*mut KeyValues, shader: *const c_char) -> *mut KeyValues,
    pub key_values_set_string:
        extern "thiscall" fn(&mut KeyValues, key: *const c_char, value: *const c_char),
    pub is_local_player: extern "thiscall" fn(&Entity) -> bool,
}

impl Patterns {
    pub unsafe fn find() -> windows::core::Result<Self> {
        let key_values_new = t(find_pattern("StudioRender.dll", "55 8B EC 56 8B F1 6A")?);
        let key_values_set_string = t(find_pattern(
            "client.dll",
            "55 8B EC 57 6A 01 FF 75 08 E8 ? ? ? ? 8B F8 85 FF 74 60",
        )?);
        let is_local_player = t(find_pattern("client.dll", "33 C0 39 0D ? ? ? ? 0F")?);

        Ok(Self {
            key_values_new,
            key_values_set_string,
            is_local_player,
        })
    }
}

unsafe fn find_pattern(module_name: &str, pattern: &str) -> windows::core::Result<*mut u8> {
    let (base, size) = module_data(module_name)?;
    let bytes = slice::from_raw_parts(base, size);
    let offset = regex_from_str(pattern)
        .ok()
        .and_then(|re| re.find(bytes))
        .map(|mat| mat.start())
        .ok_or(get_last_err!())?;
    Ok(base.add(offset))
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

unsafe fn module_data(module_name: &str) -> windows::core::Result<(*mut u8, usize)> {
    let module = GetModuleHandleA(pcstr!(module_name))?;

    let mut module_info = MODULEINFO::default();
    GetModuleInformation(
        GetCurrentProcess(),
        module,
        &mut module_info,
        mem::size_of::<MODULEINFO>() as _,
    )?;

    let base = module_info.lpBaseOfDll as *mut u8;
    let size = module_info.SizeOfImage as usize;

    Ok((base, size))
}
