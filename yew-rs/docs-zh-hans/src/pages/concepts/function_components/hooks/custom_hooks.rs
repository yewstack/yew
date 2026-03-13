pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![text("定义自定义 Hooks")]),
        p(vec![text(
            "组件的有状态逻辑可以通过创建自定义 Hooks 来提取为可重用的函数。",
        )]),
        p(vec![
            text("假设我们希望创建一个事件监听器，监听 "),
            code("window"),
            text(" 对象上的事件。"),
        ]),
        code_block(
            "rust",
            r#"use yew::prelude::*;
use gloo::events::EventListener;
use gloo::utils::window;
use std::mem::drop;


#[component(ShowStorageChanged)]
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
}"#,
        ),
        p(vec![text(
            "这段代码有一个问题：逻辑无法被另一个组件重用。如果我们构建另一个监听不同事件的组件，\
             而不是复制代码，我们可以将逻辑移入自定义 hook。",
        )]),
        p(vec![
            text("我们将首先创建一个名为 "),
            code("use_event"),
            text(" 的新函数。"),
            code("use_"),
            text(
                " 前缀表示函数是一个 hook。此函数将接受一个事件目标、一个事件类型和一个回调。所有 \
                 hook 必须在其函数定义上标记为 ",
            ),
            code("#[hook]"),
            text("。"),
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
}"#,
        ),
        p(vec![
            text("这个简单的 hook 可以通过组合内置 hook 创建。在本例中，我们将使用 "),
            code("use_effect_with"),
            text(" hook，因此当 hook 参数发生变化时，可以重新创建事件监听器。"),
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

    use_effect_with(
        deps,
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
    );
}"#,
        ),
        p(vec![text(
            "尽管这种方法在几乎所有情况下都有效，但它无法用于编写像我们已经使用的预定义 hook \
             那样的基本 hook。",
        )]),
        p(vec![
            text("查看 "),
            link("https://docs.rs/yew", vec![text("docs.rs")]),
            text(" 上的文档以及 "),
            code("hooks"),
            text(" 目录，查看预定义 hook 的实现。"),
        ]),
    ])
}

crate::doc_page!(
    "自定义 Hooks",
    "/zh-Hans/docs/concepts/function-components/hooks/custom-hooks",
    page_content()
);
