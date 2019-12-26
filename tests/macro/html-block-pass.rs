use yew::prelude::*;

fn compile_pass() {
    html! { <>{ "Hi" }</> };
    html! { <>{ format!("Hello") }</> };
    html! { <>{ String::from("Hello") }</> };

    let msg = "Hello";
    html! { <div>{ msg }</div> };

    let subview = html! { "subview!" };
    html! { <div>{ subview }</div> };

    let subview = || html! { "subview!" };
    html! { <div>{ subview() }</div> };

    html! {
        <ul>
            { for (0..3).map(|num| { html! { <span>{num}</span> }}) }
        </ul>
    };

    let item = |num| html! { <li>{format!("item {}!", num)}</li> };
    html! {
        <ul>
            { for (0..3).map(item) }
        </ul>
    };
}

fn main() {}
