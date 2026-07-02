use crate::game::{RecvTable, SendPropKind};
use crate::interfaces::client;

use std::collections::HashMap;
use std::ops::Deref;
use std::sync::LazyLock;

pub struct Netvars<'a>(NetvarsInner<'a>);

impl Netvars<'_> {
  pub fn get() -> &'static Self {
    static NETVAR_MANAGER: LazyLock<Netvars> = LazyLock::new(Netvars::precache);
    &NETVAR_MANAGER
  }

  fn precache() -> Self {
    let mut inner = NetvarsInner::new();
    let mut client_class = client().all_classes();

    while let Some(cc) = client_class {
      let cc_name = cc.name();

      log::debug!("{} ({}):", cc_name, cc.id.0);

      walk_table(&mut inner, cc_name, cc.recv_table);

      client_class = cc.next;
    }
    Self(inner)
  }
}

fn walk_table<'a>(inner: &mut NetvarsInner<'a>, class_name: &'a str, table: &'a RecvTable) {
  for prop in table.props() {
    let prop_name = prop.name();
    if prop_name == "baseclass" {
      continue;
    }

    if let Some(table) = prop.table
      && table.name().starts_with("DT_") // Data Table
      && prop.kind == SendPropKind::NumSendPropKinds
    {
      walk_table(inner, class_name, table);
    }

    log::debug!("\t{class_name}->{prop_name}: 0x{:X}", prop.offset);

    let offset = prop.offset as usize;
    let netvar = Netvar { offset };

    inner.insert((class_name, prop_name), netvar);
  }
}

impl<'a> Deref for Netvars<'a> {
  type Target = NetvarsInner<'a>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

pub type NetvarsInner<'a> = HashMap<(&'a str, &'a str), Netvar>;

pub struct Netvar {
  pub offset: usize,
}

unsafe impl Sync for Netvar {}
unsafe impl Send for Netvar {}
