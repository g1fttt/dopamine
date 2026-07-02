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
/// Consider this macro as potentially unsafe, because the string might not contain null-terminator.
///
/// # Examples
///
/// ```
/// #[inline]
/// pub fn name(&self) -> &str {
///     unsafe { rstr!(self.name) }
/// }
/// ```
#[macro_export]
macro_rules! rstr {
  ($ptr:expr) => {
    std::ffi::CStr::from_ptr($ptr).to_str().unwrap()
  };
}

/// Generates a wrapper function for a function with specified index in underlying v-table.
///
/// # Examples
///
/// ```ignore
/// impl MaterialSystem {
///     virtual_method!(
///         pub fn create_material_raw<'a>[70](&self, name: &CStr, kv: &KeyValues) -> Option<&'a Material>,
///     );
///     virtual_method!(
///         fn find_texture_raw[79](&self, name: &CStr, group: &CStr) -> Option<&Texture>
///         // You could also provide additional trailing arguments to the generated function call.
///         // Unfortunately, it requires this kind of ugly syntax.
///         where (bool: true, i32: 0),
///     );
/// }
/// ```
#[macro_export]
macro_rules! virtual_method {
  (
    $visibility:vis fn $fn_ident:ident
    // this monstrosity is just for generics
    $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)?
    // Function index in the virtual table
    [$virtual_index:literal]
    // function arguments and optional return type
    (&self $(, $param:ident : $param_ty:ty )* $(,)? ) $(-> $fn_return:ty)?
    // extra trailing arguments not listed in generated wrapper function
    $(where ($($extra_arg_ty:ty: $extra_arg:expr),+) )?
    // nice to have
    $(,)?
  ) => {
    #[allow(clippy::too_many_arguments, clippy::macro_metavars_in_unsafe)]
    $visibility fn $fn_ident $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? (&self $(, $param: $param_ty)*) $(-> $fn_return)? {
      let this = self as *const Self;
      let vtab = this as *const *const extern "fastcall" fn(&Self $(, $param_ty)* $($(, $extra_arg_ty)*)?) $(-> $fn_return)?;
      unsafe { (*(*vtab).add($virtual_index))(self $(, $param)* $($(, $extra_arg)*)?) }
    }
  };
}
