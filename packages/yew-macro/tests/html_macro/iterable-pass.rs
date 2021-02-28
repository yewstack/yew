#![no_implicit_prelude]

fn empty_vec() -> ::std::vec::Vec<::yew::Html> {
    ::std::vec::Vec::<::yew::Html>::new()
}

fn empty_iter() -> impl ::std::iter::Iterator<Item = ::yew::Html> {
    ::std::iter::empty::<::yew::Html>()
}

fn main() {
    ::yew::html! { for empty_iter() };
    ::yew::html! { for { empty_iter() } };

    let empty = empty_vec();
    ::yew::html! { for empty };

    ::yew::html! { for empty_vec() };
    ::yew::html! { for ::std::iter::IntoIterator::into_iter(empty_vec()) };
    ::yew::html! { for ::std::iter::Iterator::map(0..3, |num| { ::yew::html! { <span>{ num }</span> } }) };
}
