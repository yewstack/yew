use yew_macro::test_html;

test_html! { |t1|
    <div></div>
}

test_html! { |t2|
    <div>
        <div></div>
        <div></div>
    </div>
}

test_html! { |t3|
    <img />
}

fn main() {}
