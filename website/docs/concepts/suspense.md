---
title: "Suspense"
description: "Suspense for data fetching"
---

Suspense is a way to suspend component rendering whilst waiting a task
to complete and a fallback (placeholder) UI is shown in the meanwhile.

It can be used to fetch data from server, wait for tasks to be completed
by an agent, or other background asynchronous task.

Before suspense, data fetching usually happens after (Fetch-on-render) or before
component rendering (Fetch-then-render).

### Render-as-You-Fetch

Suspense enables a new approach that allows components to initiate data request
during the rendering process. When a component initiates a data request,
the rendering process will become suspended and a fallback UI will be
shown until the request is completed.

The recommended way to use suspense is with hooks.

```rust, ignore
use yew::prelude::*;

#[function_component(Content)]
fn content() -> Html {
    let user = use_user()?;

    html! {<div>{"Hello, "}{&user.name}</div>}
}

#[function_component(App)]
fn app() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Content />
        </Suspense>
    }
}
```

In the above example, the `use_user` hook will suspend the component
rendering while the user is loading and a `Loading...` placeholder will
be shown until `user` is loaded.

To define a hook that suspends a component rendering, it needs to return
a `SuspensionResult<T>`. When the component needs to be suspended, the
hook should return a `Err(Suspension)` and users should unwrap it with
`?` in which it will be converted into `Html`.

```rust, ignore
use yew::prelude::*;
use yew::suspense::Suspension;

struct User {
    name: String,
}

fn use_user() -> SuspensionResult<User> {
    match load_user() {
        // If a user is loaded, then we return it as Ok(user).
        Some(m) => Ok(m),
        None => {
            // When user is still loading, then we create a `Suspension`
            // and call `SuspensionHandle::resume` when data loading
            // completes, the component will be re-rendered
            // automatically.
            let (s, handle) = Suspension::new();
            on_load_user_complete(move || {handle.resume();});
            Err(s)
        },
    }
}
```

### Use Suspense in Struct Components

Whilst it's possible to suspend a struct component, it's behaviour is
not well defined and not recommended. You should consider using function
components instead when using `<Suspense />`.
