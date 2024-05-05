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
        while !client_class.is_null() {
            this.walk_table((*client_class).name, (*client_class).recv_table, None);
            client_class = (*client_class).next;
        }
        this.offsets.shrink_to_fit();
        this
    }

    unsafe fn walk_table(&mut self, name: *const c_char, table: &RecvTable, offset: Option<usize>) {
        for i in 0..table.len as usize {
            let prop = &*table.props.add(i);
            if (*prop.name as u8).is_ascii_digit() {
                continue;
            }

            let prop_name = CStr::from_ptr(prop.name);
            if prop_name == c"baseclass" {
                continue;
            }

            let final_offset = prop.offset as usize + offset.unwrap_or_default();
            if prop.kind == SendPropKind::NumSendPropKinds
                && !prop.table.is_null()
                && *(*prop.table).name == b'D' as c_char
            {
                self.walk_table(name, &*prop.table, Some(final_offset));
            }

            let class_name = CStr::from_ptr(name).to_str().unwrap_unchecked();
            let prop_name = prop_name.to_str().unwrap_unchecked();

            self.offsets.insert((class_name, prop_name), final_offset);
        }
    }
}
