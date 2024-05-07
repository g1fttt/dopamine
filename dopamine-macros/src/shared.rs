use proc_macro::TokenStream;
use quote::quote_spanned;

use syn::spanned::Spanned;
use syn::{FnArg, TraitItemFn};

use darling::ast::NestedMeta;
use darling::FromMeta;

pub fn parse_and_validate<T: FromMeta>(
    attr_args: TokenStream,
    item: TraitItemFn,
) -> Result<T, TokenStream> {
    let attr_args = match NestedMeta::parse_meta_list(attr_args.into()) {
        Ok(val) => val,
        Err(err) => return Err(From::from(darling::Error::from(err).write_errors())),
    };

    let attr_args = match T::from_list(&attr_args) {
        Ok(val) => val,
        Err(err) => return Err(From::from(err.write_errors())),
    };

    if let Some(default) = item.default {
        return Err(quote_spanned! {
            default.span() => compile_error!("This item cannot be declared with default body");
        }
        .into());
    }

    let fn_sign = item.sig;
    let fn_args = fn_sign.inputs;

    if fn_args
        .first()
        .is_some_and(|arg| !matches!(arg, FnArg::Receiver(_)))
        || fn_args.is_empty()
    {
        return Err(quote_spanned! {
            fn_sign.paren_token.span => compile_error!("This item cannot be static");
        }
        .into());
    }
    Ok(attr_args)
}
