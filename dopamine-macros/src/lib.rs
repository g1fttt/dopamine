mod virtual_method_impl;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn virtual_method(attr_args: TokenStream, item: TokenStream) -> TokenStream {
    virtual_method_impl::macro_impl(attr_args, item)
}
