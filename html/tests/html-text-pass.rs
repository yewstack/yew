use yew_html::test_html;

test_html! { |t1| "" }
test_html! { |t2| 'a' }
test_html! { |t3| "hello" }
test_html! { |t4| 42 }
test_html! { |t5| 1.234 }
test_html! { |t6| true }

test_html! { |t10| <span>{ "" }</span> }
test_html! { |t11| <span>{ 'a' }</span> }
test_html! { |t12| <span>{ "hello" }</span> }
test_html! { |t13| <span>{ 42 }</span> }
test_html! { |t14| <span>{ 1.234 }</span> }
test_html! { |t15| <span>{ true }</span> }

fn main() {}
