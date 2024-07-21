use proc_macro::TokenStream;
use quote::quote;

use syn::parse::{Parse, ParseStream};
use syn::token::{Bracket, Fn, For};
use syn::{Result as SynResult, *};

#[allow(dead_code)]
struct FieldIndex {
  bracket_token: Bracket,
  index: LitInt,
}

impl Parse for FieldIndex {
  fn parse(input: ParseStream) -> SynResult<Self> {
    let index;

    Ok(Self {
      bracket_token: bracketed!(index in input),
      index: index.parse()?,
    })
  }
}

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
  field_index: Option<FieldIndex>,
}

impl Parse for Netvar {
  fn parse(input: ParseStream) -> SynResult<Self> {
    Ok(Self {
      vis_token: input.parse().ok(),
      fn_token: input.parse()?,
      name: input.parse()?,
      output: input.parse().ok(),
      for_token: input.parse()?,
      class: input.parse()?,
      arrow_token: input.parse()?,
      field: input.parse()?,
      field_index: input.parse().ok(),
    })
  }
}

pub fn macro_impl(item: TokenStream) -> TokenStream {
  let item = parse_macro_input!(item as Netvar);

  let fn_name = item.name;
  let fn_output = item.output;

  let prop_class = item.class;
  let prop_field = item.field;

  let prop_field_index = item.field_index.map(|field| {
    let index = field.index;
    quote! { concat!('[', #index, ']') }
  });

  quote! {
    pub fn #fn_name(&self) #fn_output {
      let offset = crate::netvar_manager::NetvarManager::get()
        .offsets
        .get(&(stringify!(#prop_class), concat!(stringify!(#prop_field), #prop_field_index)))
        .cloned()
        .expect("Failed to find netvar");
      unsafe { *(self as *const Self).byte_add(offset).cast() }
    }
  }
  .into()
}
