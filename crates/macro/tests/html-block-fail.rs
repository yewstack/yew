use yew_macro::{html, test_html, test_html_block};

test_html! { |t1|
    { () }
}

test_html_block! { |t2|
    let not_tree = || ();

    html! {
        <div>{ not_tree() }</div>
    }
}

test_html_block! { |t3|
    let not_tree = || ();

    html! {
        <>{ for (0..3).map(|_| not_tree()) }</>
    }
}

fn main() {}
