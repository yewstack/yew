use ssr_router::{App, AppProps, LINK_ENDPOINT};

fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::with_props(AppProps {
        endpoint: LINK_ENDPOINT.into(),
    })
    .hydrate();
}
