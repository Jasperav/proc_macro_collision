extern crate proc_macro;
extern crate syn;

use syn::parse_macro_input;
use proc_macro_hack::proc_macro_hack;

use quote::quote;
use parser::Parser;

use proc_macro::TokenStream;

#[proc_macro_hack]
pub fn two(input: TokenStream) -> TokenStream {
    let parse = parse_macro_input!(input as Parser);
    let r = parse.lit;

    // TODO: This is a lot of work just to get the str out of a LitStr, isn't there a better way?
    let query_raw = match r {
        syn::Lit::Str(s) => s,
        _ => panic!()
    };
    let stringified = format!("{:?}", query_raw);
    let starting_quote = stringified.find("\"").expect("Failed to find leading quote");
    let ending_quote = stringified.rfind("\"").expect("Failed to find trailing quote");
    let pretty = stringified[starting_quote + 1..ending_quote].to_string();

    let x = quote! {
        #pretty
    };

    x.into()
}