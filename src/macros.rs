macro_rules! p_to_cs {
    ($path:expr) => {{
        std::ffi::CString::new($path.as_ref().as_os_str().as_encoded_bytes())
            .expect("Failed to create `CString` from path")
            .into_raw()
    }};
}

macro_rules! s_to_cs {
    ($str:expr) => {{
        std::ffi::CString::new($str)
            .expect("Failed to create `CString` from string")
            .into_raw()
    }};
}

macro_rules! call_vmethod {
    ($base:expr, $ret_type:ty, $idx:literal, $args:tt, $args_raw:tt) => {
        #[allow(unused_unsafe, clippy::useless_transmute)]
        unsafe { (*(*std::mem::transmute::<_, *const *const extern "thiscall" fn $args -> $ret_type>($base))
            .add($idx)) $args_raw }
    };
}

pub(crate) use call_vmethod;
pub(crate) use p_to_cs;
pub(crate) use s_to_cs;
