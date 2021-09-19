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
use yew::{utils::document, web_sys::{Element, Node}, Html};
// ...
fn view(&self) -> Html {
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
use yew::html;

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
use yew::html;

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
use yew::html;

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
use yew::html;

let maybe_id = Some("foobar");

html! {
    <div id=maybe_id></div>
}
```

If the attribute is set to `None`, the attribute won't be set in the DOM.

Please note that it is also valid to give only the value as properties behave
like `Into<Option<T>>`:

```rust
use yew::html;

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
use yew::{
    events::MouseEvent, html, Component,
    ComponentLink, Html, ShouldRender
};

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
        let click_callback = self.link.callback(|_: MouseEvent| Msg::Click);
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
use yew::{
    agent::Dispatcher, events::MouseEvent, html, Component,
    ComponentLink, Html, ShouldRender,
};

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
        let click_callback = self.worker.callback(|_: MouseEvent| WorkerMsg::Process);
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
use yew::{
    html, services::ConsoleService, Callback, Component,
    ComponentLink, Html, ShouldRender,
};

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
        let click_callback = Callback::from(|_| {
            ConsoleService::log("clicked!");
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

## Relevant examples
- [Inner HTML](https://github.com/yewstack/yew/tree/v0.18/examples/inner_html)
