use proc_macro::TokenStream;

use crate::function_component::function_component_handler;

mod function_component;
#[proc_macro_attribute]
pub fn function_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    function_component_handler(attr.into(), item)
}
