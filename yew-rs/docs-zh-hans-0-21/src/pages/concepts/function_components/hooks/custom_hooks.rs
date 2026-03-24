crate::doc_page!(
    "自定义钩子（Custom Hooks）",
    "/zh-Hans/docs/concepts/function-components/hooks/custom-hooks",
    Content::new(vec![
        h2![text("定义自定义钩子")],
        p![text(
            "组件中与状态有关的逻辑可以通过创建自定义 Hooks 提取到函数中。"
        )],
        p![text(
            "假设我们有一个组件，它订阅了一个代理（agent）并且会显示发送给它的消息。"
        )],
        code_block(
            "rust",
            r#"#[function_component(ShowMessages)]
pub fn show_messages() -> Html {
    let (state, set_state) = use_state(|| vec![]);

    {
        let mut state = Rc::clone(&state);
        use_effect(move || {
            let producer = EventBus::bridge(Callback::from(move |msg| {
                let mut messages = (*state).clone();
                messages.push(msg);
                set_state(messages)
            }));

            || drop(producer)
        });
    }

    let output = state.iter().map(|it| html! { <p>{ it }</p> });
    html! { <div>{ for output }</div> }
}"#
        ),
        p![text(
            "这段代码有一个问题：逻辑不能被另一个组件重用。如果我们构建另一个跟踪消息的组件，\
             我们可以将逻辑移动到自定义钩子中，而不是复制代码。"
        )],
        p![
            text("我们将首先创建一个名为"),
            code("use_subscribe"),
            text("的新函数。 "),
            code("use_"),
            text("前缀通常表示此函数是一个钩子。这个函数将不接受任何参数并返回"),
            code("Rc<RefCell<Vec<String>>>"),
            text(" 。")
        ],
        code_block(
            "rust",
            r#"fn use_subscribe() -> Rc<RefCell<Vec<String>>> {
    // ...
}"#
        ),
        p![
            text("钩子的逻辑在"),
            code("use_hook"),
            text("的回调中。 "),
            code("use_hook"),
            text("指的是自定义 Hook 的处理函数。它接受 2 个参数： "),
            code("hook_runner"),
            text("和"),
            code("initial_state_producer"),
            text(" 。")
        ],
        p![
            code("hook_runner"),
            text("中包含了所有钩子的逻辑，它的回调的返回值又会被"),
            code("use_hook"),
            text("返回。 "),
            code("hook_runner"),
            text("需要 2 个参数：分别是对钩子和"),
            code("hook_callback"),
            text("它们两个的内部状态的可变引用。 而"),
            code("hook_callback"),
            text("同样也要 2 个参数：一个回调和一个 bool，回调接受"),
            code("internal_state"),
            text(" ，也就是对内部状态实例的可变引用，并且会调执行实际的更改，还会返回表示"),
            code("ShouldRender"),
            text("的布尔值，第二个参数 bool 的用处是指示它是否在组件渲染后运行。"),
            code("use_hook"),
            text("的第二个参数"),
            code("initial_state_producer"),
            text("接受用于创建内部状态实例的回调。这里说的内部状态指的是一个实现了"),
            code("Hook"),
            text(" trait 的结构体。")
        ],
        p![
            text("现在让我们为"),
            code("use_subscribe"),
            text("钩子创建状态（state struct）。")
        ],
        code_block(
            "rust",
            r#"/// `use_subscribe` internal state
struct UseSubscribeState {
    /// holds all the messages received
    pub messages: Rc<RefCell<Vec<String>>>,
}

impl Hook for UseSubscribeState {}"#
        ),
        p![
            text("接下来我们为"),
            code("use_subscribe"),
            text("添加实际逻辑。")
        ],
        code_block(
            "rust",
            r#"fn use_subscribe() -> Rc<RefCell<Vec<String>>> {
    use_hook(
        // hook's handler. all the logic goes in here
        |state: &mut UseSubscribeState, hook_callback| {
            // calling other Hooks inside a hook
            use_effect(move || {
                let producer = EventBus::bridge(Callback::from(move |msg| {
                    hook_callback(
                        // where the mutations of state are performed
                        |state| {
                            (*state.messages).borrow_mut().deref_mut().push(msg);
                            true // should re-render
                        }, false // run post-render
                    )
                }));

                || drop(producer)
            });

            // return from hook
            state.messages.clone()
        },
        // initial state producer
        || UseSubscribeState { messages: Rc::new(RefCell::new(vec![])) },
    )
}"#
        ),
        p![text("现在我们可以使用自定义钩子了：")],
        code_block(
            "rust",
            r#"#[function_component(ShowMessages)]
pub fn show_messages() -> Html {
    let state = use_subscribe();
    let output = state.borrow().deref().into_iter().map(|it| html! { <p>{ it }</p> });

    html! { <div>{ for output }</div> }
}"#
        ),
        p![
            text("需要特别注意的是创建自定义钩子时"),
            code("use_hook"),
            text("不是必须的，它们只是用来包含其他钩子。通常应避免使用"),
            code("use_hook"),
            text("。")
        ],
        code_block(
            "rust",
            r#"fn use_subscribe() -> Rc<Vec<String>> {
    let (state, set_state) = use_state(Vec::new);

    use_effect(move || {
        let producer = EventBus::bridge(Callback::from(move |msg| {
            let mut messages = (*state).clone();
            messages.push(msg);
            set_state(messages)
        }));
        || drop(producer)
    });

    state
}"#
        )
    ])
);
