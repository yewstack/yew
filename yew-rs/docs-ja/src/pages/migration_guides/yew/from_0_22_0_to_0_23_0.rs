pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![
            code("use_reducer"),
            " が同一ディスパッチで再レンダリングしなくなりました",
        ],
        p![
            code("use_reducer"),
            " はリデューサーが同じ ",
            code("Rc"),
            " を返した場合（ポインタの同一性で判定）、\
             再レンダリングをスキップするようになりました。以前は、\
             すべてのディスパッチで再レンダリングが発生していました。",
        ],
        p![
            "リデューサーに ",
            code("self"),
            " をそのまま返すコードパスがあり、それによる再レンダリングに依存していた場合は、",
            code("use_force_update"),
            " に置き換えてください:",
        ],
        tabs![
            "before",
            tab![
                "before",
                "変更前",
                code_block(
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
            // 0.23 ではこれは再レンダリングを発生させません!
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
                { "リフレッシュ" }
            </button>
        </div>
    }
}"##,
                ),
            ],
            tab![
                "after",
                "変更後",
                code_block(
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
            <button onclick={move |_| trigger.force_update()}>{ "リフレッシュ" }</button>
        </div>
    }
}"##,
                ),
            ],
        ],
    ])
}

crate::doc_page!(
    "From 0.22.0 to 0.23.0",
    "/ja/docs/migration-guides/yew/from-0-22-0-to-0-23-0",
    page_content()
);
