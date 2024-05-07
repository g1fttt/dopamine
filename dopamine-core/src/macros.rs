#[macro_export]
macro_rules! pcstr_path {
    ($path:expr) => {
        windows::core::PCSTR::from_raw(
            std::ffi::CString::new($path.as_ref().as_os_str().as_encoded_bytes())
                .expect("Failed to create `CString` from path | `pcstr_path!`")
                .into_raw()
                .cast(),
        )
    };
}

#[macro_export]
macro_rules! pcstr {
    ($str:literal) => {
        windows::core::PCSTR::from_raw(std::concat!($str, '\0').as_ptr())
    };
    ($str:expr) => {
        windows::core::PCSTR::from_raw(
            std::ffi::CString::new($str)
                .expect("Failed to create `CString` from string | `pcstr!`")
                .into_raw()
                .cast(),
        )
    };
    () => {
        windows::core::PCSTR::from_raw(std::ptr::null())
    };
}

#[macro_export]
macro_rules! cstr {
    ($str:expr) => {
        std::ffi::CString::new($str)
            .expect("Failed to create `CString` from string | `cstr!`")
            .into_raw()
    };
}

#[macro_export]
macro_rules! ok_or_empty_err {
    ($x:expr) => {
        $x.ok_or(windows::core::Error::empty())
    };
}

#[macro_export]
macro_rules! get_last_err {
    () => {
        windows::core::Error::from_win32()
    };
}

#[macro_export]
macro_rules! declare_netvar {
    ($ret_type:ty, $name:ident, $class_name:literal, $prop_name:literal) => {
        pub fn $name(&self) -> $ret_type {
            let offset = $crate::App::netvar_offset($class_name, $prop_name).expect(std::concat!(
                "Failed to find ",
                $class_name,
                "->",
                $prop_name,
                " netvar"
            ));
            unsafe { *(self as *const Self).byte_add(offset).cast() }
        }
    };
}
