use crate::RecvPropProxy;
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
      log::debug!("{} ({}):", cc.name(), cc.id.0);

      walk_table(&mut inner, cc.name(), cc.recv_table);

      client_class = cc.next;
    }
    Self(inner)
  }
}

fn walk_table<'a>(inner: &mut NetvarsInner<'a>, class_name: &'a str, table: &'a RecvTable) {
  for i in 0..table.len as usize {
    let prop = unsafe { &mut *table.props.add(i) };

    let prop_name = prop.name();
    if prop_name.as_bytes()[0].is_ascii_digit() || prop_name == "baseclass" {
      continue;
    }

    if let Some(t) = prop.table
      && t.name().starts_with('D')
      && prop.kind == SendPropKind::NumSendPropKinds
    {
      walk_table(inner, class_name, t);
    }

    log::debug!("\t{class_name}->{prop_name}: 0x{:X}", prop.offset);

    let netvar = Netvar::new(prop.offset as usize, &mut prop.proxy);

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
  pub(crate) proxy: *mut Option<RecvPropProxy>,
}

impl Netvar {
  fn new(offset: usize, proxy: *mut Option<RecvPropProxy>) -> Self {
    Self { offset, proxy }
  }
}

unsafe impl Sync for Netvar {}
unsafe impl Send for Netvar {}
