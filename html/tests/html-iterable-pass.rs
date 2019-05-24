use yew_html::{test_html, test_html_block};

test_html_block! { |t1|
    let empty: Vec<Html<Self>> = Vec::new();

    html! { for empty.into_iter() }
}

test_html! { |t2| for (0..3).map(|num| { html! { <span>{num}</span> } }) }

fn main() {}
