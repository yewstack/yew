#![no_implicit_prelude]
// without it, there's the following error
// error[E0433]: failed to resolve: use of undeclared crate or module `std`
extern crate std;

#[derive(Debug, PartialEq, ::yew_router::Routable)]
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
