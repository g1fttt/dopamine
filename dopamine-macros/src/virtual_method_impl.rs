use proc_macro::TokenStream;
use quote::quote;

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Colon, Comma, Fn, Paren, Unsafe};
use syn::*;

use std::ops::Not;

#[allow(dead_code)]
#[derive(Clone)]
struct ColonAndType {
  colon_token: Colon,
  expr_type: Type,
}

impl Parse for ColonAndType {
  fn parse(input: ParseStream) -> Result<Self> {
    Ok(Self {
      colon_token: input.parse()?,
      expr_type: input.parse()?,
    })
  }
}

#[allow(dead_code)]
#[derive(Clone)]
struct VirtualMethodParam {
  expr: Expr,
  colon_and_type: Option<ColonAndType>,
}

impl Parse for VirtualMethodParam {
  fn parse(input: ParseStream) -> Result<Self> {
    Ok(Self {
      expr: input.parse()?,
      colon_and_type: input.parse().ok(),
    })
  }
}

#[allow(dead_code)]
struct FatArrowAndParams {
  fat_arrow_token: Token![=>],
  params_paren_token: Paren,
  params: Punctuated<VirtualMethodParam, Comma>,
}

impl Parse for FatArrowAndParams {
  fn parse(input: ParseStream) -> Result<Self> {
    let params;

    Ok(Self {
      fat_arrow_token: input.parse()?,
      params_paren_token: parenthesized!(params in input),
      params: params.parse_terminated(VirtualMethodParam::parse, Comma)?,
    })
  }
}

#[allow(dead_code)]
struct VirtualMethod {
  vis_token: Option<Visibility>,
  unsafety: Option<Unsafe>,
  fn_token: Fn,
  name: Ident,
  generics: Option<Generics>,
  args_paren_token: Paren,
  args: Punctuated<FnArg, Comma>,
  output: Option<ReturnType>,
  bracket_token: Bracket,
  virtual_index: LitInt,
  fat_arrow_and_params: Option<FatArrowAndParams>,
}

impl Parse for VirtualMethod {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let args;
    let virtual_index;

    Ok(Self {
      vis_token: input.parse().ok(),
      unsafety: input.parse().ok(),
      fn_token: input.parse()?,
      name: input.parse()?,
      generics: input.parse().ok(),
      args_paren_token: parenthesized!(args in input),
      args: args.parse_terminated(FnArg::parse, Comma)?,
      output: input.parse().ok(),
      bracket_token: bracketed!(virtual_index in input),
      virtual_index: virtual_index.parse()?,
      fat_arrow_and_params: input.parse().ok(),
    })
  }
}

pub fn macro_impl(item: TokenStream) -> TokenStream {
  let input = parse_macro_input!(item as VirtualMethod);

  let fn_args = input.args;

  let mut fn_params_names = Vec::new();
  let mut fn_params_types = Vec::new();

  for arg in fn_args.iter().skip(1) {
    let FnArg::Typed(arg) = arg else {
      unreachable!();
    };
    fn_params_names.push(arg.pat.clone());
    fn_params_types.push(arg.ty.clone());
  }

  let fn_params = input
    .fat_arrow_and_params
    .as_ref()
    .map(|x| x.params.clone())
    .unwrap_or_default();

  let mut fn_params_exprs = Vec::new();

  for param in fn_params.iter() {
    let Some(param_type) = param.colon_and_type.as_ref().map(|x| x.expr_type.clone()) else {
      unreachable!();
    };
    fn_params_types.push(Box::new(param_type));
    fn_params_exprs.push(param.expr.clone());
  }

  let optional_comma = fn_params_names.is_empty().not().then_some(quote! { , });
  let fn_params_exprs = quote! { #optional_comma #(#fn_params_exprs),* };

  let vis_token = input.vis_token;
  let unsafety = input.unsafety;

  let fn_name = input.name;
  let fn_generics = input.generics;
  let fn_output = input.output;

  let fn_virtual_index = input.virtual_index;

  quote! {
    #[allow(clippy::too_many_arguments)]
    #vis_token #unsafety fn #fn_name #fn_generics(#fn_args) #fn_output {
      unsafe {
        (*(*(self as *const Self as *const *const extern "thiscall" fn(&Self, #(#fn_params_types),*) #fn_output))
          .add(#fn_virtual_index))(self, #(#fn_params_names),* #fn_params_exprs)
      }
    }
  }
  .into()
}
