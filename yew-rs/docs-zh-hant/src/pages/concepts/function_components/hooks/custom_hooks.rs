pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["定義自訂 Hooks"],
        p!["元件的有狀態邏輯可以透過建立自訂 Hooks 來提取為可重複使用的函數。"],
        p![
            "假設我們希望建立一個事件監聽器，監聽 ",
            code("window"),
            " 物件上的事件。",
        ],
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
        p!["這段程式碼有一個問題：邏輯無法被另一個元件重複使用。\
            如果我們建立另一個監聽不同事件的元件，而不是複製程式碼，我們可以將邏輯移入自訂 hook。"],
        p![
            "我們將首先建立一個名為 ",
            code("use_event"),
            " 的新函數。 ",
            code("use_"),
            " 前綴表示函數是一個 hook。此函數將接受一個事件目標、一個事件類型和一個回呼。所有 \
             hook 必須在其函數定義上標記為 ",
            code("#[hook]"),
            "。",
        ],
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
        p![
            "這個簡單的 hook 可以透過組合內建 hook 來創建。在本例中，我們將使用 ",
            code("use_effect_with"),
            " hook，因此當 hook 參數變更時，可以重新建立事件監聽器。",
        ],
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
        p![
            "儘管這種方法在幾乎所有情況下都有效，但它無法用於編寫像我們已經使用的預定義 hook \
             那樣的基本 hook。"
        ],
        p![
            "查看 ",
            link!("https://docs.rs/yew", "docs.rs"),
            " 上的文件以及 ",
            code("hooks"),
            " 目錄，查看預先定義 hook 的實作。",
        ],
    ])
}

crate::doc_page!(
    "自訂 Hooks",
    "/zh-Hant/docs/concepts/function-components/hooks/custom-hooks",
    page_content()
);
