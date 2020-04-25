# Yewtil
Utility crate for the [Yew](https://github.com/yewstack/yew) frontend web framework.

## Purpose
Provide a place for commonly used utilities for Yew to reside without them having to be included in the Yew crate itself.
As a consequence of this, the Yew crate is free to make changes that may cause breakages in this crate.

## Features
Currently, this crate supports these features in a stable capacity:
* `NeqAssign` - makes assigning props and returning a relevant ShouldRender value easier.
* Pure Components - implement pure components using the `PureComponent` trait and the `Pure` Component adaptor. 
This should make it much easier to define simple components that don't hold state.
  * Function components - a macro that takes a function that returns `Html` and converts it to a pure component.
* `Mrc`/`Irc` smart pointers - Rc-like pointers that are more ergonomic to use within Yew.
* `History` - A wrapper that holds the history of values that have been assigned to it.
* `Effect` - A way to update component state by defining what to change inside of `html!` callbacks
 instead of handling messages in `Component::update()`.

This crate also has an experimental feature flag that enables the following features:
* `Lrc` smart pointer - Rc-like pointer implemented on top of a linked list. Allows for novel state update mechanics 
and traversal over linked shared pointers. <sup><sub>(This needs to be fuzz tested to make sure it doesn't leak.)</sub></sup>
* DSL for `Html<Self>` - A function-based domain-specific-language for Yew that can be used in a limited capacity instead of the `html!` macro. <sup><sub>(Broken by recent changes in yew. Will be rewritten from scratch eventually.)</sub></sup>

These experimental features are either not sufficiently vetted, may change significantly, or may be removed.

## Example Projects 
Examples for every stable feature exist [here](https://github.com/yewstack/yew/tree/master/yewtil/examples). 

Check out the [Pure Components example](https://github.com/yewstack/yew/tree/master/yewtil/examples/demo) to see how Pure Components work.

## Example
#### neq_assign:
```rust
fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.props.neq_assign(props)
}
```

-------------

#### Pure Component:
```rust
pub type Button = Pure<PureButton>;

#[derive(PartialEq, Clone, Properties)]
pub struct PureButton {
    pub callback: Callback<Msg>,
    #[prop_or_default]
    pub text: String,
}

impl PureComponent for PureButton {
    fn render(&self) -> VNode {
        html! {
            <button onclick=&self.callback>{ &self.text }</button>
        }
    }
}
```

--------------

#### History
```rust
pub struct Model {
    text: History<String>,
}

// ...
fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
        Msg::SetText(text) => self.text.neq_set(text),
        Msg::Reset => self.text.reset(),
        Msg::Forget => {
            self.text.forget();
            false
        }
    }
}
```

## Update Schedule
This crate will target stable Yew.

As new idioms are introduced to Yew, this crate may see updates, but given the rarity of those, this crate may sit unaltered for some time.

## Scoping
This crate aims to be more permissive in what is allowed in than Yew, so if you have a function, type, or trait you would like to share, please open a PR or Issue.

Components are welcome as well, but they must not have external dependencies, should solve some problem encountered my many users of Yew, and should allow for theming if possible, like an auto-scrolling wrapper, a RecyclerView/Infinite-scrolling component, or possibly a comprehensive Input component.

Common UI elements like modals or dropdowns should probably best be left to CSS-framework component libraries, as they should often be coupled to the external CSS used to display them.

### Stability
Since this crate aims to present a variety of helper types, traits, and functions, where the utility of each may be unknown at the time the feature is added, newer additions may be not be included in the default feature-set, and may be locked behind an `experimental` flag. 
While in early development, features marked as `experimental` may be changed more frequently or even entirely removed, while those marked as `stable` will not be removed and can be depended on to not change significantly.
