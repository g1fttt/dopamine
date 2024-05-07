use proc_macro::TokenStream;
use quote::{quote, quote_spanned};

use syn::spanned::Spanned;
use syn::{parse_macro_input, FnArg, TraitItemFn};

use darling::ast::NestedMeta;
use darling::FromMeta;

#[derive(FromMeta)]
struct VirtualMethodArgs {
    index: usize,
}

#[proc_macro_attribute]
pub fn virtual_method(args: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(val) => val,
        Err(err) => return From::from(darling::Error::from(err).write_errors()),
    };

    let attr_args = match VirtualMethodArgs::from_list(&attr_args) {
        Ok(val) => val,
        Err(err) => return From::from(err.write_errors()),
    };

    let item = parse_macro_input!(item as TraitItemFn);

    if let Some(default) = item.default {
        return quote_spanned! {
            default.span() => compile_error!("Virtual methods cannot be declared with default body");
        }
        .into();
    }

    let fn_sign = item.sig;
    let fn_args = fn_sign.inputs;

    if fn_args
        .first()
        .is_some_and(|arg| !matches!(arg, FnArg::Receiver(_)))
        || fn_args.is_empty()
    {
        return quote_spanned! {
            fn_sign.paren_token.span => compile_error!("Virtual methods cannot be static");
        }
        .into();
    }

    let mut fn_args_names = Vec::new();
    let mut fn_args_types = Vec::new();

    for arg in fn_args.iter().skip(1) {
        let FnArg::Typed(arg) = arg else {
            unreachable!();
        };
        fn_args_names.push(arg.pat.clone());
        fn_args_types.push(arg.ty.clone());
    }

    let fn_ident = fn_sign.ident;
    let fn_ret_type = fn_sign.output;

    let vtable_index = attr_args.index;

    quote! {
        pub fn #fn_ident(#fn_args) #fn_ret_type {
            #[allow(clippy::useless_transmute)]
            unsafe {
                (*(*std::mem::transmute::<_, *const *const extern "thiscall" fn (&Self, #(#fn_args_types),*) #fn_ret_type>(self))
                    .add(#vtable_index))(self, #(#fn_args_names),*)
            }
        }
    }
    .into()
}
