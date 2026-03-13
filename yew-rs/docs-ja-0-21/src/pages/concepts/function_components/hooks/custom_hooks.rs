crate::doc_page!(
    "Custom Hooks",
    "/ja/docs/concepts/function-components/hooks/custom-hooks",
    Content::new(vec![
        h2(vec![text("Defining custom Hooks")]),
        p(vec![text(
            "The stateful logic of a component can be extracted into reusable functions by \
             creating custom Hooks."
        )]),
        p(vec![
            text(
                "Consider that we wish to create an event listener that listens to an event on \
                 the "
            ),
            code("window"),
            text(" object."),
        ]),
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
            "There's one problem with this code: the logic can't be reused by another component. \
             If we build another component that listens to a different event, instead of copying \
             the code, we can move the logic into a custom hook."
        ),]),
        p(vec![
            text("We'll start by creating a new function called "),
            code("use_event"),
            text(". The "),
            code("use_"),
            text(
                " prefix denotes that a function is a hook. This function will take an event \
                 target, an event type, and a callback. All hooks must be marked by "
            ),
            code("#[hook]"),
            text(" on their function definition."),
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
            text(
                "This simple hook can be created by composing built-in hooks. For this example, \
                 we'll use the "
            ),
            code("use_effect_with"),
            text(" hook, so an event listener can be recreated when the hook arguments change."),
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
}"#
        ),
        p(vec![text(
            "Although this approach works in almost all cases, it can't be used to write \
             primitive hooks like the pre-defined hooks we've been using already."
        ),]),
        p(vec![
            text("View the docs on "),
            link("https://docs.rs/yew", vec![text("docs.rs")]),
            text(" for documentation and "),
            code("hooks"),
            text(" directory to see implementations of pre-defined hooks."),
        ]),
    ])
);
