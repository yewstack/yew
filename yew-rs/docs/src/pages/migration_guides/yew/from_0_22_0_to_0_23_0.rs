pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![
            code("use_reducer"),
            text(" no longer re-renders on identity dispatches"),
        ]),
        p(vec![
            code("use_reducer"),
            text(" now skips re-rendering when the reducer returns the same "),
            code("Rc"),
            text(
                " (checked by pointer equality). Previously, every dispatch triggered a re-render \
                 regardless.",
            ),
        ]),
        p(vec![
            text("If your reducer has a code path that returns "),
            code("self"),
            text(" unchanged and you relied on that causing a re-render, replace it with "),
            code("use_force_update"),
            text(":"),
        ]),
        tabs(
            "before",
            vec![
                tab(
                    "before",
                    "Before",
                    vec![code_block_ignore(
                        "rust",
                        r##"pub enum Action {
    Increment,
    ForceRefresh,
}

struct State {
    count: u32,
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Increment => Rc::new(Self {
                count: self.count + 1,
            }),
            // This no longer triggers a re-render in 0.23!
            Action::ForceRefresh => self,
        }
    }
}

#[component]
pub fn App() -> Html {
    use_effect(|| {
        tracing::info!("This cursed component does some effects on render");
    });
    let state = use_reducer(|| State { count: 0 });
    html! {
        <div>
            <p>{ state.count }</p>
            <button onclick={
                let state = state.clone();
                move |_| state.dispatch(Action::Increment)
            }>
                { "+1" }
            </button>
            <button onclick={move |_| state.dispatch(Action::ForceRefresh)}>
                { "Refresh" }
            </button>
        </div>
    }
}"##,
                    )],
                ),
                tab(
                    "after",
                    "After",
                    vec![code_block_ignore(
                        "rust",
                        r##"pub enum Action {
    Increment,
}

struct State {
    count: u32,
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Increment => Rc::new(Self {
                count: self.count + 1,
            }),
        }
    }
}

#[component]
pub fn App() -> Html {
    use_effect(|| {
        tracing::info!("This cursed component does some effects on render");
    });
    let state = use_reducer(|| State { count: 0 });
    let trigger = use_force_update();
    html! {
        <div>
            <p>{ state.count }</p>
            <button onclick={move |_| state.dispatch(Action::Increment)}>{ "+1" }</button>
            <button onclick={move |_| trigger.force_update()}>{ "Refresh" }</button>
        </div>
    }
}"##,
                    )],
                ),
            ],
        ),
    ])
}

crate::doc_page!(
    "From 0.22.0 to 0.23.0",
    "/docs/migration-guides/yew/from-0-22-0-to-0-23-0",
    page_content()
);
