use proc_macro::TokenStream;
use quote::quote;

use syn::parse::{Parse, ParseStream};
use syn::token::{Fn, For};
use syn::*;

#[allow(dead_code)]
struct Netvar {
  vis_token: Option<Visibility>,
  fn_token: Fn,
  name: Ident,
  output: Option<ReturnType>,
  for_token: For,
  class: Ident,
  arrow_token: Token![->],
  field: Ident,
}

impl Parse for Netvar {
  fn parse(input: ParseStream) -> Result<Self> {
    Ok(Self {
      vis_token: input.parse().ok(),
      fn_token: input.parse()?,
      name: input.parse()?,
      output: input.parse().ok(),
      for_token: input.parse()?,
      class: input.parse()?,
      arrow_token: input.parse()?,
      field: input.parse()?,
    })
  }
}

pub fn macro_impl(item: TokenStream) -> TokenStream {
  let item = parse_macro_input!(item as Netvar);

  let fn_name = item.name;
  let fn_output = item.output;
  let prop_class = item.class;
  let prop_field = item.field;

  quote! {
    pub fn #fn_name(&self) #fn_output {
      let offset = crate::netvar_manager::NetvarManager::get()
        .offsets
        .get(&(stringify!(#prop_class), stringify!(#prop_field)))
        .cloned()
        .expect("Failed to find netvar");
      unsafe { *(self as *const Self).byte_add(offset).cast() }
    }
  }
  .into()
}
