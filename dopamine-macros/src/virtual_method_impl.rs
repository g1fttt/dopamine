use proc_macro::TokenStream;
use quote::quote;

use syn::__private::{parse_brackets, parse_parens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Colon, Comma, Fn, Where};
use syn::{Result as SynResult, *};

use std::ops::Not;

struct VirtualMethodParam {
  expr: Expr,
  expr_type: Type,
}

impl Parse for VirtualMethodParam {
  fn parse(input: ParseStream) -> SynResult<Self> {
    let expr = input.parse()?;
    input.parse::<Colon>()?;
    let expr_type = input.parse()?;

    Ok(Self { expr, expr_type })
  }
}

struct VirtualMethodParams(Punctuated<VirtualMethodParam, Comma>);

impl Parse for VirtualMethodParams {
  fn parse(input: ParseStream) -> SynResult<Self> {
    input.parse::<Where>()?;
    let params = parse_parens(input)
      .map(|p| p.content)
      .and_then(|i| i.parse_terminated(VirtualMethodParam::parse, Comma))?;

    Ok(Self(params))
  }
}

struct VirtualMethod {
  visibility: Option<Visibility>,
  ident: Ident,
  generics: Option<Generics>,
  virtual_index: LitInt,
  args: Punctuated<FnArg, Comma>,
  output: Option<ReturnType>,
  params: Option<VirtualMethodParams>,
}

impl Parse for VirtualMethod {
  fn parse(input: ParseStream) -> SynResult<Self> {
    let visibility = input.parse().ok();
    input.parse::<Fn>()?;
    let ident = input.parse()?;
    let generics = input.parse().ok();
    let virtual_index = parse_brackets(input).map(|b| b.content).and_then(|i| i.parse())?;
    let args = parse_parens(input)
      .map(|p| p.content)
      .and_then(|i| i.parse_terminated(FnArg::parse, Comma))?;
    let output = input.parse().ok();
    let params = input.parse().ok();

    Ok(Self { visibility, ident, generics, virtual_index, args, output, params })
  }
}

pub fn macro_impl(item: TokenStream) -> TokenStream {
  let item = parse_macro_input!(item as VirtualMethod);

  let fn_args = item.args;

  let mut fn_params_names = Vec::new();
  let mut fn_params_types = Vec::new();

  for arg in fn_args.iter().skip(1) {
    let FnArg::Typed(arg) = arg else {
      unreachable!();
    };
    fn_params_names.push(arg.pat.clone());
    fn_params_types.push(arg.ty.clone());
  }

  let fn_params = item.params.map(|p| p.0).unwrap_or_default();

  let mut fn_params_exprs = Vec::new();

  for param in fn_params.iter() {
    fn_params_types.push(Box::new(param.expr_type.clone()));
    fn_params_exprs.push(param.expr.clone());
  }

  let optional_comma = fn_params_names.is_empty().not().then_some(quote! { , });
  let fn_params_exprs = quote! { #optional_comma #(#fn_params_exprs),* };

  let visibility = item.visibility;

  let fn_ident = item.ident;
  let fn_generics = item.generics;
  let fn_output = item.output;

  let fn_virtual_index = item.virtual_index;

  quote! {
    #[allow(clippy::too_many_arguments)]
    #visibility fn #fn_ident #fn_generics(#fn_args) #fn_output {
      unsafe {
        (*(*(self as *const Self as *const *const extern "C" fn(&Self, #(#fn_params_types),*) #fn_output))
          .add(#fn_virtual_index))(self, #(#fn_params_names),* #fn_params_exprs)
      }
    }
  }
  .into()
}
