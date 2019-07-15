#[macro_use]
mod helpers;

use std::iter;

pass_helper! {
    html! { for iter::empty::<Html<Self>>() };
    html! { for Vec::<Html<Self>>::new().into_iter() };
    html! { for (0..3).map(|num| { html! { <span>{num}</span> } }) };
    html! { for {iter::empty::<Html<Self>>()} };

    let empty: Vec<Html<Self>> = Vec::new();
    html! { for empty.into_iter() };
}

fn main() {}
