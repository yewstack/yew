#![no_implicit_prelude]

fn main() {
    ::yew::html! {};
    ::yew::html! { <></> };
    ::yew::html! {
        <>
            <></>
            <></>
        </>
    };
    ::yew::html! {
        <key=::std::string::ToString::to_string("key")>
        </>
    };

    let children = ::std::vec![
        ::yew::html! { <span>{ "Hello" }</span> },
        ::yew::html! { <span>{ "World" }</span> },
    ];
    ::yew::html! { <>{ children }</> };
}
