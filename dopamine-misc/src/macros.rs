/// Converts any rust string into winapi-compatible null-terminated `PCSTR`.
///
/// Empty call will hand nullified _(invalid)_ `PCSTR`.
///
/// Performs additional allocation for dynamic strings because of `CString` nature.
///
/// # Examples
///
/// ```
/// fn module_data(module_name: &str) -> WindowsResult<(*mut u8, usize)> {
///     let module = unsafe { GetModuleHandleA(pcstr!(module_name)) };
///     // ...
/// }
/// ```
#[macro_export]
macro_rules! pcstr {
  ($str:literal) => {
    windows::core::PCSTR::from_raw(std::concat!($str, '\0').as_ptr())
  };
  ($str:expr) => {
    windows::core::PCSTR::from_raw(std::ffi::CString::new($str).unwrap().into_raw().cast())
  };
  () => {
    windows::core::PCSTR::from_raw(std::ptr::null())
  };
}

/// Converts any rust string into C-ABI compatible null-terminated string.
///
/// Dynamic strings are the only supported kind of strings.
///
/// # Examples
///
/// ```
/// #[inline]
/// pub fn find_texture(&self, name: &str, group: &str) -> Option<&Texture> {
///     self.find_texture_raw(cstr!(name), cstr!(group))
/// }
/// ```
#[macro_export]
macro_rules! cstr {
  ($str:expr) => {
    std::ffi::CString::new($str).unwrap().into_raw()
  };
}

/// Converts C-ABI compatible null-terminated string into `&str`.
///
/// # Examples
///
/// ```
/// #[inline]
/// pub fn name(&self) -> &str {
///     rstr!(self.name)
/// }
/// ```
#[macro_export]
macro_rules! rstr {
  ($ptr:expr) => {
    unsafe { std::ffi::CStr::from_ptr($ptr) }.to_str().unwrap()
  };
}
