#![no_implicit_prelude]

#[derive(
    ::std::prelude::rust_2021::Debug,
    ::std::prelude::rust_2021::PartialEq,
    ::std::prelude::rust_2021::Clone,
)]
struct Ctx;

#[::yew::prelude::hook]
fn use_some_html() -> ::yew::prelude::Html {
    ::yew::prelude::use_context::<Ctx>().unwrap();

    if let ::std::prelude::rust_2021::Some(_m) = ::yew::prelude::use_context::<Ctx>() {
        ::std::todo!()
    }

    let _ctx = { ::yew::prelude::use_context::<Ctx>() };

    match ::yew::prelude::use_context::<Ctx>() {
        ::std::prelude::rust_2021::Some(_) => {
            ::std::todo!()
        }
        ::std::prelude::rust_2021::None => {
            ::std::todo!()
        }
    }
}

fn main() {}
