use std::ffi::c_char;

#[derive(PartialEq)]
#[repr(C)]
pub enum SendPropKind {
    NumSendPropKinds = 6,
}

#[repr(C)]
pub struct RecvProp {
    pub name: *const c_char,
    pub kind: SendPropKind,
    pad1: [u8; 29],
    pub table: Option<&'static RecvTable>,
    pub offset: i32,
    pad2: [u8; 12],
}

#[repr(C)]
pub struct RecvTable {
    pub props: *const RecvProp,
    pub len: i32,
    pad: [u8; 4],
    pub name: *const c_char,
}
