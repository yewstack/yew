crate::doc_page!(
    "Suspense",
    "/zh-Hant/docs/concepts/suspense",
    Content::new(vec![
        p(vec![text(
            "Suspense is a way to suspend component rendering whilst waiting a task to complete \
             and a fallback (placeholder) UI is shown in the meanwhile."
        ),]),
        p(vec![text(
            "It can be used to fetch data from server, wait for tasks to be completed by an \
             agent, or perform other background asynchronous task."
        ),]),
        p(vec![text(
            "Before suspense, data fetching usually happens after (Fetch-on-render) or before \
             component rendering (Fetch-then-render)."
        ),]),
        h3(vec![text("Render-as-You-Fetch")]),
        p(vec![text(
            "Suspense enables a new approach that allows components to initiate data request \
             during the rendering process. When a component initiates a data request, the \
             rendering process will become suspended and a fallback UI will be shown until the \
             request is completed."
        ),]),
        p(vec![text(
            "The recommended way to use suspense is with hooks."
        )]),
        code_block_ignore(
            "rust",
            r##"use yew::prelude::*;

#[function_component(Content)]
fn content() -> HtmlResult {
    let user = use_user()?;

    Ok(html! {<div>{"Hello, "}{&user.name}</div>})
}

#[function_component(App)]
fn app() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Content />
        </Suspense>
    }
}"##
        ),
        p(vec![
            text("In the above example, the "),
            code("use_user"),
            text(
                " hook will suspend the component rendering while user information is loading and \
                 a "
            ),
            code("Loading..."),
            text(" placeholder will be shown until "),
            code("user"),
            text(" is loaded."),
        ]),
        p(vec![
            text("To define a hook that suspends a component rendering, it needs to return a "),
            code("SuspensionResult<T>"),
            text(". When the component needs to be suspended, the hook should return a "),
            code("Err(Suspension)"),
            text(" and users should unwrap it with "),
            code("?"),
            text(" in which it will be converted into "),
            code("Html"),
            text("."),
        ]),
        code_block_ignore(
            "rust",
            r##"use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

struct User {
    name: String,
}

#[hook]
fn use_user() -> SuspensionResult<User> {
    match load_user() {
        // If a user is loaded, then we return it as Ok(user).
        Some(m) => Ok(m),
        None => {
            // When user is still loading, then we create a `Suspension`
            // and call `SuspensionHandle::resume` when data loading
            // completes, the component will be re-rendered
            // automatically.
            let (s, handle) = Suspension::new();
            on_load_user_complete(move || {handle.resume();});
            Err(s)
        },
    }
}"##
        ),
        h1(vec![text("Complete Example")]),
        code_block(
            "rust",
            r##"use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

#[derive(Debug)]
struct User {
    name: String,
}

fn load_user() -> Option<User> {
    todo!()  // implementation omitted.
}

fn on_load_user_complete<F: FnOnce()>(_fn: F) {
    todo!()  // implementation omitted.
}

#[hook]
fn use_user() -> SuspensionResult<User> {
    match load_user() {
        // If a user is loaded, then we return it as Ok(user).
        Some(m) => Ok(m),
        None => {
            // When user is still loading, then we create a `Suspension`
            // and call `SuspensionHandle::resume` when data loading
            // completes, the component will be re-rendered
            // automatically.
            let (s, handle) = Suspension::new();
            on_load_user_complete(move || {handle.resume();});
            Err(s)
        },
    }
}

#[function_component(Content)]
fn content() -> HtmlResult {
    let user = use_user()?;

    Ok(html! {<div>{"Hello, "}{&user.name}</div>})
}

#[function_component(App)]
fn app() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Content />
        </Suspense>
    }
}"##
        ),
        h3(vec![text("Use Suspense in Struct Components")]),
        p(vec![
            text(
                "It's not possible to suspend a struct component directly. However, you can use a \
                 function component as a "
            ),
            link(
                "/zh-Hant/docs/advanced-topics/struct-components/hoc",
                vec![text("HOC")]
            ),
            text(" to achieve suspense-based data fetching."),
        ]),
        code_block_ignore(
            "rust",
            r##"use yew::prelude::*;

#[function_component(WithUser)]
fn with_user<T>() -> HtmlResult
where T: BaseComponent
{
    let user = use_user()?;

    Ok(html! {<T {user} />})
}

#[derive(Debug, PartialEq, Properties)]
pub struct UserContentProps {
    pub user: User,
}

pub struct BaseUserContent;

impl Component for BaseUserContent {
    type Properties = UserContentProps;
    type Message = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let name = ctx.props().user.name;

        html! {<div>{"Hello, "}{name}{"!"}</div>}
    }
}

pub type UserContent = WithUser<BaseUserContent>;"##
        ),
        h2(vec![text("Relevant examples")]),
        ul(vec![li(vec![link(
            "https://github.com/yewstack/yew/tree/yew-v0.20.0/examples/suspense",
            vec![text("Suspense")]
        ),]),]),
    ])
);
