//! macros — proc-macros placeholder
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(TraceSchema)]
pub fn derive_trace_schema(input: TokenStream) -> TokenStream {
    let _input = parse_macro_input!(input as DeriveInput);
    quote!(/* skeleton */).into()
}
