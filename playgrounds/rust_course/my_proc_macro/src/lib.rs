extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Ident, ItemFn};

#[proc_macro]
pub fn create_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Ident);

    let name = &input;
    let expanded = quote! {
        struct #name {
            x: i32,
            y: i32
        }
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn function_to_string(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse input function
    let input_fn = parse_macro_input!(item as ItemFn);
    // create string representation of function

    let name = input.sig.ident;
    let name_str = name.to_string();
    let expanded = quote! {
        #input
        impl #name {
            fn to_string(&self) -> String {
                #name_str.to_string()
            }
        }
    };
    expanded.into()
}
