#![no_implicit_prelude]

fn main() {
    ::yew::html! { <>{ "Hi" }</> };
    ::yew::html! { <>{ ::std::format!("Hello") }</> };
    ::yew::html! { <>{ ::std::string::ToString::to_string("Hello") }</> };

    let msg = "Hello";
    ::yew::html! { <div>{ msg }</div> };

    let subview = ::yew::html! { "subview!" };
    ::yew::html! { <div>{ subview }</div> };

    let subview = || ::yew::html! { "subview!" };
    ::yew::html! { <div>{ subview() }</div> };

    ::yew::html! {
        <ul>
            { for ::std::iter::Iterator::map(0..3, |num| { ::yew::html! { <span>{ num }</span> }}) }
        </ul>
    };

    let item = |num| ::yew::html! { <li>{ ::std::format!("item {}!", num) }</li> };
    ::yew::html! {
        <ul>
            { for ::std::iter::Iterator::map(0..3, item) }
        </ul>
    };
}
