mod netvar_impl;
mod virtual_method_impl;

use proc_macro::TokenStream;

#[proc_macro]
pub fn netvar(item: TokenStream) -> TokenStream {
  netvar_impl::macro_impl(item)
}

#[proc_macro]
pub fn virtual_method(item: TokenStream) -> TokenStream {
  virtual_method_impl::macro_impl(item)
}
