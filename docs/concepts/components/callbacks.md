---
id: callbacks
title: Callbacks
description: ComponentLink and Callbacks
---
The component "link" is the mechanism through which components are able to register callbacks and update themselves.

## ComponentLink API

### callback

Registers a callback that will send a message to the component's update mechanism when it is executed. Under the hood, it will call `send_self` with the message that is returned by the provided closure. A `Fn(IN) -> Vec<COMP::Message>` is provided and a `Callback<IN>` is returned.

### send\_message

Sends a message to the component immediately after the current loop finishes, causing another update loop to initiate.

### send\_message\_batch

Registers a callback that sends a batch of many messages at once when it is executed. If any of the messages cause the component to re-render, the component will re-render after all messages in the batch have been processed. A `Fn(IN) -> COMP::Message` is provided and a `Callback<IN>` is returned.

## Callbacks

_\(This might need its own short page.\)_

Callbacks are used to communicate with services, agents, and parent components within Yew. They are just a `Fn` wrapped by an `Rc` to allow them to be cloned.

They have an `emit` function that takes their `<IN>` type as an argument and converts that to a message expected by its destination. If a callback from a parent is provided in props to a child component, the child can call `emit` on the callback in its `update` lifecycle hook to send a message back to its parent. Closures or Functions provided as props inside the `html!` macro are automatically converted to Callbacks.

