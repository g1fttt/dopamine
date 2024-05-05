macro_rules! pcstr_path {
    ($path:expr) => {
        windows::core::PCSTR::from_raw(
            std::ffi::CString::new($path.as_ref().as_os_str().as_encoded_bytes())
                .expect("Failed to create `CString` from path")
                .into_raw()
                .cast(),
        )
    };
}

macro_rules! pcstr {
    ($str:expr) => {
        windows::core::PCSTR::from_raw(
            std::ffi::CString::new($str)
                .expect("Failed to create `CString` from string")
                .into_raw()
                .cast(),
        )
    };
    () => {
        windows::core::PCSTR::from_raw(std::ptr::null())
    };
}

macro_rules! cstr {
    ($str:expr) => {
        std::ffi::CString::new($str)
            .expect("Failed to create `CString` from string")
            .into_raw()
    };
}

macro_rules! ok_or_empty_err {
    ($x:expr) => {
        $x.ok_or(windows::core::Error::empty())
    };
}

macro_rules! call_vmethod {
    ($base:expr, $ret_type:ty, $idx:literal, $args:tt, $args_raw:tt) => {
        #[allow(unused_unsafe, clippy::useless_transmute)]
        unsafe { (*(*std::mem::transmute::<_, *const *const extern "thiscall" fn $args -> $ret_type>($base))
            .add($idx)) $args_raw }
    };
}

pub(crate) use call_vmethod;
pub(crate) use cstr;
pub(crate) use ok_or_empty_err;
pub(crate) use pcstr;
pub(crate) use pcstr_path;
