#[derive(yew_router::Routable)]
enum Routes {
    One,
}

#[derive(yew_router::Routable)]
enum RoutesTwo {
    #[at("/one")]
    #[at("/two")]
    One,
}

fn main() {}
