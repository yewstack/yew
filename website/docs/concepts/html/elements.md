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
use yew::{
    Component, Context, html, Html, utils::document, 
    web_sys::{Element, Node}
};

struct Comp;

impl Component for Comp {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // Create a div element from the document
        let div: Element = document().create_element("div").unwrap();
        // Add content, classes etc.
        div.set_inner_html("Hello, World!");
        // Convert Element into a Node
        let node: Node = div.into();
        // Return that Node as a Html value
        Html::VRef(node)
    }
}
```

## Dynamic tag names

When building a higher-order component you might find yourself in a situation where the element's tag name isn't static.
For example, you might have a `Title` component which can render anything from `h1` to `h6` depending on a level prop.
Instead of having to use a big match expression, Yew allows you to set the tag name dynamically
using `@{name}` where `name` can be any expression that returns a string.

```rust
use yew::html;

let level = 5;
let text = "Hello World!".to_owned();

html! {
    <@{format!("h{}", level)} class="title">{ text }</@>
};
```

## Boolean Attributes 

Some content attributes (e.g checked, hidden, required) are called boolean attributes. In Yew, 
boolean attributes need to be set to a bool value:

```rust
use yew::html;

html! {
    <div hidden=true>
        { "This div is hidden." }
    </div>
};
```

This will result in **HTML** that's functionally equivalent to this:
```html
<div hidden>This div is hidden.</div>
```

Setting a boolean attribute to false is equivalent to not using the attribute at all; values from 
boolean expressions can be used:

```rust
use yew::html;

let no = 1 + 1 != 2;

html! {
    <div hidden={no}>
        { "This div is NOT hidden." }
    </div>
};
```

This will result in the following **HTML**:

```html
<div>This div is NOT hidden.</div>
```

## Optional attributes for HTML elements

Most HTML attributes can use optional values (Some(x) or None). This allows us to omit the attribute if the attribute is marked as optional.

```rust
use yew::html;

let maybe_id = Some("foobar");

html! {
    <div id={maybe_id}></div>
};
```

If the attribute is set to `None`, the attribute won't be set in the DOM.

## Listeners

Listener attributes need to be passed a `Callback` which is a wrapper around a closure. How you create your callback depends on how you wish your app to react to a listener event:

<!--DOCUSAURUS_CODE_TABS-->
<!--Component handler-->

```rust
use yew::{Component, Context, html, Html};

struct MyComponent;

enum Msg {
    Click,
}

impl Component for MyComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Click => {
                // Handle Click
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Create a callback from a component link to handle it in a component
        let click_callback = ctx.link().callback(|_| Msg::Click);
        html! {
            <button onclick={click_callback}>
                { "Click me!" }
            </button>
        }
    }
}
```

<!--Agent Handler-->

```rust
use yew::{html, Component, Context, Html};
use yew_agent::{Dispatcher, Dispatched};
use website_test::agents::{MyWorker, WorkerMsg};

struct MyComponent {
    worker: Dispatcher<MyWorker>,
}

impl Component for MyComponent {
    type Message = WorkerMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        MyComponent {
            worker: MyWorker::dispatcher(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.worker.send(msg);
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Create a callback from a worker to handle it in another context
        let click_callback = ctx.link().callback(|_| WorkerMsg::Process);
        html! {
            <button onclick={click_callback}>
                { "Click me!" }
            </button>
        }
    }
}
```

<!--Other Cases-->

```rust
use yew::{Callback, Context, Component, html, Html};
use weblog::console_log;

struct MyComponent;

impl Component for MyComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        MyComponent
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // Create an ephemeral callback
        let click_callback = Callback::from(|_| {
            console_log!("clicked!");
        });

        html! {
            <button onclick={click_callback}>
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
| `onchange`                  | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
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
| `onfocusin`                 | [FocusEvent](https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html)           |
| `onfocusout`                | [FocusEvent](https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html)           |
| `onformdata`                | [Event](https://docs.rs/web-sys/latest/web_sys/struct.Event.html)                     |
| `oninput`                   | [InputEvent](https://docs.rs/web-sys/latest/web_sys/struct.InputEvent.html)           |
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

## Relevant examples
- [Inner HTML](https://github.com/yewstack/yew/tree/master/examples/inner_html)
