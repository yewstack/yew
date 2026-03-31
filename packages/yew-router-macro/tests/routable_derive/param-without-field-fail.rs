#[derive(yew_router::Routable, Debug, Clone, PartialEq)]
enum Routes {
    #[at("/")]
    Home,
    #[at("/settings/{*_rest}")]
    Settings,
}

fn main() {}
