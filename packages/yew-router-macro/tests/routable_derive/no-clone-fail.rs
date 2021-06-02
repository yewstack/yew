#![no_implicit_prelude]

#[derive(::yew_router::Routable)]
enum Routes {
    #[at("/")]
    One,
    #[at("/two/:id")]
    Two {
        id: u32,
        #[bind(query)]
        query: u32,
    },
    #[at("/404")]
    #[bind(not_found)]
    NotFound,
}

fn main() {}
