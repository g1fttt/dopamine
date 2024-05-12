use crate::{cstr, App};

#[repr(C)]
pub struct KeyValues {
    pad: [u8; 40],
}

impl KeyValues {
    pub fn new_boxed(shader: &str) -> Box<Self> {
        let mut this = Box::new_uninit();
        (App::patterns().key_values_new)(this.as_mut_ptr(), cstr!(shader));
        unsafe { this.assume_init() }
    }

    pub fn set_string(&mut self, key: &str, value: &str) {
        (App::patterns().key_values_set_string)(self, cstr!(key), cstr!(value));
    }
}
