---
title: Callbacks
description: ComponentLink and Callbacks
---

The component "link" is the mechanism through which components are able to create callbacks and update themselves.

## ComponentLink API

### send_message

Sends a message to the component.
Messages are handled by the `update` method which determines whether the component should re-render.

### send_message_batch

Sends multiple messages to the component at the same time.
This is similar to `send_message` but if any of the messages cause the `update` method to return `true`,
the component will re-render after all messages in the batch have been processed.

If the given vector is empty, this function doesn't do anything.

### callback

Create a callback that will send a message to the component when it is executed.
Under the hood, it will call `send_message` with the message returned by the provided closure.

There is a different method called `callback_once` which accepts a `FnOnce` instead of a `Fn`.
You should use this with care though, as the resulting callback will panic if executed more than once.

```rust
// Create a callback that accepts some text and sends it to the component as the `Msg::Text` message variant.
let cb = link.callback(|text: String| Msg::Text(text));

// The previous line is needlessly verbose to make it clearer.
// It can be simplified it to this:
let cb = link.callback(Msg::Text);

// Will send `Msg::Text("Hello World!")` to the component.
cb.emit("Hello World!".to_owned());
```

### batch_callback

Create a callback that will send a batch of messages to the component when it is executed.
The difference to `callback` is that the closure passed to this method doesn't have to return a message.
Instead, the closure can return either `Vec<Msg>` or `Option<Msg>` where `Msg` is the component's message type.

`Vec<Msg>` is treated as a batch of messages and uses `send_message_batch` under the hood.

`Option<Msg>` calls `send_message` if it is `Some`. If the value is `None`, nothing happens.
This can be used in cases where, depending on the situation, an update isn't required.

This is achieved using the `SendAsMessage` trait which is only implemented for these types.
You can implement `SendAsMessage` for your own types which allows you to use them in `batch_callback`.

Like `callback`, this method also has a `FnOnce` counterpart, `batch_callback_once`.
The same restrictions apply as for `callback_once`.

## Callbacks

_\(This might need its own short page.\)_

Callbacks are used to communicate with services, agents, and parent components within Yew.
Internally their type is just `Fn` wrapped in `Rc` to allow them to be cloned.

They have an `emit` function that takes their `<IN>` type as an argument and converts that to a message expected by its destination. If a callback from a parent is provided in props to a child component, the child can call `emit` on the callback in its `update` lifecycle hook to send a message back to its parent. Closures or Functions provided as props inside the `html!` macro are automatically converted to Callbacks.

A simple use of a callback might look something like this:

```rust
let onclick = self.link.callback(|_| Msg::Clicked);
html! {
    <button onclick=onclick>{ "Click" }</button>
}
```

The function passed to `callback` must always take a parameter. For example, the `onclick` handler requires a function which takes a parameter of type `MouseEvent`. The handler can then decide what kind of message should be sent to the component. This message is scheduled for the next update loop unconditionally.

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

## Relevant examples
- [Counter](https://github.com/yewstack/yew/tree/master/examples/counter)
- [Timer](https://github.com/yewstack/yew/tree/master/examples/timer)
