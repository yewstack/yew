#![no_implicit_prelude]

#[derive(Debug, PartialEq, Clone, ::yew_router::Routable)]
enum Routes {
    #[at("/")]
    One,
    #[at("/two/:id")]
    Two { id: u32 },
    #[at("/:a/:b/*rest")]
    Three { a: u32, b: u32, rest: ::std::string::String },
    #[at("/404")]
    #[not_found]
    NotFound,
}

#[derive(Debug, PartialEq, Clone, ::yew_router::Routable)]
enum MoreRoutes {
    #[at("/subpath/*rest")]
    Subpath { rest: ::std::string::String },
    #[at("/*all")]
    CatchAll { all: ::std::string::String },
}

fn main() {}
