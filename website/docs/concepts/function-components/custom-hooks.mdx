---
title: "Custom Hooks"
description: "Defining your own Hooks "
---

## Defining custom Hooks

Component's stateful logic can be extracted into usable function by creating custom Hooks

Consider that we wish to create an event listener that listens to an event on the `window`
object.

```rust
use yew::prelude::*;
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
}
```

There's one problem with this code: the logic can't be reused by another component.
If we build another component which keeps track of the an event,
instead of copying the code we can move the logic into a custom hook.

We'll start by creating a new function called `use_event`.
The `use_` prefix conventionally denotes that a function is a hook.
This function will take an event target, a event type and a callback.
```rust
use web_sys::{Event, EventTarget};
use std::borrow::Cow;
use gloo::events::EventListener;

pub fn use_event<E, F>(target: &EventTarget, event_type: E, callback: F)
where
    E: Into<Cow<'static, str>>,
    F: Fn(&Event) + 'static,
{
    todo!()
}
```

This is a simple hook which can be created by using built-in hooks. For this example, we'll use the `use_effect_with_deps` hook,
which subscribes to the dependencies so an event listener can be recreated when hook arguments change.

```rust
use yew::prelude::*;
use web_sys::{Event, EventTarget};
use std::borrow::Cow;
use std::rc::Rc;
use gloo::events::EventListener;

pub fn use_event<E, F>(target: &EventTarget, event_type: E, callback: F)
where
    E: Into<Cow<'static, str>>,
    F: Fn(&Event) + 'static,
{
    #[derive(Clone)]
    struct EventDependents {
        target: EventTarget,
        event_type: Cow<'static, str>,
        callback: Rc<dyn Fn(&Event)>,
    }

    #[allow(clippy::vtable_address_comparisons)]
    impl PartialEq for EventDependents {
        fn eq(&self, rhs: &Self) -> bool {
            self.target == rhs.target
                && self.event_type == rhs.event_type
                && Rc::ptr_eq(&self.callback, &rhs.callback)
        }
    }

    let deps = EventDependents {
        target: target.clone(),
        event_type: event_type.into(),
        callback: Rc::new(callback) as Rc<dyn Fn(&Event)>,
    };

    use_effect_with_deps(
        |deps| {
            let EventDependents {
                target,
                event_type,
                callback,
            } = deps.clone();

            let listener = EventListener::new(&target, event_type, move |e| {
                callback(e);
            });

            move || {
                drop(listener);
            }
        },
        deps,
    );
}
```

Although this approach works in almost all cases, it can't be used to write primitive hooks like the pre-defined hooks we've been using already.

View the docs on [docs.rs](https://docs.rs/yew) for documentation and `hooks` directory to see implementations of pre-defined hooks.
