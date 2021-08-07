#![no_implicit_prelude]

#[derive(Debug, PartialEq, Clone, ::yew_router::Routable)]
enum Routes {
    #[at("/")]
    One,
    #[at("/two/:id")]
    Two { id: u32 },
    #[at("/404")]
    #[not_found]
    NotFound,
}

fn main() {}
