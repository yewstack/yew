use yew_macro::test_html;

test_html! { |t1| "valid" "invalid" }
test_html! { |t2| <span>{ "valid" "invalid" }</span> }

// unsupported literals
test_html! { |t10| b'a' }
test_html! { |t11| b"str" }
test_html! { |t12| 1111111111111111111111111111111111111111111111111111111111111111111111111111 }
test_html! { |t13| <span>{ b'a' }</span> }
test_html! { |t14| <span>{ b"str" }</span> }
test_html! { |t15| <span>{ 1111111111111111111111111111111111111111111111111111111111111111111111111111 }</span> }

fn main() {}
