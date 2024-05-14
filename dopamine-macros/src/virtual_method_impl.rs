use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, TraitItemFn};

#[derive(FromMeta)]
struct AttrArgs {
    index: usize,
    private: Option<bool>,
}

pub fn macro_impl(attr_args: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as TraitItemFn);
    let attr_args = match crate::shared::parse_and_validate::<AttrArgs>(attr_args, item.clone()) {
        Ok(args) => args,
        Err(err) => return err,
    };

    let fn_sign = item.sig;
    let fn_args = fn_sign.inputs;

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
    let fn_generics = fn_sign.generics;
    let fn_output = fn_sign.output;

    let vtable_index = attr_args.index;
    let pub_token = match attr_args.private {
        None | Some(false) => Some(quote! { pub }),
        _ => None,
    };

    quote! {
        #[allow(clippy::too_many_arguments)]
        #pub_token fn #fn_ident #fn_generics(#fn_args) #fn_output {
            unsafe {
                (*(*(self as *const Self as *const *const extern "thiscall" fn(&Self, #(#fn_args_types),*) #fn_output))
                    .add(#vtable_index))(self, #(#fn_args_names),*)
            }
        }
    }
    .into()
}
