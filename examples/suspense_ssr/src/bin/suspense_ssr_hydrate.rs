use suspense_ssr::App;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::prelude::*;
use tracing_web::MakeWebConsoleWriter;

fn main() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .without_time()
        .with_writer(MakeWebConsoleWriter::new())
        .with_filter(
            Targets::new()
                .with_target("yew", tracing::Level::DEBUG)
                .with_default(tracing::Level::TRACE),
        );

    tracing_subscriber::registry().with(fmt_layer).init();

    yew::Renderer::<App>::new().hydrate();
}
