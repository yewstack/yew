use yew_macro::{test_html, test_html_block};

test_html! { |t1| for }
test_html! { |t2| for () }

// unsupported
test_html_block! { |t3|
    let empty: Vec<Html<Self>> = Vec::new();

    html! { for empty.iter() }
}

fn main() {}
