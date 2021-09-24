
The [`web-sys` crate](https://crates.io/crates/web-sys) provides bindings for Web APIs. This is
procedurally generated from browser WebIDL which is why some of the names are so long and why
some of the types are vague.

## Features in `web-sys`

The `web-sys` crate with all of it's features enabled can add lots of bloat to a Wasm application,
in order to get around this issue most types are feature gated so that you only include the types
you require for your application. Yew includes quite a number of features from `web-sys` and
re-exports them, but you will often find that you need to add `web-sys` as a dependency yourself
so you can enable features for APIs not used by Yew internally.

## Inheritance in `web-sys`

In the [Simulating inheritance section](../wasm-bindgen#simulating-inheritance) you can read how in
general Rust provides an approach to simulate inheritance in JavaScript. This is very important in
`web-sys` as understanding what methods are available on a type means understanding it's inheritance.

This section is going to look at a specific element and list out it's inheritance using Rust by
calling [`Deref::deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html#tymethod.deref) until
the value is [`JsValue`](../wasm-bindgen#jsvalue):

```rust
use std::ops::Deref;
use web_sys::{
	Element,
	EventTarget,
	HtmlElement,
	HtmlTextAreaElement,
	Node,
};

fn inheritance_of_text_area(text_area: HtmlTextAreaElement) {
	// HtmlTextAreaElement is <textarea> in html.
	let html_element: &HtmlElement = text_area.deref();

	let element: &Element = html_element.deref();

	let node: &Node = element.deref();

	let event_target: &EventTarget = node.deref();

	// Notice we've moved from web-sys types now into built-in
	// JavaScript types which are in the js-sys crate.
	let object: &js_sys::Object = event_target.deref();

	// Notice we've moved from js-sys type to the root JsValue from
	// the wasm-bindgen crate.
	let js_value: &wasm_bindgen::JsValue = object.deref();

	// Using deref like this means we have to manually traverse
	// the inheritance tree, however, you can call JsValue methods
	// on the HtmlTextAreaElement type.
	// The `is_string` method comes from JsValue.
	assert!(!text_area.is_string());

	// The AsRef implementations allow you to treat the HtmlTextAreaElement
	// as an &EventTarget.

	// empty function just to prove we can pass HtmlTextAreaElement as a
	// &EventTarget.
	fn this_function_only_takes_event_targets(targets: &EventTarget) {};

	// If you are reading this then it compiled because we check the website
	// code blocks :)
	this_function_only_takes_event_targets(&text_area);
}
```

_[Inheritance in `web-sys` in The `wasm-bindgen` Guide](https://rustwasm.github.io/wasm-bindgen/web-sys/inheritance.html)._

## The `Node` in `NodeRef`

Yew uses a [`NodeRef`](../components/refs) in order to provide a way for keeping a reference to
a `Node` made by the [`html!`](../html) macro. The `Node` part of `NodeRef` is referring to
[`web_sys::Node`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Node.html). The
`NodeRef::get` method will return a `Option<Node>` value, however, most of the time in Yew you want
to cast this value to a specific element so you can use it's specific methods. This casting
can be done using [`JsCast`](../wasm-bindgen#JsCast) on the `Node` value, if present, but Yew
provides the `NodeRef::cast` method to perform this casting for convenience and so that you don't
necessarily have to include the `wasm-bindgen` dependency for the `JsCast` trait.

The two code blocks below do essentially the same thing, the first is using `NodeRef::cast` and
the second is using [`JsCast::dyn_into`](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_into)
on the `web_sys::Node` returned from `NodeRef::get`.

```rust
use yew::{web_sys::HtmlInputElement, NodeRef};

fn with_node_ref_cast(node_ref: NodeRef) {
	if let Some(input) = node_ref.cast::<HtmlInputElement>() {
		// do something with HtmlInputElement
	}
}
```

```rust
use yew::{web_sys::HtmlInputElement, NodeRef};
use wasm_bindgen::JsCast;

fn with_jscast(node_ref: NodeRef) {
	if let Some(input) = node_ref
		.get()
		.and_then(|node| node.dyn_into::<HtmlInputElement>().ok()) {
		// do something with HtmlInputElement
	}
}
```

## JavaScript example to Rust

This section is to help show that any examples that use JavaScript to interact with the Web APIs
can be adapted and written using Rust with `web-sys`.

The JavaScript example:

```js
document.getElementById('mousemoveme').onmousemove = (e) => {
    // e = Mouse event.
    var rect = e.target.getBoundingClientRect();
    var x = e.clientX - rect.left; //x position within the element.
    var y = e.clientY - rect.top;  //y position within the element.
    console.log("Left? : " + x + " ; Top? : " + y + ".");
}
```

In Yew you will mostly be creating [`Callback`](../components/callbacks)s to use in the
[`html!`](../html) macro so the example is going to use this approach instead of completely copying
the approach above (but you could still do it!):

```toml title=Cargo.toml
[dependencies.web-sys]
version = "0.3"
# We need to enable the `DomRect` feature in order to use the
# `get_bounding_client_rect` method.
features = [
	"DomRect"
]

```

```rust
use yew::{
    html,
	web_sys::{console, HtmlElement},
    Callback, MouseEvent, TargetCast,
};

let onmousemove = Callback::from(|e: MouseEvent| {
    if let Some(target) = e.target_dyn_into::<HtmlElement>() {
        let rect = target.get_bounding_client_rect();
        let x = (e.client_x() as f64) - rect.left();
        let y = (e.client_y() as f64) - rect.top();
        console::log_1(&format!("Left? : {} ; Top? : {}", x, y).into());
    }
});

html! {
    <div id="mousemoveme" {onmousemove}></div>
};
```

The `web-sys` feature kicks in even though the [`HtmlElement`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlElement.html)
is used from the re-exported version of `web-sys` by Yew, this works but because features are
additive but it's important that both versions of `web-sys` are compatible.

## External libraries

`web-sys` is a raw binding to the Web API so it comes with some pain in Rust because it was not
designed with Rust or even a strong type system in mind, this is where community crates come in to
provide abstractions over `web-sys` to provide more idiomatic Rust APIs.

_[External libraries page](../../more/external-libs)_