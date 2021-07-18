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
#[function_component(UseState)]
fn state() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };


    html! {
        <div>
            <button onclick=onclick>{ "Increment value" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { *counter }
            </p>
        </div>
    }
}
```

## `use_ref`
`use_ref` is used for obtaining a mutable reference to a value.
Its state persists across renders.

It is important to note that you do not get notified of state changes.
If you need the component to be re-rendered on state change, consider using [`use_state`](#use_state).

### Example

```rust
#[function_component(UseRef)]
fn ref_hook() -> Html {
    let (message, set_message) = use_state(|| "".to_string());
    let message_count = use_ref(|| 0);

    let onclick = Callback::from(move |e| {
        let window = yew::utils::window();

        if *message_count.borrow_mut() > 3 {
            window.alert_with_message("Message limit reached");
        } else {
            *message_count.borrow_mut() += 1;
            window.alert_with_message("Message sent");
        }
    });

    let onchange = Callback::from(move |e| {
        if let ChangeData::Value(value) = e {
            set_message(value)
        }
    });

    html! {
        <div>
            <input onchange=onchange value=message />
            <button onclick=onclick>{ "Send" }</button>
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
            }
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

            <button onclick=double_onclick>{ "Double" }</button>
            <button onclick=square_onclick>{ "Square" }</button>
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
```

## `use_effect`

`use_effect` is used for hooking into the component's lifecycle. 
Similar to `rendered` from the `Component` trait, 
`use_effect` takes a function which is called after the render finishes.

The input function has to return a closure, the destructor, which is called when the component is destroyed.
The destructor can be used to clean up the effects introduced and it can take ownership of values to delay dropping them until the component is destroyed.

### Example

```rust
#[function_component(UseEffect)]
fn effect() -> Html {
    let (counter, set_counter) = use_state(|| 0);

    {
        let counter = counter.clone();
        use_effect(move || {
            // Make a call to DOM API after component is rendered
            yew::utils::document().set_title(&format!("You clicked {} times", counter));
    
            // Perform the cleanup
            || yew::utils::document().set_title("You clicked 0 times")
        });
    }    
    let onclick = {
        let counter = Rc::clone(&counter);
        Callback::from(move |_| set_counter(*counter + 1))
    };

    html! {
        <button onclick=onclick>{ format!("Increment to {}", counter) }</button>
    }
}
```

### `use_effect_with_deps`

Sometimes, it's needed to manually define dependencies for [`use_effect`](#use_effect). In such cases, we use `use_effect_with_deps`.
```rust
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
        // `ctx` is type `Rc<UseStateHandle<Theme>>` while we need `Theme` so we deref it
        // It derefs to `&Theme`, hence the clone
        <ContextProvider<Theme> context=(*ctx).clone()>
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
        <button style=format!("background: {}; color: {};", theme.background, theme.foreground)>
            { "Click me!" }
        </button>
    }
}
```
