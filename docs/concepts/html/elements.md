---
title: Elements
description: Both HTML and SVG elements are supported
---

## Dynamic tag names

When building a higher-order component you might find yourself in a situation where the element's tag name isn't static.
For example, you might have a `Title` component which can render anything from `h1` to `h6` depending on a level prop.
Instead of having to use a big match expression, Yew allows you to set the tag name dynamically
using `@{name}` where `name` can be any expression that returns a string.

```rust
let level = 5;
let text = "Hello World!".to_owned()

html! {
    <@{format!("h{}", level)} class="title">{ content }</@>
}
```

## Optional attributes for HTML elements

Most HTML attributes can be marked as optional by placing a `?` in front of
the `=` sign. This makes them accept the same type of value as otherwise, but
wrapped in an `Option<T>`:

```rust
let maybe_id = Some("foobar");

html! {
    <div id?=maybe_id></div>
}
```

If the attribute is set to `None`, the attribute won't be set in the DOM.

## Listeners

Listener attributes need to be passed a `Callback` which is a wrapper around a closure. How you create your callback depends on how you wish your app to react to a listener event:

<!--DOCUSAURUS_CODE_TABS-->
<!--Component handler-->

```rust
struct MyComponent {
    link: ComponentLink<Self>,
}

enum Msg {
    Click,
}

impl Component for MyComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        MyComponent { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                // Handle Click
            }
        }
    }

    fn view(&self) -> Html {
        // Create a callback from a component link to handle it in a component
        let click_callback = self.link.callback(|_: ClickEvent| Msg::Click);
        html! {
            <button onclick=click_callback>
                { "Click me!" }
            </button>
        }
    }
}
```

<!--Agent Handler-->

```rust
struct MyComponent {
    worker: Dispatcher<MyWorker>,
}

impl Component for MyComponent {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        MyComponent {
            worker: MyWorker::dispatcher()
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // Create a callback from a worker to handle it in another context
        let click_callback = self.worker.callback(|_: ClickEvent| WorkerMsg::Process);
        html! {
            <button onclick=click_callback>
                { "Click me!" }
            </button>
        }
    }
}
```

<!--Other Cases-->

```rust
struct MyComponent;

impl Component for MyComponent {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        MyComponent
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // Create an ephemeral callback
        let click_callback = Callback::from(|| {
            ConsoleService::new().log("clicked!");
        });

        html! {
            <button onclick=click_callback>
                { "Click me!" }
            </button>
        }
    }
}
```

<!--END_DOCUSAURUS_CODE_TABS-->

## Event Types

:::tip
All the event types mentioned in the following table are re-exported under `yew::events`.
Using the types from `yew::events` makes it easier to ensure version compatibility than
if you were to manually include `web-sys` as a dependency in your crate because you won't
end up using a version which conflicts with the version that Yew specifies.
:::

| Event name                  | `web_sys` Event Type                                                                  |
| --------------------------- | ------------------------------------------------------------------------------------- |
| `onabort`                   | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onauxclick`                | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           |
| `onblur`                    | [FocusEvent](https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html)           |
| `oncancel`                  | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `oncanplay`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `oncanplaythrough`          | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onchange`                  | [ChangeData](https://docs.rs/yew/latest/yew/events/enum.ChangeData.html)              |
| `onclick`                   | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           |
| `onclose`                   | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `oncontextmenu`             | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           |
| `oncuechange`               | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `ondblclick`                | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           |
| `ondrag`                    | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html)             |
| `ondragend`                 | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html)             |
| `ondragenter`               | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html)             |
| `ondragexit`                | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html)             |
| `ondragleave`               | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.htmk)             |
| `ondragover`                | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html)             |
| `ondragstart`               | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html)             |
| `ondrop`                    | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html)             |
| `ondurationchange`          | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onemptied`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onended`                   | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onerror`                   | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onfocus`                   | [FocusEvent](https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html)           |
| `onformdata`                | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `oninput`                   | [InputData](https://docs.rs/yew/latest/yew/events/struct.InputData.html)              |
| `oninvalid`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onkeydown`                 | [KeyboardEvent](https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html)     |
| `onkeypress`                | [KeyboardEvent](https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html)     |
| `onkeyup`                   | [KeyboardEvent](https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html)     |
| `onload`                    | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onloadeddata`              | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onloadedmetadata`          | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onloadstart`               | [ProgressEvent](https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html)     |
| `onmousedown`               | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           |
| `onmouseenter`              | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           |
| `onmouseleave`              | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           |
| `onmousemove`               | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           |
| `onmouseout`                | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           |
| `onmouseover`               | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           |
| `onmouseup`                 | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           |
| `onpause`                   | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onplay`                    | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onplaying`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onprogress`                | [ProgressEvent](https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html)     |
| `onratechange`              | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onreset`                   | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onresize`                  | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onscroll`                  | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onsecuritypolicyviolation` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onseeked`                  | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onseeking`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onselect`                  | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onslotchange`              | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onstalled`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onsubmit`                  | [FocusEvent](https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html)           |
| `onsuspend`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `ontimeupdate`              | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `ontoggle`                  | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onvolumechange`            | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onwaiting`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onwheel`                   | [WheelEvent](https://docs.rs/web-sys/latest/web_sys/struct.WheelEvent.html)           |
| `oncopy`                    | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `oncut`                     | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onpaste`                   | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onanimationcancel`         | [AnimationEvent](https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html)   |
| `onanimationend`            | [AnimationEvent](https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html)   |
| `onanimationiteration`      | [AnimationEvent](https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html)   |
| `onanimationstart`          | [AnimationEvent](https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html)   |
| `ongotpointercapture`       | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       |
| `onloadend`                 | [ProgressEvent](https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html)     |
| `onlostpointercapture`      | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       |
| `onpointercancel`           | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       |
| `onpointerdown`             | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       |
| `onpointerenter`            | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       |
| `onpointerleave`            | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       |
| `onpointerlockchange`       | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onpointerlockerror`        | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onpointermove`             | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       |
| `onpointerout`              | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       |
| `onpointerover`             | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       |
| `onpointerup`               | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       |
| `onselectionchange`         | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onselectstart`             | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `onshow`                    | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `ontouchcancel`             | [TouchEvent](https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html)           |
| `ontouchend`                | [TouchEvent](https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html)           |
| `ontouchmove`               | [TouchEvent](https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html)           |
| `ontouchstart`              | [TouchEvent](https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html)           |
| `ontransitioncancel`        | [TransitionEvent](https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html) |
| `ontransitionend`           | [TransitionEvent](https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html) |
| `ontransitionrun`           | [TransitionEvent](https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html) |
| `ontransitionstart`         | [TransitionEvent](https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html) |
