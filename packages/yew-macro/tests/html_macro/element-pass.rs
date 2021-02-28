#![no_implicit_prelude]

fn main() {
    let onclick: ::yew::Callback<_> = ::std::convert::From::from(|_: ::yew::MouseEvent| ());
    let parent_ref: ::yew::NodeRef = ::std::default::Default::default();

    ::yew::html! {
        <div>
            <div data-key="abc"></div>
            <div ref=parent_ref class="parent">
                <span class="child" value="anything"></span>
                <label for="first-name">{ "First Name" }</label>
                <input type="text" id="first-name" value="placeholder" />
                <input type="checkbox" checked=true />
                <textarea value="write a story" />
                <select name="status">
                    <option selected=true disabled=false value="">{ "Selected" }</option>
                    <option selected=false disabled=true value="">{ "Unselected" }</option>
                </select>
            </div>
            <img class=::yew::classes!("avatar", "hidden") src="http://pic.com" />
            <img class="avatar hidden" />
            <button onclick=&onclick onclick=onclick />
            <a href="http://google.com" />
            <custom-tag-a>
                <custom-tag-b />
            </custom-tag-a>

            <a href?=::std::option::Option::Some("http://google.com") media?=::std::option::Option::<&str>::None />
            <track kind?=::std::option::Option::Some("subtitles") src?=::std::option::Option::<&str>::None />
            <track kind?=::std::option::Option::Some(5) mixed="works" />
            <input value?=::std::option::Option::Some("value") onblur?=::std::option::Option::Some(<::yew::Callback<_> as ::std::convert::From<_>>::from(|_| ())) />
        </div>
    };

    let children = ::std::vec![
        ::yew::html! { <span>{ "Hello" }</span> },
        ::yew::html! { <span>{ "World" }</span> },
    ];
    ::yew::html! { <div>{ children }</div> };
}
