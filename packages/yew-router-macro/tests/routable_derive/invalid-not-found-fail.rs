#[derive(Clone, Debug, PartialEq, yew_router::Routable)]
enum RoutesOne {
    #[at("/")]
    #[bind(not_found)]
    Home,
    #[at("/404")]
    #[bind(not_found)]
    NotFound,
}

#[derive(Clone, Debug, PartialEq, yew_router::Routable)]
enum RoutesTwo {
    #[at("/404")]
    #[bind(not_found)]
    #[bind(not_found)]
    NotFound,
}
fn main() {}
