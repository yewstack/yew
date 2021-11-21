#[derive(Debug, PartialEq, yew_router::Routable)]
enum RoutesOne {
    #[at("/")]
    #[not_found]
    Home,
    #[at("/404")]
    #[not_found]
    NotFound,
}

#[derive(Debug, PartialEq, yew_router::Routable)]
enum RoutesTwo {
    #[at("/404")]
    #[not_found]
    #[not_found]
    NotFound,
}
fn main() {}
