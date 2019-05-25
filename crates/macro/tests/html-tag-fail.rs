use yew_macro::test_html;

test_html! { |t1| <div> }
test_html! { |t2| <div><div> }
test_html! { |t3| </div> }
test_html! { |t4| <div><div></div> }
test_html! { |t5| <div></div><div></div> }
test_html! { |t6| <div></span> }
test_html! { |t7| <div></span></div> }
test_html! { |t8| <img /></img> }
test_html! { |t9| <div>Invalid</div> }

fn main() {}
