#![recursion_limit = "256"]

use yew_macro::{html, test_html};

test_html! { |t1|
    <div>
        <div></div>
        <div class="parent">
            <span class="child", value="anything",></span>
            <input type="text" value="placeholder" />
            <input type="checkbox" checked=true />
            <textarea value="write a story" />
            <select name="status">
                <option selected=true disabled=false value="">{"Selected"}</option>
                <option selected=false disabled=true value="">{"Unselected"}</option>
            </select>
        </div>
        <img class=("avatar", "hidden") src="http://pic.com" />
        <img class="avatar hidden", />
        <button onclick=|e| panic!(e) />
        <a href="http://google.com" />
    </div>
}

fn main() {}
