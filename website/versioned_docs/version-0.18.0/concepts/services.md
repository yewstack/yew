---
title: "Services"
sidebar_label: Overview
description: "Yew's glue to browser APIs"
---

:::caution
Yew services will be removed in the next version of Yew `v0.19`.

Recommended replacements:
- `ConsoleService` - [weblog](https://crates.io/crates/weblog) or [gloo_console](https://crates.io/crates/gloo-console)
- `DialogService` - [gloo_dialogs](https://crates.io/crates/gloo-dialogs)
- `IntervalService` - [gloo-timers](https://crates.io/crates/gloo-timers)
- `KeyboardService` - `onkeydown` / `onkeypress` / `onkeyup` like so:
    ```rust
    let callback = Callback::from(|e| {
        e.prevent_default();
        todo!("use `e`, like you would in service methods.");
    });
    html! {
        <input onkeydown={callback} />
    }
    ```
- `ResizeService` - use `EventListener` from [gloo_events](https://crates.io/crates/gloo-events)
to attach the event listener instead.
- `StorageService` - [gloo-storage](https://crates.io/crates/gloo-storage)
- `TimeoutService` - [gloo-timers](https://crates.io/crates/gloo-timers)
- `WebSocketService` - [wasm-sockets](https://crates.io/crates/wasm-sockets) or [reqwasm](https://crates.io/crates/reqwasm)
- `FetchService` - [reqwest](https://crates.io/crates/reqwest) or [reqwasm](https://crates.io/crates/reqwasm)

:::
