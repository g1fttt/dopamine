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

/// Used specifically for generating getter functions in the [`crate::interfaces`] module.
///
/// # Examples
///
/// This code:
/// ```ignore
/// singleton_fields! {
///     struct Interfaces<'a> {
///         pub client: &'a Client,
///         pub server: &'a Server,
///     }
/// }
/// ```
/// ... generates this code:
/// ```ignore
/// struct Interfaces<'a> {
///     pub client: &'a Client,
///     pub server: &'a Server,
/// }
///
/// pub fn client<'a>() -> &'a Client {
///   Interfaces::get().client
/// }
///
/// pub fn server<'a>() -> &'a Server {
///   Interfaces::get().server
/// }
/// ```
#[macro_export]
macro_rules! singleton_fields {
  (
    $(#[$attr:meta])*
    $struct_visibility:vis struct $Struct:ident $(< $( $lt:tt $(: $clt:tt $(+ $dlt:tt )* )? ),+ >)? {
      $($field_visibility:vis $field_name:ident: $field_type:ty),*
      $(,)?
    }
  ) => {
    $(#[$attr])*
    $struct_visibility struct $Struct $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? {
      $($field_visibility $field_name: $field_type),*
    }

    $crate::singleton_fields! {
      @munch
      struct_name = $Struct;
      generics = [ $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? ];
      fields = [ $($field_visibility $field_name: $field_type,)* ];
    }
  };

  (
    @munch
    struct_name = $Struct:ident;
    generics = [ $($generics:tt)* ];
    fields = [
      $field_visibility:vis $field_name:ident: $field_type:ty,
      $($rest:tt)*
    ];
  ) => {
    $field_visibility fn $field_name $($generics)* () -> $field_type {
      $Struct::get().$field_name
    }

    $crate::singleton_fields! {
      @munch
      struct_name = $Struct;
      generics = [ $($generics)* ];
      fields = [ $($rest)* ];
    }
  };

  (
    @munch
    struct_name = $Struct:ident;
    generics = [ $($generics:tt)* ];
    fields = [];
  ) => {};
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
      let vtab = this as *const *const extern "C" fn(&Self $(, $param_ty)* $($(, $extra_arg_ty)*)?) $(-> $fn_return)?;
      unsafe { (*(*vtab).add($virtual_index))(self $(, $param)* $($(, $extra_arg)*)?) }
    }
  };
}

/// Generates a wrapper function for accessing specific Source netvar.
///
/// # Examples
///
/// ```ignore
/// struct Entity;
///
/// impl Entity {
///     netvar!(pub fn flags -> EntityFlags as CBasePlayer->m_fFlags);
/// }
/// ```
#[macro_export]
macro_rules! netvar {
  (
    $visibility:vis fn $fn_name:ident $(-> $fn_return:ty)?
    as $class:ident->$field:ident $([$index:literal])?
    $(,)?
  ) => {
    $visibility fn $fn_name(&self) $(-> $fn_return)? {
      const PROP_CLASS: &str = stringify!($class);
      const PROP_FIELD: &str = concat!(stringify!($field), $("[", stringify!($index), "]")?);

      let offset = $crate::utils::Netvars::get()
      .get(&(PROP_CLASS, PROP_FIELD))
      .map(|n| n.offset)
      .unwrap_or_else(|| {
        ::log::error!("Failed to find netvar: {PROP_CLASS}->{PROP_FIELD}");
        panic!();
      });
      unsafe { *(self as *const Self).byte_add(offset).cast() }
    }
  };
}
