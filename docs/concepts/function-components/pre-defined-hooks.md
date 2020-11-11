---
title: Pre-defined hooks
description: The pre-defined hooks that Yew comes with 
---

:::note Why do Hooks return `Rc`?

In most cases, you'll be cloning the values returned from the hooks.
As it may be expensive to clone such values, they're `Rc`ed, so they can be cloned relatively cheaply.

Following example shows one of the most common cases which requires cloning the values:

```rust
let (counter, set_counter) = use_state(|| 0);
let onclick = {
    let counter = Rc::clone(&counter);
    // Values must be moved into this closure so in order to be able to use them later on, they must be cloned
    Callback::from(move |_| set_counter(*counter + 1)) 
};

html! {
    // If `counter` wasn't cloned above, it would've been impossible to use it here
    { counter }
}
```
:::

## `use_state`

`use_state` is used to mange state in a function component.
It returns a `Rc` of the stateful value, and a setter function.

Initially, the state is set to the result of the function passed.
This value remains up-to-date on subsequent renders.

The setter function is used to update the value and trigger a re-render.

### Example

```rust
#[function_component(UseState)]
fn state() -> Html {
    let (
        counter, // the returned state
        set_counter // setter to update the state
    ) = use_state(|| 0);
    let onclick = {
        let counter = Rc::clone(&counter);
        Callback::from(move |_| set_counter(*counter + 1))
    };

    html! {
        <div>
            <button onclick=onclick>{ "Increment value" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { counter }
            </p>
        </div>
    }
}
```

## `use_ref`
`use_ref` is used for obtaining a mutable reference to a stateful value.
Its state persists across renders.

It is important to note that you do not get notified of state changes.
If you need the component to be re-rendered on state change, consider using [`use_state`](#use_state).

### Example

```rust
#[function_component(UseRef)]
fn ref_hook() -> Html {
    let (outer_html, set_outer_html) = use_state(|| "".to_string());

    let node_ref: Rc<RefCell<NodeRef>> = use_ref(|| NodeRef::default());

    let on_click = {
        let node_ref = Rc::clone(&node_ref);

        Callback::from(move |e| {
            let to_set = (*node_ref.borrow().deref())
                .cast::<yew::web_sys::Element>()
                .unwrap()
                .outer_html();
            set_outer_html(to_set)
        })
    };
    html! {
        <>
            <div id="result" ref=(*node_ref.borrow_mut().deref_mut()).clone()>{"Filler"}</div>
            {outer_html}
            <br />
            <button onclick=on_click>{"Refresh"}</button>
        </>
    }
}
```


## `use_reducer`

`use_reducer` is an alternative to [`use_state`](#use_state). It is used to handle component's state and is used
when complex actions needs to be performed on said state.

It accepts a reducer function and initial state and returns `Rc` of the state, and a dispatch function.
The dispatch function takes one argument of `Action`. When called, the action and current value
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

    let (
        counter, // the state
        // function to update the state 
        // as the same suggests, it dispatches the values to the reducer function
        dispatch  
    ) = use_reducer(
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
        let dispatch = Rc::clone(&dispatch);
        Callback::from(move |_| dispatch(Action::Double))
    };
    let square_onclick = Callback::from(move |_| dispatch(Action::Square));

    html! {
        <>
            <div id="result">{counter.counter}</div>

            <button onclick=double_onclick>{"Double"}</button>
            <button onclick=square_onclick>{"Square"}</button>
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
let (counter, dispatch) = use_reducer_with_init(
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
Similar to `rendered` method of `Component` trait, 
`use_effect` takes a function which is called after the render finishes.

The said function returns another function, the destructor function,
which called when the component is destroyed. It can be used to clean up the effects introduced.
This is similar to `destroyed` method of `Component` trait.

### Example

```rust
#[function_component(UseEffect)]
fn effect() -> Html {
    let (counter, set_counter) = use_state(|| 0);

    let counter_one = counter.clone();
    use_effect(move || {
        // Make a call to DOM API after component is rendered
        yew::utils::document().set_title(&format!("You clicked {} times", counter_one));

        // Perform the cleanup
        || yew::utils::document().set_title(&format!("You clicked 0 times"))
    });
    
    let onclick = {
        let counter = Rc::clone(&counter);
        Callback::from(move |_| set_counter(*counter + 1))
    };

    return html! {<>
        <button onclick=onclick>{ format!("Increment to {}", counter) }</button>
    </>};
}
```

### `use_effect_with_deps`

Sometimes, it's needed to manually define dependencies for [`use_effect`](#use_effect). In such cases, we use `use_effect_with_deps`.
```rust
use_effect_with_deps(
    move |_| {
        // ...
        || {}
    },
    (), // dependents
);
```

**Note**: `dependents` must implement `PartialEq`.

## `use_context`

<!-- TODO -->
