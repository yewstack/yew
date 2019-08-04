#[macro_use]
mod helpers;

use std::iter;

pass_helper! {
    html! { for iter::empty::<Html<TestComponent>>() };
    html! { for Vec::<Html<TestComponent>>::new().into_iter() };
    html! { for (0..3).map(|num| { html! { <span>{num}</span> } }) };
    html! { for {iter::empty::<Html<TestComponent>>()} };

    let empty: Vec<Html<TestComponent>> = Vec::new();
    html! { for empty.into_iter() };
}

fn main() {}
