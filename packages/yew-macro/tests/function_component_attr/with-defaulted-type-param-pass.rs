#![no_implicit_prelude]

#[derive(::yew::prelude::Properties, ::std::fmt::Debug)]
pub struct CompProps<A> {
    #[prop_or_default]
    _phantom: ::std::marker::PhantomData<A>,
}

impl<A> ::std::cmp::PartialEq for CompProps<A> {
    fn eq(&self, _rhs: &Self) -> bool {
        true
    }
}

#[::yew::prelude::function_component(Comp)]
pub fn comp<A = ()>(_props: &CompProps<A>) -> ::yew::prelude::Html {
    ::std::todo!()
}

#[::yew::prelude::function_component(App)]
pub fn app() -> ::yew::prelude::Html {
    ::yew::prelude::html! { <Comp /> } // No generics here.
}

fn main() {}
