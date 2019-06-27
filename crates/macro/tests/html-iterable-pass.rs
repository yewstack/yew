use std::iter;
use yew_macro::{html, test_html, test_html_block};

test_html! { |t1| for iter::empty::<Html<Self>>() }
test_html! { |t2| for Vec::<Html<Self>>::new().into_iter() }
test_html! { |t3| for (0..3).map(|num| { html! { <span>{num}</span> } }) }
test_html! { |t4| for {iter::empty::<Html<Self>>()} }

test_html_block! { |t5|
    let empty: Vec<Html<Self>> = Vec::new();

    html! { for empty.into_iter() }
}

fn main() {}
