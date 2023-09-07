extern crate proc_macro;

use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn function_to_string(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // parse input function
    let input_fn = parse_macro_input!(item as ItemFn);
    // create string representation of function
    let function_str = format!("{}", input_fn.to_token_stream());

    // define new function with the same signature as the input function
    let fn_ident = input_fn.sig.ident;
    let fn_inputs = input_fn.sig.inputs;
    let fn_generics = input_fn.sig.generics;

    // construct output
    let output = quote! {

        pub fn #fn_ident #fn_generics (#fn_inputs) -> &'static str {
            #function_str
        }
    };

    output.into()
}
