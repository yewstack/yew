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

test_html! { |t20| <input attr=1 attr=2 /> }
test_html! { |t21| <input value="123" value="456" /> }
test_html! { |t22| <input kind="checkbox" kind="submit" /> }
test_html! { |t23| <input checked=true checked=false /> }

fn main() {}
