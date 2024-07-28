use crate::game::{RecvTable, SendPropKind};
use crate::interfaces::Interfaces;

use std::collections::HashMap;
use std::ffi::{c_char, CStr};
use std::sync::LazyLock;

pub type Offsets<'a> = HashMap<(&'a str, &'a str), usize>;

pub struct NetvarManager<'a> {
  pub offsets: Offsets<'a>,
}

impl NetvarManager<'_> {
  pub fn get() -> &'static Self {
    static NETVAR_MANAGER: LazyLock<NetvarManager> = LazyLock::new(NetvarManager::precache);
    &NETVAR_MANAGER
  }

  fn precache() -> Self {
    let mut offsets = Offsets::new();

    let mut client_class = Interfaces::get().client.all_classes();

    while let Some(cc) = client_class {
      unsafe { walk_table(&mut offsets, cc.name, cc.recv_table) };
      client_class = cc.next;
    }
    Self { offsets }
  }
}

unsafe fn walk_table(offsets: &mut Offsets, class_name: *const c_char, table: &RecvTable) {
  for i in 0..table.len as usize {
    let prop = &*table.props.add(i);
    if (*prop.name as u8).is_ascii_digit() {
      continue;
    }

    let prop_name = CStr::from_ptr(prop.name);
    if prop_name == c"baseclass" {
      continue;
    }

    if let Some(t) = prop.table
      && *t.name == b'D' as c_char
      && prop.kind == SendPropKind::NumSendPropKinds
    {
      walk_table(offsets, class_name, t);
    }

    let class_name = CStr::from_ptr(class_name).to_str().unwrap();
    let prop_name = prop_name.to_str().unwrap();

    offsets.insert((class_name, prop_name), prop.offset as usize);
  }
}
