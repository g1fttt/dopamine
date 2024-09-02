use proc_macro::TokenStream;
use quote::quote;

use syn::__private::parse_brackets;
use syn::parse::{Parse, ParseStream};
use syn::token::{Fn, For};
use syn::{Result as SynResult, *};

struct PropField {
  ident: Ident,
  index: Option<LitInt>,
}

impl Parse for PropField {
  fn parse(input: ParseStream) -> SynResult<Self> {
    let ident = input.parse()?;
    let index = parse_brackets(input).map(|b| b.content).and_then(|i| i.parse()).ok();

    Ok(Self { ident, index })
  }
}

struct Netvar {
  visibility: Option<Visibility>,
  ident: Ident,
  output: Option<ReturnType>,
  prop_class: Ident,
  prop_field: PropField,
}

impl Parse for Netvar {
  fn parse(input: ParseStream) -> SynResult<Self> {
    let visibility = input.parse().ok();
    input.parse::<Fn>()?;
    let ident = input.parse()?;
    let output = input.parse().ok();
    input.parse::<For>()?;
    let prop_class = input.parse()?;
    input.parse::<Token![->]>()?;
    let prop_field = input.parse()?;

    Ok(Self { visibility, ident, output, prop_class, prop_field })
  }
}

pub fn macro_impl(item: TokenStream) -> TokenStream {
  let item = parse_macro_input!(item as Netvar);

  let visibility = item.visibility;

  let fn_ident = item.ident;
  let fn_output = item.output;

  let prop_class = item.prop_class;
  let prop_field = item.prop_field;

  let prop_field_ident = prop_field.ident;
  let prop_field_index = prop_field.index.map(|index| {
    quote! { concat!('[', #index, ']') }
  });

  quote! {
    #visibility fn #fn_ident(&self) #fn_output {
      const PROP_CLASS: &str = stringify!(#prop_class);
      const PROP_FIELD: &str = concat!(stringify!(#prop_field_ident), #prop_field_index);

      let offset = crate::netvar_manager::NetvarManager::get()
        .offsets
        .get(&(PROP_CLASS, PROP_FIELD))
        .cloned()
        .unwrap_or_else(|| { log::error!("Failed to find netvar: {}->{}", PROP_CLASS, PROP_FIELD); panic!() });
      unsafe { *(self as *const Self).byte_add(offset).cast() }
    }
  }
  .into()
}
