crate::doc_page!(
    "Portals",
    "/ja/docs/advanced-topics/portals",
    Content::new(vec![
        h2!["What is a portal?"],
        p![
            "Portals provide a first-class way to render children into a DOM node that exists \
             outside the DOM hierarchy of the parent component. ",
            code("yew::create_portal(child, host)"),
            " returns a ",
            code("Html"),
            " value that renders ",
            code("child"),
            " not hierarchically under its parent component, but as a child of the ",
            code("host"),
            " element.",
        ],
        h2!["Usage"],
        p![
            "Typical uses of portals can include modal dialogs and hovercards, as well as more \
             technical applications such as controlling the contents of an element's ",
            link![
                "https://developer.mozilla.org/en-US/docs/Web/API/Element/shadowRoot",
                "shadowRoot"
            ],
            ", appending stylesheets to the surrounding document's ",
            code("<head>"),
            " and collecting referenced elements inside a central ",
            code("<defs>"),
            " element of an ",
            code("<svg>"),
            ".",
        ],
        p![
            "Note that ",
            code("yew::create_portal"),
            " is a low-level building block. Libraries should use it to implement higher-level \
             APIs which can then be consumed by applications. For example, here is a simple modal \
             dialogue that renders its ",
            code("children"),
            " into an element outside ",
            code("yew"),
            "'s control, identified by the ",
            code("id=\"modal_host\""),
            ".",
        ],
        code_block(
            "rust",
            r#"use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Modal)]
fn Modal(props: &ModalProps) -> Html {
    let modal_host = gloo::utils::document()
        .get_element_by_id("modal_host")
        .expect("Expected to find a #modal_host element");

    create_portal(
        html!{ {for props.children.iter()} },
        modal_host.into(),
    )
}"#
        ),
        h2!["Event handling"],
        p![
            "Events emitted on elements inside portals follow the virtual DOM when bubbling up. \
             That is, if a portal is rendered as the child of an element, then an event listener \
             on that element will catch events dispatched from inside the portal, even if the \
             portal renders its contents in an unrelated location in the actual DOM."
        ],
        p![
            "This allows developers to be oblivious of whether a component they consume, is \
             implemented with or without portals. Events fired on its children will bubble up \
             regardless."
        ],
        p![
            "A known issue is that events from portals into ",
            bold!["closed"],
            " shadow roots will be dispatched twice, once targeting the element inside the shadow \
             root and once targeting the host element itself. Keep in mind that ",
            bold!["open"],
            " shadow roots work fine. If this impacts you, feel free to open a bug report about \
             it.",
        ],
        h2!["Further reading"],
        ul![li![link![
            "https://github.com/yewstack/yew/tree/yew-v0.20.0/examples/portals",
            "Portals example"
        ]]],
    ])
    .with_description("Rendering into out-of-tree DOM nodes")
);
