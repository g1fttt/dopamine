mod field_singleton_impl;

use proc_macro::TokenStream;

#[proc_macro_derive(FieldSingleton)]
pub fn field_singleton(item: TokenStream) -> TokenStream {
  field_singleton_impl::macro_impl(item)
}
