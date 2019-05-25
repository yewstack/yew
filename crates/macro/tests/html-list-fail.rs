use yew_macro::test_html;

test_html! { |t1| <> }
test_html! { |t2| </> }
test_html! { |t3| <><> }
test_html! { |t4| </></> }
test_html! { |t5| <><></> }
test_html! { |t6| <></><></> }

fn main() {}
