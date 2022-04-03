use proc_macro::TokenStream;
use syn::parse_macro_input;

mod agent_fn;
mod station;

use agent_fn::{AgentFn, AgentName};
use station::{station_impl, StationFn};

#[proc_macro_error::proc_macro_error]
#[proc_macro_attribute]
pub fn station(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as AgentFn<StationFn>);
    let attr = parse_macro_input!(attr as AgentName);

    station_impl(attr, item)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
