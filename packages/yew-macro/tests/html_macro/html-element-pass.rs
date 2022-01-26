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

    ::yew::html! {
        <div>
            <div class="abc"></div>
            <div ref={parent_ref} class="parent">
                <span class="child"></span>
                <label for="first-name">{"First Name"}</label>
                <input type="text" id="first-name" value="placeholder" />
                <input type="checkbox" checked=true />
                <textarea>{"write a story"}</textarea>
                <select name="status">
                    <option selected=true disabled=false value="">{"Selected"}</option>
                    <option selected=false disabled=true value="">{"Unselected"}</option>
                </select>
                <video autoplay=true controls=true />
            </div>
            <img class={::yew::classes!("avatar", "hidden")} src="http://pic.com" />
            <img class="avatar hidden" />
            <button {onclick} />
            <a href="http://google.com" />
            <@{dyn_tag()}>
                <@{::std::iter::Iterator::next(&mut extra_tags_iter).unwrap()} class="extra-a"/>
                <@{::std::iter::Iterator::next(&mut extra_tags_iter).unwrap()} class="extra-b"/>
            </@>

            <@{
                let tag = dyn_tag();
                if tag == "test" {
                    "div"
                } else {
                    "a"
                }
            }/>

            <a href={::std::option::Option::Some(::yew::virtual_dom::AttrValue::Static("http://google.com"))} media={::std::clone::Clone::clone(&attr_val_none)} />
            <track kind={::std::option::Option::Some(::yew::virtual_dom::AttrValue::Static("subtitles"))} src={::std::clone::Clone::clone(&attr_val_none)} />
            <track kind={::std::option::Option::Some(::yew::virtual_dom::AttrValue::Static("5"))} />
            <input value={::std::option::Option::Some(::yew::virtual_dom::AttrValue::Static("value"))}
                onblur={::std::option::Option::Some(<::yew::Callback<::yew::FocusEvent> as ::std::convert::From<_>>::from(|_| ()))}
            />
        </div>
    };

    let children = ::std::vec![
        ::yew::html! { <span>{ "Hello" }</span> },
        ::yew::html! { <span>{ "World" }</span> },
    ];
    ::yew::html! { <div>{children}</div> };

    // handle misleading angle brackets
    ::yew::html! { <div id={<::std::string::String as ::std::default::Default>::default()}></div> };
    ::yew::html! { <div><a class={<::std::string::String as ::std::default::Default>::default()} /></div> };
}

fn main() {}
