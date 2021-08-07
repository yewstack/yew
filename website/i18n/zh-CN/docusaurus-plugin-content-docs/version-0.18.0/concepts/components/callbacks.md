---
title: 回调（Callbacks）
description: ComponentLink 和 Callbacks.
---

组件“link”是一种机制，通过该机制，组件可以注册回调并自行更新。

## ComponentLink API

### send_message

向组件发送消息。消息将由`update`方法处理来确定组件是否应重新渲染。

### send_message_batch

同时向组件发送多条消息。这与`send_message`类似，但如果任何消息导致`update`方法返回`true` ，则组件将在处理完所有消息后重新渲染。

如果给定的vector为空，则此函数不执行任何操作。

### callback

创建一个回调，该回调将在执行时向组件发送消息。 实际上，它会 将闭包返回的结果传递给`send_message` 。

有一种不同的方法叫做`callback_once` ，它接受`FnOnce`而不是`Fn` 。但是，您应该小心使用它，因为如果这个方法执行多次，其回调将会引发panic。

```rust
// 创建一个回调来接收文本，并且将文本作为`Msg::Text`消息变量发送给组件。
let cb = link.callback(|text: String| Msg::Text(text));

// 前面那行仅仅是为了清楚的演示
// 它可以简化成这样：
let cb = link.callback(Msg::Text);

// 将会发送`Msg::Text("Hello World!")`到组件。
cb.emit("Hello World!".to_owned());
```

### batch_callback

创建一个回调，该回调将在执行时向组件发送一批消息。 该方法与`callback`的不同之处在于，传递给此方法的闭包不必返回消息。作为替代，闭包可以返回`Vec<Msg>`或`Option<Msg>` ，其中`Msg`是组件的消息类型。

`Vec<Msg>`被视为一批消息，并在底层调用`send_message_batch`发送。

`Option<Msg>`会在值为`Some`时调用`send_message` 。当你不需要更新时，可以将值设置为`None` ，此时什么都不会发生。

这是通过使用`SendAsMessage`特性来实现的。您可以为你自己的类型实现`SendAsMessage` ，这样您就可以在`batch_callback`中使用它们。

和`callback`一样，这个方法也有一个`FnOnce`对应实现，那就是 `batch_callback_once` 。它和`callback_once`的限制条件也是相同的。

## Callbacks

*（这部分可能会独立成为一小章。）*

Callbacks 用于与 Yew 中的 services，agents 和父组件进行通信。它们仅仅是个 `Fn`，并由`Rc` 包裹，这将允许他们被克隆。

它们有一个 `emit` 函数，该函数将它的`<IN>` 类型作为参数并将其转换为目标所期望的消息。如果一个回调从父组件中通过 props 提供给子组件，则子组件可以在其 `update`生命周期钩子中对该回调调用`emit`，以将消息发送回父组件。在`html!` 宏内被提供作为 props 的闭包或函数会自动转换为 Callbacks。

一个简单的回调示例是这样的：

```rust
let onclick = self.link.callback(|_| Msg::Clicked);
html! {
    <button onclick=onclick>{ "Click" }</button>
}
```

传递给`callback`的函数必须始终带有一个参数。例如， `onclick`处理程序需要一个函数，该函数采用`MouseEvent`类型的参数。然后处理程序可以决定应该向组件发送什么样的消息。该消息会被无条件地安排在下一个更新循环中。

如果您需要一个可能会引起更新的回调，请使用`batch_callback` 。

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

## 相关例子

- [Counter](https://github.com/yewstack/yew/tree/master/examples/counter)
- [Timer](https://github.com/yewstack/yew/tree/master/examples/timer)
