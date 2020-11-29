use yew::prelude::*;
use std::iter;

fn compile_pass() {
    html! { for iter::empty::<Html>() };
    html! { for Vec::<Html>::new() };
    html! { for Vec::<Html>::new().into_iter() };
    html! { for (0..3).map(|num| { html! { <span>{num}</span> } }) };
    html! { for {iter::empty::<Html>()} };

    let empty: Vec<Html> = Vec::new();
    html! { for empty.into_iter() };

    let empty: Vec<Html> = Vec::new();
    html! { for empty };
}

fn main() {}
