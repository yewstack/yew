---
title: Callbacks
description: ComponentLink and Callbacks
---
The component "link" is the mechanism through which components are able to register callbacks and update themselves.

## ComponentLink API

### callback

Registers a callback that will send a message to the component's update mechanism when it is executed. Under the hood, it will call `send_message` with the message that is returned by the provided closure. `ComponentLink::callback` takes a `Fn(IN) -> COMP::Message` and returns a `Callback<IN>`.

This method has a version that accepts an `FnOnce` instead, `ComponentLink::callback_once`. You should avoid using this unless necessary, as making sure the callback is only called once has a small overhead.

### send\_message

Sends a message to the component immediately after the current loop finishes, causing another update loop to initiate.

### batch\_callback

Registers a callback that sends a batch of messages at once when it is executed. If any of the messages cause the component to re-render, the component will re-render after all messages in the batch have been processed. The function can take either `Fn(IN) -> Vec<COMP::Message>` or `Fn(IN) -> Option<COMP::Message>`. Returns a `Callback<IN>`.

If the function returns an empty `Vec` or `None`, update *will not* be scheduled. Use this when the callback might not need to do anything.

This method has a version that accepts an `FnOnce` instead, `ComponentLink::batch_callback_once`. You should avoid using this unless necessary, as making sure the callback is only called once has a small overhead.

## Callbacks

_\(This might need its own short page.\)_

Callbacks are used to communicate with services, agents, and parent components within Yew. They are just an `Fn` wrapped by an `Rc` to allow them to be cloned.

They have an `emit` function that takes their `<IN>` type as an argument and converts that to a message expected by its destination. If a callback from a parent is provided in props to a child component, the child can call `emit` on the callback in its `update` lifecycle hook to send a message back to its parent. Closures or Functions provided as props inside the `html!` macro are automatically converted to Callbacks.

Simple usage of callbacks can look something like this:

```rust
let onclick = self.link.callback(|_| Msg::Clicked);
html! {
    <button onclick=onclick>{"Click"}</button>
}
```

The callback is always given some value. For example, the `onclick` handler will pass a `MouseEvent`. The handler can then decide what kind of message the event should resolve to. This message is scheduled for the next update loop unconditionally.

If you need a callback that might not need to cause an update, use `batch_callback`.

```rust
let onkeypress = self.link.batch_callback(|event| {
    if event.key() == "Enter" {
        Some(Msg::Submit)
    } else {
        None
    }
});

html! {
    <input type="text" onkeypress=onkeypress />
}
```

