use yew_macro::{html, test_html};

test_html! { |t0| }
test_html! { |t1| <></> }
test_html! { |t2|
    <>
        <></>
        <></>
    </>
}

fn main() {}
