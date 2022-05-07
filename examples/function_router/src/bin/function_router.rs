pub use function_router::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    #[cfg(feature = "csr")]
    yew::Renderer::<App>::new().render();
    #[cfg(not(feature = "csr"))]
    panic!("You must enable the csr feature to run this binary");
}
