pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![code("use_reducer"), text(" 不再在恒等分发时重新渲染")]),
        p(vec![
            code("use_reducer"),
            text(" 现在在 reducer 返回相同的 "),
            code("Rc"),
            text(" 时（通过指针相等性判断）会跳过重新渲染。之前，每次分发都会触发重新渲染。"),
        ]),
        p(vec![
            text("如果你的 reducer 有一个返回 "),
            code("self"),
            text(" 不变的代码路径，并且你依赖它来触发重新渲染，请用 "),
            code("use_force_update"),
            text(" 替代："),
        ]),
        tabs(
            "before",
            vec![
                tab(
                    "before",
                    "之前",
                    vec![code_block(
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
            // 在 0.23 中这不再触发重新渲染！
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
                { "刷新" }
            </button>
        </div>
    }
}"##,
                    )],
                ),
                tab(
                    "after",
                    "之后",
                    vec![code_block(
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
            <button onclick={move |_| trigger.force_update()}>{ "刷新" }</button>
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
    "/zh-Hans/docs/migration-guides/yew/from-0-22-0-to-0-23-0",
    page_content()
);
