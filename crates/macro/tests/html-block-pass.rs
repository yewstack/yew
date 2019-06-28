use yew_macro::{html, test_html, test_html_block};

test_html! { |t1| <>{ "Hi" }</> }
test_html! { |t2| <>{ format!("Hello") }</> }
test_html! { |t3| <>{ String::from("Hello") }</> }

test_html_block! { |t10|
    let msg = "Hello";

    html! {
        <div>{ msg }</div>
    }
}

test_html_block! { |t11|
    let subview = html! { "subview!" };

    html! {
        <div>{ subview }</div>
    }
}

test_html_block! { |t12|
    let subview = || html! { "subview!" };

    html! {
        <div>{ subview() }</div>
    }
}

test_html! { |t20|
    <ul>
        { for (0..3).map(|num| { html! { <span>{num}</span> }}) }
    </ul>
}

test_html_block! { |t21|
    let item = |num| html! { <li>{format!("item {}!", num)}</li> };

    html! {
        <ul>
            { for (0..3).map(item) }
        </ul>
    }
}

fn main() {}
