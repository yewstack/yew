#[derive(Clone, yew_router::Routable)]
enum Routes {
    One,
}

#[derive(Clone, yew_router::Routable)]
enum RoutesTwo {
    #[at("/one")]
    #[at("/two")]
    One,
}

fn main() {}
