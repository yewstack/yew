---
title: "Elements"
description: "Both HTML and SVG elements are supported"
---

## DOM nodes

There are many reasons why you might want to create or manage DOM nodes manually in Yew, such as
when integrating with JS libraries that can cause conflicts with managed components.

Using `web-sys`, you can create DOM elements and convert them into a `Node` - which can then be 
used as a `Html` value using `VRef`:

```rust
    // ...
    fn view(&self) -> Html {
        use yew::{utils::document, web_sys::{Element, Node}};

        // Create a div element from the document
        let div: Element = document().create_element("div").unwrap();
        // Add content, classes etc.
        div.set_inner_html("Hello, World!");
        // Convert Element into a Node
        let node: Node = div.into();
        // Return that Node as a Html value
        Html::VRef(node)
    }
```

## Dynamic tag names

When building a higher-order component you might find yourself in a situation where the element's tag name isn't static.
For example, you might have a `Title` component which can render anything from `h1` to `h6` depending on a level prop.
Instead of having to use a big match expression, Yew allows you to set the tag name dynamically
using `@{name}` where `name` can be any expression that returns a string.

```rust
let level = 5;
let text = "Hello World!".to_owned()

html! {
    <@{format!("h{}", level)} class="title">{ text }</@>
}
```

## Boolean Attributes 

Some content attributes (e.g checked, hidden, required) are called boolean attributes. In Yew, 
boolean attributes need to be set to a bool value:

```rust
    html! {
        <div hidden=true>
            { "This div is hidden." }
        </div>
    }
```

This will result in **HTML** that's functionally equivalent to this:
```html
    <div hidden>This div is hidden.</div>
```

Setting a boolean attribute to false is equivalent to not using the attribute at all; values from 
boolean expressions can be used:

```rust
    let no = 1 + 1 != 2;

    html! {
        <div hidden=no>
            { "This div is NOT hidden." }
        </div>
    }
```

This will result in the following **HTML**:

```html
    <div>This div is NOT hidden.</div>
```

## Optional attributes for HTML elements

Most HTML attributes can use optional values (`Some(x)` or `None`). This allows us
to omit the attribute if the attribute is marked as optional.

```rust
let maybe_id = Some("foobar");

html! {
    <div id=maybe_id></div>
}
```

If the attribute is set to `None`, the attribute won't be set in the DOM.

Please note that it is also valid to give only the value as properties behave
like `Into<Option<T>>`:

```rust
let id = "foobar";

html! {
    <div id=id></div>
}
```

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

In the following table `web-sys`'s event types should only be used if you're using `yew` with `web-sys`
(this is enabled by default). Use `stdweb`'s event types if you're using the `yew-stdweb` crate. See
[the documentation page about whether to choose `web-sys` or `stdweb`](https://yew.rs/getting-started/choose-web-library) for more information.

:::tip
All the event types mentioned in the following table are re-exported under `yew::events`.
Using the types from `yew::events` makes it easier to ensure version compatibility than if you were to manually include `web-sys` or `stdweb` as dependencies in your crate because you won't end up using a version which conflicts with the version Yew specifies.
:::

| Event name                  | `web_sys` Event Type                                                                  | `stdweb` Event Type                                                                                           |
| --------------------------- | ------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `onabort`                   | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | [ResourceAbortEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ResourceAbortEvent.html)           |
| `onauxclick`                | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           | [AuxClickEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.AuxClickEvent.html)                     |
| `onblur`                    | [FocusEvent](https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html)           | [BlurEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.BlurEvent.html)                             |
| `oncancel`                  | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `oncanplay`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `oncanplaythrough`          | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onchange`                  | [ChangeData](https://docs.rs/yew/latest/yew/events/enum.ChangeData.html)              | [ChangeData](https://docs.rs/yew-stdweb/latest/yew_stdweb/events/enum.ChangeData.html)                        |
| `onclick`                   | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           | [ClickEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ClickEvent.html)                           |
| `onclose`                   | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `oncontextmenu`             | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           | [ContextMenuEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ContextMenuEvent.html)               |
| `oncuechange`               | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `ondblclick`                | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           | [DoubleClickEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DoubleClickEvent.html)               |
| `ondrag`                    | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html)             | [DragEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragEvent.html)                             |
| `ondragend`                 | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html)             | [DragEndEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragEndEvent.html)                       |
| `ondragenter`               | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html)             | [DragEnterEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragEnterEvent.html)                   |
| `ondragexit`                | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html)             | [DragExitEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragExitEvent.html)                     |
| `ondragleave`               | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.htmk)             | [DragLeaveEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragLeaveEvent.html)                   |
| `ondragover`                | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html)             | [DragOverEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragOverEvent.html)                     |
| `ondragstart`               | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html)             | [DragStartEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragStartEvent.html)                   |
| `ondrop`                    | [DragEvent](https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html)             | [DragDropEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.DragDropEvent.html)                     |
| `ondurationchange`          | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onemptied`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onended`                   | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onerror`                   | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | [ResourceErrorEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ResourceErrorEvent.html)           |
| `onfocus`                   | [FocusEvent](https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html)           | [FocusEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.FocusEvent.html)                           |
| `onformdata`                | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `oninput`                   | [InputData](https://docs.rs/yew/latest/yew/events/struct.InputData.html)              | [InputData](https://docs.rs/yew-stdweb/latest/yew_stdweb/events/struct.InputData.html)                        |
| `oninvalid`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onkeydown`                 | [KeyboardEvent](https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html)     | [KeyDownEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.KeyDownEvent.html)                       |
| `onkeypress`                | [KeyboardEvent](https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html)     | [KeyPressEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.KeyPressEvent.html)                     |
| `onkeyup`                   | [KeyboardEvent](https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html)     | [KeyUpEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.KeyUpEvent.html)                           |
| `onload`                    | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | [ResourceLoadEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ResourceLoadEvent.html)             |
| `onloadeddata`              | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onloadedmetadata`          | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onloadstart`               | [ProgressEvent](https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html)     | [LoadStartEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.LoadStartEvent.html)                   |
| `onmousedown`               | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           | [MouseDownEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseDownEvent.html)                   |
| `onmouseenter`              | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           | [MouseEnterEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseEnterEvent.html)                 |
| `onmouseleave`              | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           | [MouseLeaveEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseLeaveEvent.html)                 |
| `onmousemove`               | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           | [MouseMoveEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseMoveEvent.html)                   |
| `onmouseout`                | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           | [MouseOutEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseOutEvent.html)                     |
| `onmouseover`               | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           | [MouseOverEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseOverEvent.html)                   |
| `onmouseup`                 | [MouseEvent](https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html)           | [MouseUpEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseUpEvent.html)                       |
| `onpause`                   | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onplay`                    | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onplaying`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onprogress`                | [ProgressEvent](https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html)     | [ProgressEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ProgressEvent.html)                     |
| `onratechange`              | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onreset`                   | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onresize`                  | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | [ResizeEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ResizeEvent.html)                         |
| `onscroll`                  | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | [ScrollEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.ScrollEvent.html)                         |
| `onsecuritypolicyviolation` | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onseeked`                  | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onseeking`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onselect`                  | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onslotchange`              | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | [SlotChangeEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.SlotChangeEvent.html)                 |
| `onstalled`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onsubmit`                  | [FocusEvent](https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html)           | [SubmitEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.SubmitEvent.html)                         |
| `onsuspend`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `ontimeupdate`              | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `ontoggle`                  | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onvolumechange`            | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onwaiting`                 | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onwheel`                   | [WheelEvent](https://docs.rs/web-sys/latest/web_sys/struct.WheelEvent.html)           | [MouseWheelEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.MouseWheelEvent.html)                 |
| `oncopy`                    | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `oncut`                     | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onpaste`                   | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onanimationcancel`         | [AnimationEvent](https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html)   | Unsupported                                                                                                   |
| `onanimationend`            | [AnimationEvent](https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html)   | Unsupported                                                                                                   |
| `onanimationiteration`      | [AnimationEvent](https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html)   | Unsupported                                                                                                   |
| `onanimationstart`          | [AnimationEvent](https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html)   | Unsupported                                                                                                   |
| `ongotpointercapture`       | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       | [GotPointerCaptureEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.GotPointerCaptureEvent.html)   |
| `onloadend`                 | [ProgressEvent](https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html)     | [LoadEndEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.LoadEndEvent.html)                       |
| `onlostpointercapture`      | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       | [LostPointerCaptureEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.LostPointerCaptureEvent.html) |
| `onpointercancel`           | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       | [PointerCancelEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerCancelEvent.html)           |
| `onpointerdown`             | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       | [PointerDownEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerDownEvent.html)               |
| `onpointerenter`            | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       | [PointerEnterEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerEnterEvent.html)             |
| `onpointerleave`            | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       | [PointerLeaveEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerLeaveEvent.html)             |
| `onpointerlockchange`       | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | [PointerLockChangeEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerLockChangeEvent.html)   |
| `onpointerlockerror`        | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | [PointerLockErrorEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerLockErrorEvent.html)     |
| `onpointermove`             | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       | [PointerMoveEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerMoveEvent.html)               |
| `onpointerout`              | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       | [PointerOutEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerOutEvent.html)                 |
| `onpointerover`             | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       | [PointerOverEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerOverEvent.html)               |
| `onpointerup`               | [PointerEvent](https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html)       | [PointerUpEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.PointerUpEvent.html)                   |
| `onselectionchange`         | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | [SelectionChangeEvent](https://docs.rs/stdweb/latest/stdweb/web/event/struct.SelectionChangeEvent.html)       |
| `onselectstart`             | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `onshow`                    | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     | Unsupported                                                                                                   |
| `ontouchcancel`             | [TouchEvent](https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html)           | [TouchCancel](https://docs.rs/stdweb/latest/stdweb/web/event/struct.TouchCancel.html)                         |
| `ontouchend`                | [TouchEvent](https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html)           | [TouchEnd](https://docs.rs/stdweb/latest/stdweb/web/event/struct.TouchEnd.html)                               |
| `ontouchmove`               | [TouchEvent](https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html)           | [TouchMove](https://docs.rs/stdweb/latest/stdweb/web/event/struct.TouchMove.html)                             |
| `ontouchstart`              | [TouchEvent](https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html)           | [TouchStart](https://docs.rs/stdweb/latest/stdweb/web/event/struct.TouchStart.html)                           |
| `ontransitioncancel`        | [TransitionEvent](https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html) | Unsupported                                                                                                   |
| `ontransitionend`           | [TransitionEvent](https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html) | Unsupported                                                                                                   |
| `ontransitionrun`           | [TransitionEvent](https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html) | Unsupported                                                                                                   |
| `ontransitionstart`         | [TransitionEvent](https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html) | Unsupported                                                                                                   |

## Relevant examples
- [Inner HTML](https://github.com/yewstack/yew/tree/master/examples/inner_html)
