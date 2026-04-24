#[derive(yew_router::Routable, Debug, Clone, PartialEq)]
enum Routes {
    #[at("/")]
    Home,
    #[at("/posts/:id")]
    Post { id: u32 },
    #[at("/files/*path")]
    File { path: String },
}

fn main() {}
