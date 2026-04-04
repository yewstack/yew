#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::yew::Properties)]
pub struct TextProps {
    pub children: ::yew::html::ChildrenRenderer<::yew::virtual_dom::VNode>,
}

#[::yew::component]
fn Text(_props: &TextProps) -> ::yew::Html {
    ::yew::html! {}
}

#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::yew::Properties)]
pub struct ExampleProps {
    pub text: ::yew::AttrValue,
}

#[::yew::component]
fn Example(props: &ExampleProps) -> ::yew::Html {
    ::yew::html! {
        <Text>
            {&props.text}
        </Text>
    }
}

fn main() {
    let _ = ::yew::html! { <Text>{"hello"}</Text> };

    let s = ::std::string::String::from("world");
    let _ = ::yew::html! { <Text>{s}</Text> };

    let _ = ::yew::html! { <Text>{ ::std::format!("year {}", 2024) }</Text> };

    let status: ::std::option::Option<::std::string::String> =
        ::std::option::Option::Some("Active".into());
    let _ = ::yew::html! {
        <Text>
            { status.as_ref().map_or_else(
                ::std::string::String::new,
                |s| ::std::format!("Status: {}", s)
            ) }
        </Text>
    };
}
