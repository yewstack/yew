use proc_macro::TokenStream;
use syn::parse_macro_input;

mod agent_fn;
mod oneshot;
mod reactor;

use agent_fn::{AgentFn, AgentName};
use oneshot::{oneshot_impl, OneshotFn};
use reactor::{reactor_impl, ReactorFn};

#[proc_macro_attribute]
pub fn reactor(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as AgentFn<ReactorFn>);
    let attr = parse_macro_input!(attr as AgentName);

    reactor_impl(attr, item)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn oneshot(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as AgentFn<OneshotFn>);
    let attr = parse_macro_input!(attr as AgentName);

    oneshot_impl(attr, item)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
