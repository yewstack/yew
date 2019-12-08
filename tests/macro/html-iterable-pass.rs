#[macro_use]
mod helpers;

use std::iter;

pass_helper! {
    html! { for iter::empty::<Html>() };
    html! { for Vec::<Html>::new().into_iter() };
    html! { for (0..3).map(|num| { html! { <span>{num}</span> } }) };
    html! { for {iter::empty::<Html>()} };

    let empty: Vec<Html> = Vec::new();
    html! { for empty.into_iter() };
}

fn main() {}
