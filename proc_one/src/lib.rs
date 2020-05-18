extern crate proc_macro;
extern crate syn;

use syn::parse_macro_input;
use proc_macro_hack::proc_macro_hack;

use quote::quote;
use parser::Parser;

use proc_macro::TokenStream;

#[proc_macro_hack]
pub fn one(input: TokenStream) -> TokenStream {
    let parse = parse_macro_input!(input as Parser);
    let r = parse.lit;

    let x = quote! {
        two!(#r)
    };

    x.into()
}