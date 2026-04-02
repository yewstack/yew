crate::doc_page!(
    "Callbacks",
    "/docs/advanced-topics/struct-components/callbacks",
    Content::new(vec![
        h2!["Callbacks"],
        p![
            "Callbacks are used to communicate with services, agents, and parent components \
             within Yew. Internally their type is just ",
            code("Fn"),
            " wrapped in ",
            code("Rc"),
            " to allow them to be cloned.",
        ],
        p![
            "They have an ",
            code("emit"),
            " function that takes their ",
            code("<IN>"),
            " type as an argument and converts that to a message expected by its destination. If \
             a callback from a parent is provided in props to a child component, the child can \
             call ",
            code("emit"),
            " on the callback in its ",
            code("update"),
            " lifecycle hook to send a message back to its parent. Closures or Functions provided \
             as props inside the ",
            code("html!"),
            " macro are automatically converted to Callbacks.",
        ],
        p!["A simple use of a callback might look something like this:"],
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
}"#
        ),
        p![
            "The function passed to ",
            code("callback"),
            " must always take a parameter. For example, the ",
            code("onclick"),
            " handler requires a function which takes a parameter of type ",
            code("MouseEvent"),
            ". The handler can then decide what kind of message should be sent to the component. \
             This message is scheduled for the next update loop unconditionally.",
        ],
        p![
            "If you need a callback that might not need to cause an update, use ",
            code("batch_callback"),
            ".",
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
}"#
        ),
        h2!["Relevant examples"],
        ul![
            li![link![
                "https://github.com/yewstack/yew/tree/yew-v0.20.0/examples/counter",
                "Counter"
            ],],
            li![link![
                "https://github.com/yewstack/yew/tree/yew-v0.20.0/examples/timer",
                "Timer"
            ],],
        ],
    ])
);
