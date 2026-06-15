#[derive(yew_router::Routable)]
enum Routes {
    #[at("/foo")]
    Foo,
    #[at("/foo")]
    #[not_found]
    NotFound,
}

fn main() {}
