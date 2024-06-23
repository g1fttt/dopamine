use darling::FromMeta;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};

use syn::spanned::Spanned;
use syn::{parse_macro_input, TraitItemFn};

#[derive(FromMeta)]
struct AttrArgs {
  path: String,
}

pub fn macro_impl(attr_args: TokenStream, item: TokenStream) -> TokenStream {
  let item = parse_macro_input!(item as TraitItemFn);
  let attr_args = match crate::shared::parse_and_validate::<AttrArgs>(attr_args, item.clone()) {
    Ok(args) => args,
    Err(err) => return err,
  };

  let fn_sign = item.clone().sig;
  let fn_ident = fn_sign.ident;
  let fn_output = fn_sign.output;

  let Some((class_name, prop_name)) = attr_args.path.split_once("->") else {
    return quote_spanned! {
      item.span() => compile_error!("Netvar must be declared using `->`: `Class->prop`"),
    }
    .into();
  };

  quote! {
    pub fn #fn_ident(&self) #fn_output {
      let offset = crate::netvar_manager::NetvarManager::get()
        .offsets
        .get(&(#class_name, #prop_name))
        .cloned()
        .expect("Failed to find netvar");
      unsafe { *(self as *const Self).byte_add(offset).cast() }
    }
  }
  .into()
}
