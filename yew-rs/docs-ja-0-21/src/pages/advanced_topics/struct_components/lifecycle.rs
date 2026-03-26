crate::doc_page!(
    "ライフサイクル",
    "/ja/docs/advanced-topics/struct-components/lifecycle",
    Content::new(vec![
        p![
            "The ",
            code("Component"),
            " trait has a number of methods which need to be implemented; Yew will call these at \
             different stages in the lifecycle of a component.",
        ],
        h2!["Lifecycle"],
        admonition![
            AdmonitionType::Important,
            Some("contribute"),
            p![
                code("Contribute to our docs:"),
                " ",
                link!(
                    "https://github.com/yewstack/yew/issues/1915",
                    "Add a diagram of the component lifecycle"
                ),
            ],
        ],
        h2!["Lifecycle Methods"],
        h3!["Create"],
        p![
            "When a component is created, it receives properties from its parent component and is \
             stored within the ",
            code("Context<Self>"),
            " that is passed down to the ",
            code("create"),
            " method. The properties can be used to initialize the component's state and the \
             \"link\" can be used to register callbacks or send messages to the component.",
        ],
        code_block(
            "rust",
            r#"use yew::{Component, Context, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct MyComponent;

impl Component for MyComponent {
    type Message = ();
    type Properties = Props;

    // highlight-start
    fn create(ctx: &Context<Self>) -> Self {
        MyComponent
    }
    // highlight-end

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            // impl
        }
    }
}"#
        ),
        h3!["View"],
        p![
            "The ",
            code("view"),
            " method allows you to describe how a component should be rendered to the DOM. \
             Writing HTML-like code using Rust functions can become quite messy, so Yew provides \
             a macro called ",
            code("html!"),
            " for declaring HTML and SVG nodes (as well as attaching attributes and event \
             listeners to them) and a convenient way to render child components. The macro is \
             somewhat similar to React's JSX (the differences in programming language aside). One \
             difference is that Yew provides a shorthand syntax for properties, similar to \
             Svelte, where instead of writing ",
            code("onclick={onclick}"),
            ", you can just write ",
            code("{onclick}"),
            ".",
        ],
        code_block(
            "rust",
            r#"use yew::{Component, Context, html, Html, Properties};

enum Msg {
    Click,
}

#[derive(PartialEq, Properties)]
struct Props {
    button_text: String,
}

struct MyComponent;

impl Component for MyComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    // highlight-start
    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::Click);
        html! {
            <button {onclick}>{ &ctx.props().button_text }</button>
        }
    }
    // highlight-end
}"#
        ),
        p![
            "For usage details, check out ",
            doc_link!(
                crate::pages::concepts::html::introduction,
                "the html! guide"
            ),
            ".",
        ],
        h3!["Rendered"],
        p![
            "The ",
            code("rendered"),
            " component lifecycle method is called once ",
            code("view"),
            " has been called and Yew has rendered the results to the DOM, but before the browser \
             refreshes the page. This method is useful when you want to perform actions that can \
             only be completed after the component has rendered elements. There is also a \
             parameter called ",
            code("first_render"),
            " which can be used to determine whether this function is being called on the first \
             render, or instead a subsequent one.",
        ],
        code_block(
            "rust",
            r#"use web_sys::HtmlInputElement;
use yew::{
    Component, Context, html, Html, NodeRef,
};

pub struct MyComponent {
    node_ref: NodeRef,
}

impl Component for MyComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <input ref={self.node_ref.clone()} type="text" />
        }
    }

    // highlight-start
    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
                input.focus();
            }
        }
    }
    // highlight-end
}"#
        ),
        admonition![
            AdmonitionType::Tip,
            Some("note"),
            p![
                "Note that this lifecycle method does not require implementation and will do \
                 nothing by default."
            ],
        ],
        h3!["Update"],
        p![
            "Communication with components happens primarily through messages which are handled \
             by the ",
            code("update"),
            " lifecycle method. This allows the component to update itself based on what the \
             message was, and determine if it needs to re-render itself. Messages can be sent by \
             event listeners, child components, Agents, Services, or Futures.",
        ],
        p![
            "Here is an example of what an implementation of ",
            code("update"),
            " could look like:",
        ],
        code_block(
            "rust",
            r#"use yew::{Component, Context, html, Html};

// highlight-start
pub enum Msg {
    SetInputEnabled(bool)
}
// highlight-end

struct MyComponent {
    input_enabled: bool,
}

impl Component for MyComponent {
    // highlight-next-line
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input_enabled: false,
        }
    }

    // highlight-start
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetInputEnabled(enabled) => {
                if self.input_enabled != enabled {
                    self.input_enabled = enabled;
                    true // Re-render
                } else {
                    false
                }
            }
        }
    }
    // highlight-end

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            // impl
        }
    }

}"#
        ),
        h3!["Changed"],
        p![
            "Components may be re-rendered by their parents. When this happens, they could \
             receive new properties and need to re-render. This design facilitates \
             parent-to-child component communication by just changing the values of a property. \
             There is a default implementation that re-renders the component when props are \
             changed.",
        ],
        h3!["Destroy"],
        p![
            "After Components are unmounted from the DOM, Yew calls the ",
            code("destroy"),
            " lifecycle method; this is necessary if you need to undertake operations to clean up \
             after earlier actions of a component before it is destroyed. This method is optional \
             and does nothing by default.",
        ],
        h3!["Infinite loops"],
        p![
            "Infinite loops are possible with Yew's lifecycle methods but are only caused when \
             trying to update the same component after every render, when that update also \
             requests the component to be rendered.",
        ],
        p!["A simple example can be seen below:"],
        code_block(
            "rust",
            r#"use yew::{Context, Component, Html};

struct Comp;

impl Component for Comp {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        // We are going to always request to re-render on any msg
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // For this example it doesn't matter what is rendered
        Html::default()
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        // Request that the component is updated with this new msg
        ctx.link().send_message(());
    }
}"#
        ),
        p!["Let's run through what happens here:"],
        ol![
            li![
                "Component is created using the ",
                code("create"),
                " function.",
            ],
            li![
                "The ",
                code("view"),
                " method is called so Yew knows what to render to the browser DOM.",
            ],
            li![
                "The ",
                code("rendered"),
                " method is called, which schedules an update message using the ",
                code("Context"),
                " link.",
            ],
            li!["Yew finishes the post-render phase."],
            li![
                "Yew checks for scheduled events and sees the update message queue is not empty \
                 so works through the messages.",
            ],
            li![
                "The ",
                code("update"),
                " method is called which returns ",
                code("true"),
                " to indicate something has changed and the component needs to re-render.",
            ],
            li!["Jump back to 2."],
        ],
        p![
            "You can still schedule updates in the ",
            code("rendered"),
            " method and it is often useful to do so, but consider how your component will \
             terminate this loop when you do.",
        ],
        h2!["Associated Types"],
        p![
            "The ",
            code("Component"),
            " trait has two associated types: ",
            code("Message"),
            " and ",
            code("Properties"),
            ".",
        ],
        code_block_ignore(
            "rust",
            r#"impl Component for MyComponent {
    type Message = Msg;
    type Properties = Props;

    // ...
}"#
        ),
        p![
            "The ",
            code("Message"),
            " type is used to send messages to a component after an event has taken place; for \
             example, you might want to undertake some action when a user clicks a button or \
             scrolls down the page. Because components tend to have to respond to more than one \
             event, the ",
            code("Message"),
            " type will normally be an enum, where each variant is an event to be handled.",
        ],
        p![
            "When organizing your codebase, it is sensible to include the definition of the ",
            code("Message"),
            " type in the same module in which your component is defined. You may find it helpful \
             to adopt a consistent naming convention for message types. One option (though not \
             the only one) is to name the types ",
            code("ComponentNameMsg"),
            ", e.g. if your component was called ",
            code("Homepage"),
            " then you might call the type ",
            code("HomepageMsg"),
            ".",
        ],
        code_block(
            "rust",
            r#"enum Msg {
    Click,
    FormInput(String)
}"#
        ),
        p![
            code("Properties"),
            " represents the information passed to a component from its parent. This type must \
             implement the ",
            code("Properties"),
            " trait (usually by deriving it) and can specify whether certain properties are \
             required or optional. This type is used when creating and updating a component. It \
             is common practice to create a struct called ",
            code("Props"),
            " in your component's module and use that as the component's ",
            code("Properties"),
            " type. It is common to shorten \"properties\" to \"props\". Since props are handed \
             down from parent components, the root component of your application typically has a ",
            code("Properties"),
            " type of ",
            code("()"),
            ". If you wish to specify properties for your root component, use the ",
            code("App::mount_with_props"),
            " method.",
        ],
        admonition![
            AdmonitionType::Info,
            None,
            p![doc_link!(
                crate::pages::advanced_topics::struct_components::properties,
                "Learn more about properties"
            ),],
        ],
        h2!["Lifecycle Context"],
        p![
            "All component lifecycle methods take a context object. This object provides a \
             reference to the component's scope, which allows sending messages to a component and \
             the props passed to the component.",
        ],
    ])
);
