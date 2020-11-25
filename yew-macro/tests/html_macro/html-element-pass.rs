#![recursion_limit = "768"]
use yew::prelude::*;

fn compile_pass() {
    let onclick = Callback::from(|_: MouseEvent| ());
    let parent_ref = NodeRef::default();

    let dyn_tag = || String::from("test");
    let mut extra_tags_iter = vec!["a", "b"].into_iter();

    html! {
        <div>
            <div data-key="abc"></div>
            <div ref=parent_ref class="parent">
                <span class="child" value="anything"></span>
                <label for="first-name">{"First Name"}</label>
                <input type="text" id="first-name" value="placeholder" />
                <input type="checkbox" checked=true />
                <textarea value="write a story" />
                <select name="status">
                    <option selected=true disabled=false value="">{"Selected"}</option>
                    <option selected=false disabled=true value="">{"Unselected"}</option>
                </select>
            </div>
            <svg width="149" height="147" viewBox="0 0 149 147" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M60.5776 13.8268L51.8673 42.6431L77.7475 37.331L60.5776 13.8268Z" fill="#DEB819"/>
                <path d="M108.361 94.9937L138.708 90.686L115.342 69.8642" stroke="black" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
                <g filter="url(#filter0_d)">
                    <circle cx="75.3326" cy="73.4918" r="55" fill="#FDD630"/>
                    <circle cx="75.3326" cy="73.4918" r="52.5" stroke="black" stroke-width="5"/>
                </g>
                <circle cx="71" cy="99" r="5" fill="white" fill-opacity="0.75" stroke="black" stroke-width="3"/>
                <defs>
                    <filter id="filter0_d" x="16.3326" y="18.4918" width="118" height="118" filterUnits="userSpaceOnUse" color-interpolation-filters="sRGB">
                        <feGaussianBlur stdDeviation="2"/>
                        <feColorMatrix in="SourceAlpha" type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"/>
                    </filter>
                </defs>
            </svg>
            <img class=classes!("avatar", "hidden") src="http://pic.com" />
            <img class="avatar hidden" />
            <button onclick=&onclick onclick=onclick />
            <a href="http://google.com" />
            <custom-tag-a>
                <custom-tag-b />
            </custom-tag-a>
            <@{dyn_tag()}>
                <@{extra_tags_iter.next().unwrap()} class="extra-a"/>
                <@{extra_tags_iter.next().unwrap()} class="extra-b"/>
            </@>

            <@{
                let tag = dyn_tag();
                if tag == "test" {
                    "div"
                } else {
                    "a"
                }
            }/>

            <a href?=Some("http://google.com") media?=Option::<&str>::None />
            <track kind?=Some("subtitles") src?=Option::<&str>::None />
            <track kind?=Some(5) mixed="works" />
            <input value?=Some("value") onblur?=Some(Callback::from(|_| ())) />
        </div>
    };

    let children = vec![
        html! { <span>{ "Hello" }</span> },
        html! { <span>{ "World" }</span> },
    ];
    html! { <div>{children}</div> };
}

fn main() {}
