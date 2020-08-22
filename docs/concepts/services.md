---
title: Services
sidebar_label: Overview
description: Yew's glue to browser APIs
---

Services provide a straightforward way to integrate browser APIs into Yew applications. They act as
abstractions over lower-level one-to-one bindings to browser APIs provided by `web-sys` or `stdweb`.

Yew provides a number of services. You can use these services to:
* create WebSocket connections
* launch HTTP requests to web servers
* perform tasks at a specified interval
* subscribe to browser events (e.g. page resizes)
* save data to persistent browser storage
* [and more (this links to the API documentation where you can find a complete list of all available
services)](https://docs.rs/yew/*/yew/services/index.html).

All of Yew's services are contained in the `yew::services` module. All services provide a struct
named `<service name>Service` which is used to construct instances of the service. Services which
perform asynchronous tasks (e.g. the fetch service) provide a struct implementing the `Task` trait.
If you use a method belonging to a service which returns an item implementing `Task` you must ensure
that the task isn't dropped before your component! This is so important that we're going to place it 
in a big red box below.

:::danger
`Task`s shouldn't be dropped before the components they are created within are dropped.
:::

Because of the way Rust handles memory this code won't work.

```rust
use yew::prelude::*;
struct App {
}
impl Component for App {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let task = yew::services::IntervalService::spawn(
            std::time::Duration::from_secs(10), 
            link.callback(|| ())
        );
        Self {}
        // `task` is dropped here
    }
    // other methods emitted
}
```

This code, however, will work fine. This is because here the instance of `Task` will be kept alive
until the task can be completed.

```rust
use yew::prelude::*;
struct App {
    interval_task: yew::services::interval::IntervalTask
}
impl Component for App {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            interval_task: yew::services::IntervalService::spawn(
                std::time::Duration::from_secs(10), 
                link.callback(|| ())
            )
        }
    }
    // other methods emitted
}
```
If you haven't stored a `Task` somewhere to stop it from being immediately aborted the compiler will 
warn you!
