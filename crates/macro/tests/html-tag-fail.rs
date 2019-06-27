use yew_macro::{html, test_html};

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
test_html! { |t24| <input disabled=true disabled=false /> }
test_html! { |t25| <option selected=true selected=false /> }
test_html! { |t26| <div class="first" class="second" /> }

test_html! { |t30| <input checked=1 /> }
test_html! { |t31| <input disabled=1 /> }
test_html! { |t32| <option selected=1 /> }
test_html! { |t33| <input type=() /> }
test_html! { |t34| <input value=() /> }
test_html! { |t35| <a href=() /> }

test_html! { |t40| <input onclick=1 /> }
test_html! { |t41| <input onclick=|| () /> }
test_html! { |t42| <input onclick=|a, b| () /> }
test_html! { |t43| <input onclick=|a: String| () /> }

fn main() {}
