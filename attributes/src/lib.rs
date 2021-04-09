extern crate proc_macro;

use proc_macro::*;

#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream{
    item
}