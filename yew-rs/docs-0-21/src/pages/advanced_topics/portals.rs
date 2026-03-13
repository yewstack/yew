crate::doc_page!(
    "Portals",
    "/docs/advanced-topics/portals",
    Content::new(vec![
        h2(vec![text("What is a portal?")]),
        p(vec![
            text(
                "Portals provide a first-class way to render children into a DOM node that exists \
                 outside the DOM hierarchy of the parent component. "
            ),
            code("yew::create_portal(child, host)"),
            text(" returns an "),
            code("Html"),
            text(" value that renders "),
            code("child"),
            text(" not hierarchically under its parent component, but as a child of the "),
            code("host"),
            text(" element."),
        ]),
        h2(vec![text("Usage")]),
        p(vec![
            text(
                "Typical uses of portals can include modal dialogs and hovercards, as well as \
                 more technical applications such as controlling the contents of an element's "
            ),
            link(
                "https://developer.mozilla.org/en-US/docs/Web/API/Element/shadowRoot",
                vec![text("shadowRoot")]
            ),
            text(", appending stylesheets to the surrounding document's "),
            code("<head>"),
            text(" and collecting referenced elements inside a central "),
            code("<defs>"),
            text(" element of an "),
            code("<svg>"),
            text("."),
        ]),
        p(vec![
            text("Note that "),
            code("yew::create_portal"),
            text(
                " is a low-level building block. Libraries should use it to implement \
                 higher-level APIs which can then be consumed by applications. For example, here \
                 is a simple modal dialogue that renders its "
            ),
            code("children"),
            text(" into an element outside "),
            code("yew"),
            text("'s control, identified by the "),
            code(r#"id="modal_host""#),
            text("."),
        ]),
        code_block(
            "rust",
            r##"use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn Modal(props: &ModalProps) -> Html {
    let modal_host = gloo::utils::document()
        .get_element_by_id("modal_host")
        .expect("Expected to find a #modal_host element");

    create_portal(
        props.children.clone(),
        modal_host.into(),
    )
}"##
        ),
        h2(vec![text("Event handling")]),
        p(vec![text(
            "Events emitted on elements inside portals follow the virtual DOM when bubbling up. \
             That is, if a portal is rendered as the child of an element, then an event listener \
             on that element will catch events dispatched from inside the portal, even if the \
             portal renders its contents in an unrelated location in the actual DOM."
        ),]),
        p(vec![text(
            "This allows developers to be oblivious of whether a component they consume, is \
             implemented with or without portals. Events fired on its children will bubble up \
             regardless."
        ),]),
        p(vec![
            text("A known issue is that events from portals into "),
            bold(vec![text("closed")]),
            text(
                " shadow roots will be dispatched twice, once targeting the element inside the \
                 shadow root and once targeting the host element itself. Keep in mind that "
            ),
            bold(vec![text("open")]),
            text(
                " shadow roots work fine. If this impacts you, feel free to open a bug report \
                 about it."
            ),
        ]),
        h2(vec![text("Further reading")]),
        ul(vec![li(vec![link(
            "https://github.com/yewstack/yew/tree/master/examples/portals",
            vec![text("Portals example")]
        ),]),]),
    ])
);
