use yew_macro::{test_html, test_html_block};

test_html! { |t1| for }
test_html! { |t2| for () }
test_html! { |t3| for {()} }
test_html! { |t4| for Vec::<()>::new().into_iter() }

test_html_block! { |t10|
    let empty = Vec::<()>::new().into_iter();
    html! { for empty }
}

test_html_block! { |t11|
    let empty = Vec::<()>::new();
    html! { for empty.iter() }
}

fn main() {}
