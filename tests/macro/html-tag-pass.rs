#![recursion_limit = "256"]

#[macro_use]
mod helpers;

pass_helper! {
    html! {
        <div>
            <div data-key="abc"></div>
            <div class="parent">
                <span class="child", value="anything",></span>
                <label for="first-name">{"First Name"}</label>
                <input type="text" id="first-name" value="placeholder" />
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
            <custom-tag>
                <custom-tag />
            </custom-tag>
        </div>
    };
}

fn main() {}
