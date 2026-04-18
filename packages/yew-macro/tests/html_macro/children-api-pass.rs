#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::yew::Properties)]
pub struct ContainerProps {
    #[prop_or_default]
    pub children: ::yew::Html,
}

#[::yew::component]
fn Container(_props: &ContainerProps) -> ::yew::Html {
    ::yew::html! {}
}

#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::yew::Properties)]
pub struct AccordionProps {
    pub children: ::yew::html::ChildrenRenderer<::yew::virtual_dom::VNode>,
}

#[::yew::component]
fn Accordion(props: &AccordionProps) -> ::yew::Html {
    ::yew::html! {
        <div>
            { for props.children.iter().map(|item| ::yew::html!(
                <div class="item">
                    { item }
                </div>
            )) }
        </div>
    }
}

#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::yew::Properties)]
pub struct PanelProps {
    #[prop_or_default]
    pub children: ::yew::html::ChildrenRenderer<::yew::virtual_dom::VNode>,
}

#[::yew::component]
fn Panel(props: &PanelProps) -> ::yew::Html {
    if props.children.is_empty() {
        ::yew::html! { <div>{"no children"}</div> }
    } else {
        ::yew::html! { <div>{ for props.children.iter() }</div> }
    }
}

#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::yew::Properties)]
pub struct LabelProps {
    pub content: ::yew::Html,
}

#[::yew::component]
fn Label(_props: &LabelProps) -> ::yew::Html {
    ::yew::html! {}
}

fn main() {
    let _ = ::yew::html! {
        <Container>
            <div>{"hello"}</div>
            <span>{"world"}</span>
        </Container>
    };

    let _ = ::yew::html! {
        <Container>{"just text"}</Container>
    };

    let _ = ::yew::html! { <Container /> };

    let s = ::std::string::String::from("hello");
    let _ = ::yew::html! { <Label content={s} /> };

    let _ = ::yew::html! { <Label content={"world"} /> };

    let label = ::std::string::String::from("hello");
    let _ = ::yew::html! { <div>{&label}</div> };
    let _ = ::yew::html! { <div>{label}</div> };
}
