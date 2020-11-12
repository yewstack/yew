---
title: Custom Hooks
description: Defining your own Hooks 
---

## Defining custom Hooks

Component's stateful logic can be extracted into usable function by creating custom Gooks. 

Consider that we have a component which subscribes to an agent and displays the messages sent to it.
```rust
#[function_component(ShowMessages)]
pub fn show_messages() -> Html {
    let (state, set_state) = use_state(|| RefCell::new(vec![]));

    {
        let mut state = Rc::clone(&state);
        use_effect(move || {
            let producer = EventBus::bridge(Callback::from(move |msg| {
                (*state).borrow_mut().deref_mut().push(msg);
                set_state((*state).clone())
            }));

            || drop(producer)
        });
    }

    let output = state.borrow().deref().iter().map(|it| html! { <p>{it} </p>}).collect::<Html>();
    html! { <div> {output} </div> }
}
```

There's one problem with this code: if this stateful logic can't be reused. If it is to be used in other component, 
there will be code duplication. This problem can easily be solved by moving this logic to a new hook.

We'll start by creating a new function called `use_subscribe`. The `use_` prefix conventionally denotes this is a hook.
This function will take no arguments and return `Rc<RefCell<Vec<String>>>`.
```rust
fn use_subscribe() -> Rc<RefCell<Vec<String>>> {
    // ...
}
```

The hook's logic goes inside the `use_hook`'s callback.
`use_hook` is the handler function for custom Hooks. It takes in 2 arguments: `hook_runner` and `initial_state_producer`. 

`hook_runner` is where all the hook's logic goes. `use_hook` returns the value returned by this callback.
`hook_runner` itself takes 2 arguments: a mutable reference to the internal state of the hook and `hook_callback`.
`hook_callback` also takes in 2 arguments: a callback and, a bool indicating whether it is run post render of the component.
The callback takes in `internal_state` which is a mutable reference to the instance of the internal state and performs the actual mutations. 
It returns `ShouldRender` bool.
`use_hook`'s second argument of `initial_state_producer` takes in a callback for creating an instance of the internal state.
The internal state is a struct which implements the `Hook` trait.

Now let's create the state struct for our `use_subscribe` hook.
```rust
/// `use_subscribe` internal state
struct UseSubscribeState {
    /// holds all the messages received
    pub messages: Rc<RefCell<Vec<String>>>,
}

impl Hook for UseSubscribeState {}
```

Now we'll modify `use_subscribe` to contain the actual logic.
```rust
fn use_subscribe() -> Rc<RefCell<Vec<String>>> {
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
}
```

We can consume this hook like:
```rust
#[function_component(ShowMessages)]
pub fn show_messages() -> Html {
    let state = use_subscribe();
    let output = state.borrow().deref().into_iter().map(|it| html! { <p>{it} </p>}).collect::<Html>();

    html! { <div> {output} </div> }
}
```
