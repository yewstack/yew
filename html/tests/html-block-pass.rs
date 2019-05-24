use yew_html::{test_html, test_html_block};

test_html! { |t0|
    <>{ "Hi" }</>
}

test_html_block! { |t1|
    let subview = || html! { "subview!" };

    html! {
        <div>{ subview() }</div>
    }
}

test_html_block! { |t2|
    let subview = html! { "subview!" };

    html! {
        <div>{ subview }</div>
    }
}

test_html_block! { |t3|
    let item = |num| html! { <li>{format!("item {}!", num)}</li> };

    html! {
        <ul>
            { for (0..3).map(item) }
        </ul>
    }
}

test_html! { |t4|
    <ul>
        { for (0..3).map(|num| { html! { <span>{num}</span> }}) }
    </ul>
}

fn main() {}
