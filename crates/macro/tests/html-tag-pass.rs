#![recursion_limit = "128"]

use yew_macro::test_html;

test_html! { |t1|
    <div>
        <div></div>
        <div class="parent">
            <span class="child",></span>
            <input type="text" />
        </div>
        <img class=("avatar", "hidden") src="http://pic.com" />
        <img class="avatar", class="hidden", />
    </div>
}

fn main() {}
