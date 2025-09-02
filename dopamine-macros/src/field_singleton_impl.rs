use proc_macro::TokenStream;

use quote::quote;
use syn::*;

pub fn macro_impl(item: TokenStream) -> TokenStream {
  let item = parse_macro_input!(item as DeriveInput);

  let struct_generics = item.generics;

  let Data::Struct(data_struct) = item.data else {
    panic!("Only structs are allowed");
  };

  let Fields::Named(fields) = data_struct.fields else {
    panic!("Only named fields are supported");
  };

  /*
    pub fn client() -> &'static Client<?> {
      Interfaces::get().client
    }
  */
  let mut functions = quote! {};

  for field in fields.named {
    let field_visibility = field.vis;
    let Some(field_name) = field.ident else { unreachable!() };
    let field_type = field.ty;

    functions.extend(quote! {
      #field_visibility fn #field_name #struct_generics() -> #field_type {
        Interfaces::get().#field_name
      }
    });
  }
  functions.into()
}
