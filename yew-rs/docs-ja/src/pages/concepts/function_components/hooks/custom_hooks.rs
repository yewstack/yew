pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![text("カスタムフックの定義")]),
        p(vec![text(
            "コンポーネントのステートフルなロジックは、\
             カスタムフックを作成することで再利用可能な関数として抽出できます。",
        )]),
        p(vec![
            text("例えば、"),
            code("window"),
            text(" オブジェクト上のイベントをリッスンするイベントリスナーを作成したいとします。"),
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
            "このコードには問題があります。ロジックは他のコンポーネントで再利用できません。\
             異なるイベントをリッスンする別のコンポーネントを作成する場合、\
             コードをコピーするのではなく、ロジックをカスタムフックに移すことができます。",
        )]),
        p(vec![
            text("まず、"),
            code("use_event"),
            text(" という新しい関数を作成します。"),
            code("use_"),
            text(
                " プレフィックスは関数がフックであることを示します。この関数はイベントターゲット、\
                 イベントタイプ、およびコールバックを受け取ります。すべてのフックはその関数定義に ",
            ),
            code("#[hook]"),
            text(" とマークする必要があります。"),
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
            text(
                "このシンプルなフックは、組み込みのフックを組み合わせることで作成できます。\
                 この例では、",
            ),
            code("use_effect_with"),
            text(
                " フックを使用します。これにより、\
                 フックのパラメータが変更されたときにイベントリスナーを再作成できます。",
            ),
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
            "この方法はほとんどすべてのケースで有効ですが、\
             私たちがすでに使用しているような基本的なフックを作成するためには使用できません。",
        )]),
        p(vec![
            link("https://docs.rs/yew", vec![text("docs.rs")]),
            text(" 上のドキュメントや "),
            code("hooks"),
            text(" ディレクトリを参照して、事前定義されたフックの実装を確認してください。"),
        ]),
    ])
}

crate::doc_page!(
    "カスタムフック",
    "/ja/docs/concepts/function-components/hooks/custom-hooks",
    page_content()
);
