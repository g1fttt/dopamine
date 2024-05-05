use crate::game::{RecvTable, SendPropKind};
use crate::interfaces::Interfaces;

use std::collections::HashMap;
use std::ffi::{c_char, CStr};

#[derive(Default)]
pub struct NetvarManager {
    pub offsets: HashMap<(&'static str, &'static str), usize>,
}

impl NetvarManager {
    pub unsafe fn precache(interfaces: &Interfaces) -> Self {
        let mut this = Self::default();

        let mut client_class = interfaces.client.all_classes();
        while let Some(cc) = client_class {
            this.walk_table(cc.name, cc.recv_table);
            client_class = cc.next;
        }

        this.offsets.shrink_to_fit();
        this
    }

    unsafe fn walk_table(&mut self, name: *const c_char, table: &RecvTable) {
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
                self.walk_table(name, t);
            }

            let class_name = CStr::from_ptr(name).to_str().unwrap_unchecked();
            let prop_name = prop_name.to_str().unwrap_unchecked();

            self.offsets
                .insert((class_name, prop_name), prop.offset as usize);
        }
    }
}
