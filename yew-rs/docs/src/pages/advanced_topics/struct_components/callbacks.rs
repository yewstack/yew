pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("Callbacks")],
        p![
            text(
                "Callbacks are used to communicate with services, agents, and parent components \
                 within Yew. Internally their type is just ",
            ),
            code("Fn"),
            text(" wrapped in "),
            code("Rc"),
            text(" to allow them to be cloned."),
        ],
        p![
            text("They have an "),
            code("emit"),
            text(" function that takes their "),
            code("<IN>"),
            text(
                " type as an argument and converts that to a message expected by its destination. \
                 If a callback from a parent is provided in props to a child component, the child \
                 can call ",
            ),
            code("emit"),
            text(" on the callback in its "),
            code("update"),
            text(
                " lifecycle hook to send a message back to its parent. Closures or Functions \
                 provided as props inside the ",
            ),
            code("html!"),
            text(" macro are automatically converted to Callbacks."),
        ],
        p![text(
            "A simple use of a callback might look something like this:",
        )],
        code_block(
            "rust",
            r#"use yew::{html, Component, Context, Html};

enum Msg {
    Clicked,
}

struct Comp;

impl Component for Comp {

    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // highlight-next-line
        let onclick = ctx.link().callback(|_| Msg::Clicked);
        html! {
            // highlight-next-line
            <button {onclick}>{ "Click" }</button>
        }
    }
}"#,
        ),
        p![
            text("The function passed to "),
            code("callback"),
            text(" must always take a parameter. For example, the "),
            code("onclick"),
            text(" handler requires a function that takes a parameter of type "),
            code("MouseEvent"),
            text(
                ". The handler can then decide what kind of message should be sent to the \
                 component. This message is scheduled for the next update loop unconditionally.",
            ),
        ],
        p![
            text("If you need a callback that might not need to cause an update, use "),
            code("batch_callback"),
            text("."),
        ],
        code_block(
            "rust",
            r#"use yew::{events::KeyboardEvent, html, Component, Context, Html};

enum Msg {
    Submit,
}

struct Comp;

impl Component for Comp {

    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // highlight-start
        let onkeypress = ctx.link().batch_callback(|event: KeyboardEvent| {
            if event.key() == "Enter" {
                Some(Msg::Submit)
            } else {
                None
            }
        });

        html! {
            <input type="text" {onkeypress} />
        }
        // highlight-end
    }
}"#,
        ),
        h2![text("Relevant examples")],
        ul![
            li![link![
                "https://github.com/yewstack/yew/tree/master/examples/counter",
                text("Counter"),
            ]],
            li![link![
                "https://github.com/yewstack/yew/tree/master/examples/timer",
                text("Timer"),
            ]],
        ],
    ])
}

crate::doc_page!(
    "Callbacks",
    "/docs/advanced-topics/struct-components/callbacks",
    page_content()
);
