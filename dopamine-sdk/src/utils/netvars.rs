use crate::game::{RecvTable, SendPropKind};
use crate::utils::Interfaces;

use std::collections::HashMap;
use std::sync::LazyLock;

pub type Offsets<'a> = HashMap<(&'a str, &'a str), usize>;

pub struct Netvars<'a> {
  pub offsets: Offsets<'a>,
}

impl Netvars<'_> {
  pub fn get() -> &'static Self {
    static NETVAR_MANAGER: LazyLock<Netvars> = LazyLock::new(Netvars::precache);
    &NETVAR_MANAGER
  }

  fn precache() -> Self {
    let mut offsets = Offsets::new();

    let mut client_class = Interfaces::get().client.all_classes();

    while let Some(cc) = client_class {
      walk_table(&mut offsets, cc.name(), cc.recv_table);
      client_class = cc.next;
    }
    Self { offsets }
  }
}

fn walk_table<'a>(offsets: &mut Offsets<'a>, class_name: &'a str, table: &'a RecvTable) {
  for i in 0..table.len as usize {
    let prop = unsafe { &*table.props.add(i) };

    let prop_name = prop.name();
    if prop_name.as_bytes()[0].is_ascii_digit() || prop_name == "baseclass" {
      continue;
    }

    if let Some(t) = prop.table
      && t.name().starts_with('D')
      && prop.kind == SendPropKind::NumSendPropKinds
    {
      walk_table(offsets, class_name, t);
    }

    // TODO: Dump netvars in Debug mode

    offsets.insert((class_name, prop_name), prop.offset as usize);
  }
}
