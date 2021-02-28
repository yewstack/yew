# Yewtil
A utility crate for the [Yew](https://github.com/yewstack/yew) frontend web framework.

> This crate used to contain a domain specific language which made it possible to create components without use of the `html!` macro, but this has now been moved into a different crate ([`yew-dsl`](https://github.com/yewstack/yew/tree/master/packages/yew-dsl)).

## Purpose
To provide a place for utilities which are commonly used with Yew to reside without having to include them in the core Yew crate.
Because of this the Yew crate can make breaking changes which will cause `yewtil` to become incompatible with Yew.

## Features
Currently, this crate supports the following features in a "stable" capacity:
* `NeqAssign` - makes assigning props and returning a relevant value for `ShouldRender` easier.
* Pure Components - implement pure components using the `PureComponent` trait and the `Pure` Component adaptor. 
This should make it much easier to define simple components that don't hold state.
  * Function components - a macro that takes a function that returns `Html` and converts it to a pure component.
* `Mrc`/`Irc` smart pointers - Rc-like pointers that are more ergonomic to use within Yew.
* `History` - A wrapper that holds the history of values that have been assigned to it.
* `Effect` - A way to update component state by defining what to change inside of `html!` callbacks
 instead of handling messages in `Component::update()`. (Deprecated)


This crate also has feature flags which will enable the following **experimental** features:
* `Lrc` smart pointer - an Rc-like pointer implemented on top of a linked list which allows for novel state update mechanics 
and traversal over linked shared pointers. <sup><sub>(This needs to be fuzz tested to make sure it doesn't leak.)</sub></sup>
(Deprecated)

These experimental features are either not sufficiently vetted and may change significantly or be removed.

## Example Projects 
There are [examples showing how to use every stable feature in this crate](https://github.com/yewstack/yew/tree/master/packages/yewtil/examples). 

Check out the [Pure Components example](https://github.com/yewstack/yew/tree/master/packages/yewtil/examples/pure_component) to see how Pure Components work.

## Example usages
#### neq_assign:

```rust
fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.props.neq_assign(props)
}
```

-------------

#### Pure Components:
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

As new idioms are introduced to Yew, this crate may see some updates, but given the rarity of those, this crate may sit unaltered for some time.

## Scope
This crate has less stringent requirements for its code than the main Yew crate; if you have a function, type, or trait you would like to include, please open a pull request or an issue.

Components are welcome as well, but they must not have external dependencies, should solve some problem encountered by many users of Yew, and should allow for theming if possible, like an auto-scrolling wrapper, a RecyclerView/Infinite-scrolling component, or possibly a comprehensive input component.

Common UI elements like modals or dropdowns are best be left to component libraries, as they are often coupled to external CSS used to display them. The [Yewtify](https://github.com/yewstack/yewtify) crate is one such component library.

### Stability
Since this crate aims to provide a variety of helper types, traits, and functions, where the utility of each may be unknown at the time the feature is added, newer additions may be not be included in the default feature set, and may be kept behind a feature flag.

While in early development, features marked as `experimental` may be changed frequently or even entirely removed, while those marked as `stable` will not be removed and can be relied upon to not change significantly.
