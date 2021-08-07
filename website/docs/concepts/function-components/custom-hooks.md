---
title: "Custom Hooks"
description: "Defining your own Hooks "
---

## Defining custom Hooks

Component's stateful logic can be extracted into usable function by creating custom Hooks. 

Consider that we have a component which subscribes to an agent and displays the messages sent to it.
```rust
#[function_component(ShowMessages)]
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
}
```

There's one problem with this code: the logic can't be reused by another component.
If we build another component which keeps track of the messages, instead of copying the code we can move the logic into a custom hook.

We'll start by creating a new function called `use_subscribe`.
The `use_` prefix conventionally denotes that a function is a hook.
This function will take no arguments and return `Rc<RefCell<Vec<String>>>`.
```rust
fn use_subscribe() -> Rc<RefCell<Vec<String>>> {
    todo!()
}
```

This is a simple hook which can be created by combining other hooks. For this example, we'll two pre-defined hooks. 
We'll use `use_state` hook to store the `Vec` for messages, so they persist between component re-renders.
We'll also use `use_effect` to subscribe to the `EventBus` `Agent` so the subscription can be tied to component's lifecycle. 

```rust
fn use_subscribe() -> Rc<Vec<String>> {
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
}
```

Although this approach works in almost all cases, it can't be used to write primitive hooks like the pre-defined hooks we've been using already 

### Writing primitive hooks

`use_hook` function is used to write such hooks. View the docs on [docs.rs](https://docs.rs/yew/0.18.0/yew-functional/use_hook.html) for the documentation
and `hooks` directory to see implementations of pre-defined hooks.
