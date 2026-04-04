use simple_ssr::App;

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_ansi(false) // Only partially supported across browsers
            .without_time() // std::time is not available in browsers
            .with_writer(tracing_web::MakeWebConsoleWriter::new())
            .with_filter(tracing_subscriber::filter::LevelFilter::TRACE);
        use tracing_subscriber::prelude::*;
        tracing_subscriber::registry().with(fmt_layer).init();
    }
    yew::Renderer::<App>::new().hydrate();
}
