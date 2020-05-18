extern crate proc_macro;
extern crate syn;

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::parse_quote::ParseQuote;

#[derive(Debug)]
pub struct Parser {
    pub lit: syn::Lit,
}

impl Parse for Parser {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lit: syn::Lit = syn::parse::Parse::parse(input)?;

        Ok(Parser {
            lit
        })
    }
}

