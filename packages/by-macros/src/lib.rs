extern crate proc_macro;

mod api_model;
mod query_display;

use api_model::api_model_impl;
use proc_macro::TokenStream;
use query_display::query_display_impl;

#[proc_macro_derive(QueryDisplay)]
pub fn query_display_derive(input: TokenStream) -> TokenStream {
    query_display_impl(input)
}

#[proc_macro_attribute]
pub fn api_model(attr: TokenStream, item: TokenStream) -> TokenStream {
    api_model_impl(attr.into(), item.into()).into()
}
