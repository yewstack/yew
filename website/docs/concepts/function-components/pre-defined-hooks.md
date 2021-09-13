---
title: "Pre-defined Hooks"
description: "The pre-defined Hooks that Yew comes with "
---

## `use_state`

`use_state` is used to manage state in a function component.
It returns a `UseState` object which `Deref`s to the current value 
and provides a `set` method to update the value.

The hook takes a function as input which determines the initial state.
This value remains up-to-date on subsequent renders.

### Example

```rust
use yew::{Callback, function_component, html, use_state};

#[function_component(UseState)]
fn state() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };


    html! {
        <div>
            <button {onclick}>{ "Increment value" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { *counter }
            </p>
        </div>
    }
}
```

## `use_ref`
`use_ref` is used for obtaining an immutable reference to a value.
Its state persists across renders.

`use_ref` can be useful for keeping things in scope for the lifetime of the component, so long as 
you don't store a clone of the resulting `Rc` anywhere that outlives the component.

If you need a mutable reference, consider using [`use_mut_ref`](#use_mut_ref).
If you need the component to be re-rendered on state change, consider using [`use_state`](#use_state).

```rust
// EventBus is an implementation of yew_agent::Agent
use website_test::agents::EventBus;
use yew::{function_component, html, use_ref, use_state, Callback};
use yew_agent::Bridged;

#[function_component(UseRef)]
fn ref_hook() -> Html { 
    let greeting = use_state(|| "No one has greeted me yet!".to_owned());

    {
        let greeting = greeting.clone();
        use_ref(|| EventBus::bridge(Callback::from(move |msg| {
            greeting.set(msg);
        })));
    }

    html! {
        <div>
            <span>{ (*greeting).clone() }</span>
        </div>
    }
}
``` 

## `use_mut_ref`
`use_mut_ref` is used for obtaining a mutable reference to a value.
Its state persists across renders.

It is important to note that you do not get notified of state changes.
If you need the component to be re-rendered on state change, consider using [`use_state`](#use_state).

### Example

```rust
use web_sys::HtmlInputElement;
use yew::{
    events::Event,
    function_component, html, use_mut_ref, use_state,
    Callback, TargetCast,
};

#[function_component(UseMutRef)]
fn mut_ref_hook() -> Html {
    let message = use_state(|| "".to_string());
    let message_count = use_mut_ref(|| 0);

    let onclick = Callback::from(move |_| {
        let window = yew::utils::window();

        if *message_count.borrow_mut() > 3 {
            window.alert_with_message("Message limit reached").unwrap();
        } else {
            *message_count.borrow_mut() += 1;
            window.alert_with_message("Message sent").unwrap();
        }
    });

    let onchange = {
        let message = message.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            message.set(input.value());
        })
    };

    html! {
        <div>
            <input {onchange} value={(*message).clone()} />
            <button {onclick}>{ "Send" }</button>
        </div>
    }
}
```

## `use_node_ref`
`use_node_ref` is used for obtaining a `NodeRef` that persists across renders.

When conditionally rendering elements you can use `NodeRef` in conjunction with `use_effect_with_deps`
to perform actions each time an element is rendered and just before its going to be removed from the
DOM.

### Example

```rust
use web_sys::HtmlInputElement;
use yew::{
    function_component, functional::*, html,
    NodeRef
};

#[function_component(UseRef)]
pub fn ref_hook() -> Html {
    let input_ref = use_node_ref();
    let value = use_state(|| 25f64);

    let onclick = {
        let input_ref = input_ref.clone();
        let value = value.clone();
        move |_| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                value.set(*value + input.value_as_number());
            }
        }
    };

    html! {
        <div>
            <input ref={input_ref} type="number" />
            <button {onclick}>{ format!("Add input to {}", *value) }</button>
        </div>
    }
}
```

## `use_reducer`

`use_reducer` is an alternative to [`use_state`](#use_state). It is used to handle component's state and is used
when complex actions needs to be performed on said state.

It accepts a reducer function and initial state and returns `Rc` pointing to the state, and a dispatch function.
The dispatch function takes one argument of type `Action`. When called, the action and current value
are passed to the reducer function which computes a new state which is returned,
and the component is re-rendered.

For lazy initialization, consider using [`use_reducer_with_init`](#use_reducer_with_init) instead.

### Example

```rust
use std::rc::Rc;
use yew::{function_component, html, use_reducer, Callback};

#[function_component(UseReducer)]
fn reducer() -> Html {
    /// reducer's Action
    enum Action {
        Double,
        Square,
    }

    /// reducer's State
    struct CounterState {
        counter: i32,
    }

    let counter = use_reducer(
        // the reducer function
        |prev: Rc<CounterState>, action: Action| CounterState {
            counter: match action {
                Action::Double => prev.counter * 2,
                Action::Square => prev.counter * prev.counter,
            },
        },
        // initial state
        CounterState { counter: 1 },
    );

    let double_onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.dispatch(Action::Double))
    };
    let square_onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.dispatch(Action::Square))
    };

    html! {
        <>
            <div id="result">{ counter.counter }</div>

            <button onclick={double_onclick}>{ "Double" }</button>
            <button onclick={square_onclick}>{ "Square" }</button>
        </>
    }
}
```

### `use_reducer_with_init`
`use_reducer` but with init argument. The Hook is passed the initial state
which is then passed down to `init` function which initializes the state and returns it.
The hook then returns this state.

This is useful for lazy initialization where it is beneficial not to perform expensive
computation up-front.

```rust
use std::rc::Rc;
use yew::{function_component, use_reducer_with_init, html};

#[function_component(ReducerWithInit)]
fn reducer_with_init() -> Html {

    /// reducer's State
    struct CounterState {
        counter: i32,
    }

    let counter = use_reducer_with_init(
        // reducer function
        |prev: Rc<CounterState>, action: i32| CounterState {
            counter: prev.counter + action,
        },
        0, // initial value
        |initial: i32| CounterState { // init method
            counter: initial + 10,
        },
    );

    html! {
        <>
            <div id="result">{ counter.counter }</div>
        </>
    }
}
```

## `use_effect`

`use_effect` is used for hooking into the component's lifecycle. 
Similar to `rendered` from the `Component` trait, 
`use_effect` takes a function which is called after the render finishes.

The input function has to return a closure, the destructor, which is called when the component is destroyed.
The destructor can be used to clean up the effects introduced and it can take ownership of values to delay dropping them until the component is destroyed.

### Example

```rust
use yew::{Callback, function_component, html, use_effect, use_state};

#[function_component(UseEffect)]
fn effect() -> Html {
    let counter = use_state(|| 0);

    {
        let counter = counter.clone();
        use_effect(move || {
            // Make a call to DOM API after component is rendered
            yew::utils::document().set_title(&format!("You clicked {} times", *counter));
    
            // Perform the cleanup
            || yew::utils::document().set_title("You clicked 0 times")
        });
    }    
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    html! {
        <button {onclick}>{ format!("Increment to {}", *counter) }</button>
    }
}
```

### `use_effect_with_deps`

Sometimes, it's needed to manually define dependencies for [`use_effect`](#use_effect). In such cases, we use `use_effect_with_deps`.
```rust ,no_run
use yew::use_effect_with_deps;

use_effect_with_deps(
    move |_| {
        // ...
        || ()
    },
    (), // dependents
);
```

**Note**: `dependents` must implement `PartialEq`.

## `use_context`

`use_context` is used for consuming [contexts](../contexts.md) in function components. 


### Example

```rust
use yew::{ContextProvider, function_component, html, use_context, use_state};


/// App theme
#[derive(Clone, Debug, PartialEq)]
struct Theme {
    foreground: String,
    background: String,
}

/// Main component 
#[function_component(App)]
pub fn app() -> Html {
    let ctx = use_state(|| Theme {
        foreground: "#000000".to_owned(),
        background: "#eeeeee".to_owned(),
    });

    html! {
        // `ctx` is type `Rc<UseStateHandle<Theme>>` while we need `Theme`
        // so we deref it.
        // It derefs to `&Theme`, hence the clone
        <ContextProvider<Theme> context={(*ctx).clone()}>
            // Every child here and their children will have access to this context.
            <Toolbar />
        </ContextProvider<Theme>>
    }
}

/// The toolbar.
/// This component has access to the context
#[function_component(Toolbar)]
pub fn toolbar() -> Html {
    html! {
        <div>
            <ThemedButton />
        </div>
    }
}

/// Button placed in `Toolbar`.
/// As this component is a child of `ThemeContextProvider` in the component tree, it also has access to the context.
#[function_component(ThemedButton)]
pub fn themed_button() -> Html {
    let theme = use_context::<Theme>().expect("no ctx found");

    html! {
        <button style={format!("background: {}; color: {};", theme.background, theme.foreground)}>
            { "Click me!" }
        </button>
    }
}
```
