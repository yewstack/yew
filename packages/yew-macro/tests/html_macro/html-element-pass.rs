#![no_implicit_prelude]

// Shadow primitives
#[allow(non_camel_case_types)]
pub struct bool;
#[allow(non_camel_case_types)]
pub struct char;
#[allow(non_camel_case_types)]
pub struct f32;
#[allow(non_camel_case_types)]
pub struct f64;
#[allow(non_camel_case_types)]
pub struct i128;
#[allow(non_camel_case_types)]
pub struct i16;
#[allow(non_camel_case_types)]
pub struct i32;
#[allow(non_camel_case_types)]
pub struct i64;
#[allow(non_camel_case_types)]
pub struct i8;
#[allow(non_camel_case_types)]
pub struct isize;
#[allow(non_camel_case_types)]
pub struct str;
#[allow(non_camel_case_types)]
pub struct u128;
#[allow(non_camel_case_types)]
pub struct u16;
#[allow(non_camel_case_types)]
pub struct u32;
#[allow(non_camel_case_types)]
pub struct u64;
#[allow(non_camel_case_types)]
pub struct u8;
#[allow(non_camel_case_types)]
pub struct usize;

fn compile_pass() {
    let onclick = <::yew::Callback<::yew::events::MouseEvent> as ::std::convert::From<_>>::from(
        |_: ::yew::events::MouseEvent| (),
    );
    let parent_ref = <::yew::NodeRef as ::std::default::Default>::default();

    let dyn_tag =
        || <::std::string::String as ::std::convert::From<&::std::primitive::str>>::from("test");
    let mut extra_tags_iter = ::std::iter::IntoIterator::into_iter(::std::vec!["a", "b"]);

    let attr_val_none: ::std::option::Option<::yew::virtual_dom::AttrValue> = ::std::option::Option::None;

    _ = ::yew::html! {
        <div>
            <div data-key="abc"></div>
            <div ref={&parent_ref}></div>
            <div ref={parent_ref} class="parent">
                <span class="child" value="anything"></span>
                <label for="first-name">{"First Name"}</label>
                <input type="text" id="first-name" value="placeholder" />
                <input type="checkbox" checked=true />
                <textarea value="write a story" />
                <select name="status">
                    <option selected=true disabled=false value="">{"Selected"}</option>
                    <option selected=false disabled=true value="">{"Unselected"}</option>
                </select>
                <video autoplay=true controls=true />
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
            <img class={::yew::classes!("avatar", "hidden")} src="http://pic.com" />
            <img class="avatar hidden" />
            <button onclick={&onclick} {onclick} />
            <a href="http://google.com" />
            <custom-tag-a>
                <custom-tag-b />
            </custom-tag-a>
            <@{dyn_tag()}>
                <@{::std::iter::Iterator::next(&mut extra_tags_iter).unwrap()} class="extra-a"/>
                <@{::std::iter::Iterator::next(&mut extra_tags_iter).unwrap()} class="extra-b"/>
            </@>

            <@{
                if dyn_tag() == "test" {
                    "div"
                } else {
                    "a"
                }
            }/>

            <a href={::std::option::Option::Some(::yew::virtual_dom::AttrValue::Static("http://google.com"))} media={::std::clone::Clone::clone(&attr_val_none)} />
            <track kind={::std::option::Option::Some(::yew::virtual_dom::AttrValue::Static("subtitles"))} src={::std::clone::Clone::clone(&attr_val_none)} />
            <track kind={::std::option::Option::Some(::yew::virtual_dom::AttrValue::Static("5"))} mixed="works" />
            <input value={::std::option::Option::Some(::yew::virtual_dom::AttrValue::Static("value"))}
                onblur={::std::option::Option::Some(<::yew::Callback<::yew::FocusEvent> as ::std::convert::From<_>>::from(|_| ()))}
            />
        </div>
    };

    let children = ::std::vec![
        ::yew::html! { <span>{ "Hello" }</span> },
        ::yew::html! { <span>{ "World" }</span> },
    ];
    _ = ::yew::html! { <div>{children}</div> };

    // handle misleading angle brackets
    _ = ::yew::html! { <div data-val={<::std::string::String as ::std::default::Default>::default()}></div> };
    _ = ::yew::html! { <div><a data-val={<::std::string::String as ::std::default::Default>::default()} /></div> };

    // test for https://github.com/yewstack/yew/issues/2810
    _ = ::yew::html! {  <div data-type="date" data-as="calender" /> };

    let option_vnode = ::std::option::Option::Some(::yew::html! {});
    _ = ::yew::html! { <div>{option_vnode}</div> };
}

fn main() {}
