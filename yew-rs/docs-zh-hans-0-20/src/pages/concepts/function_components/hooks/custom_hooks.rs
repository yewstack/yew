crate::doc_page!(
    "自定义钩子（Custom Hooks）",
    "/zh-Hans/docs/concepts/function-components/hooks/custom-hooks",
    Content::new(vec![
        h2(vec![text("定义自定义钩子")]),
        p(vec![text(
            "组件中与状态有关的逻\\
             u{8f91}可以通过创建自定义 Hooks 提取到函数中。"
        )]),
        p(vec![text(
            "假设我们有一个组件，\\
             u{5b83}订阅了一个代理（agent）\\
             u{5e76}且会显示发送给它的\\
             u{6d88}息。"
        )]),
        code_block(
            "rust",
            r#"use yew::prelude::*;
use gloo::events::EventListener;
use gloo::utils::window;
use std::mem::drop;


#[function_component(ShowStorageChanged)]
pub fn show_storage_changed() -> Html {
    let state_storage_changed = use_state(|| false);

    {
        let state_storage_changed = state_storage_changed.clone();
        use_effect(|| {
            let listener = EventListener::new(&window(), "storage", move |_| state_storage_changed.set(true));

            move || { drop(listener); }
        });
    }

    html! { <div>{"Storage Event Fired: "}{*state_storage_changed}</div> }
}"#
        ),
        p(vec![text(
            "这段代码有一个问题：\\
             u{903b}辑不能被另一个组件\\
             u{91cd}用。如果我们构建另\\
             u{4e00}个跟踪消息的组件，\\
             u{6211}们可以将逻辑移动到\\
             u{81ea}定义钩子中，而不是\\
             u{590d}制代码。"
        )]),
        p(vec![
            text(
                "我们将首先创建一个名\\
                 u{4e3a}"
            ),
            code("use_subscribe"),
            text("的新函数。 "),
            code("use_"),
            text(
                "前缀通常表示此函数是\\
                 u{4e00}个钩子。这个函数将\\
                 u{4e0d}接受任何参数并返回"
            ),
            code("Rc<RefCell<Vec<String>>>"),
            text(" 。"),
        ]),
        code_block(
            "rust",
            r#"use web_sys::{Event, EventTarget};
use std::borrow::Cow;
use gloo::events::EventListener;
use yew::prelude::*;

#[hook]
pub fn use_event<E, F>(target: &EventTarget, event_type: E, callback: F)
where
    E: Into<Cow<'static, str>>,
    F: Fn(&Event) + 'static,
{
    todo!()
}"#
        ),
        p(vec![
            text("钩子的逻辑在"),
            code("use_hook"),
            text("的回调中。 "),
            code("use_hook"),
            text("指的是自定义 Hook 的处理函数。它接受 2 个参数： "),
            code("hook_runner"),
            text("和"),
            code("initial_state_producer"),
            text(" 。"),
        ]),
        p(vec![
            code("hook_runner"),
            text(
                "中包含了所有钩子的逻\\
                 u{8f91}，它的回调的返回值\\
                 u{53c8}会被"
            ),
            code("use_hook"),
            text("返回。 "),
            code("hook_runner"),
            text(
                "需要 2 个参数：分别是对钩子\\
                 u{548c}"
            ),
            code("hook_callback"),
            text(
                "它们两个的内部状态的\\
                 u{53ef}变引用。 而"
            ),
            code("hook_callback"),
            text(
                "同样也要 2 个参数：一个回调和一\\
                 u{4e2a} bool，回调接受"
            ),
            code("internal_state"),
            text(
                " ，也就是对内部状态\\
                 u{5b9e}例的可变引用，并且\\
                 u{4f1a}调执行实际的更改，\\
                 u{8fd8}会返回表示"
            ),
            code("ShouldRender"),
            text(
                "的布尔值，第二个参数 bool 的用处是指示它是否\\
                 u{5728}组件渲染后运行。"
            ),
            code("use_hook"),
            text("的第二个参数"),
            code("initial_state_producer"),
            text(
                "接受用于创建内部状态\\
                 u{5b9e}例的回调。这里说的\\
                 u{5185}部状态指的是一个实\\
                 u{73b0}了"
            ),
            code("Hook"),
            text(" trait 的结构体。"),
        ]),
        p(vec![
            text("现在让我们为"),
            code("use_subscribe"),
            text("钩子创建状态（state struct）。"),
        ]),
        code_block(
            "rust",
            r#"use yew::prelude::*;
use web_sys::{Event, EventTarget};
use std::borrow::Cow;
use std::rc::Rc;
use gloo::events::EventListener;

#[hook]
pub fn use_event<E, F>(target: &EventTarget, event_type: E, callback: F)
where
    E: Into<Cow<'static, str>>,
    F: Fn(Event) + 'static,
{
    #[derive(PartialEq, Clone)]
    struct EventDependents {
        target: EventTarget,
        event_type: Cow<'static, str>,
        callback: Callback<Event>,
    }

    let deps = EventDependents {
        target: target.clone(),
        event_type: event_type.into(),
        callback: Callback::from(callback),
    };

    use_effect_with_deps(
        |deps| {
            let EventDependents {
                target,
                event_type,
                callback,
            } = deps.clone();

            let listener = EventListener::new(&target, event_type, move |e| {
                callback.emit(e.clone());
            });

            move || {
                drop(listener);
            }
        },
        deps,
    );
}"#
        ),
        p(vec![
            text("接下来我们为"),
            code("use_subscribe"),
            text("添加实际逻辑。"),
        ]),
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
        p(vec![text(
            "现在我们可以使用自定\\
             u{4e49}钩子了："
        )]),
        code_block(
            "rust",
            r#"#[function_component(ShowMessages)]
pub fn show_messages() -> Html {
    let state = use_subscribe();
    let output = state.borrow().deref().into_iter().map(|it| html! { <p>{ it }</p> });

    html! { <div>{ for output }</div> }
}"#
        ),
        p(vec![
            text(
                "需要特别注意的是创建\\
                 u{81ea}定义钩子时"
            ),
            code("use_hook"),
            text(
                "不是必须的，它们只是\\
                 u{7528}来包含其他钩子。通\\
                 u{5e38}应避免使用"
            ),
            code("use_hook"),
            text("。"),
        ]),
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
        ),
    ])
);
