#![allow(clippy::needless_doctest_main)]
#![doc(html_logo_url = "https://static.yew.rs/logo.svg")]

//! # Yew Framework - API Documentation
//!
//! Yew is a modern Rust framework for creating multi-threaded front-end web apps using WebAssembly
//!
//! - Features a macro for declaring interactive HTML with Rust expressions. Developers who have experience using JSX in React should feel quite at home when using Yew.
//! - Achieves high performance by minimizing DOM API calls for each page render and by making it easy to offload processing to background web workers.
//! - Supports JavaScript interoperability, allowing developers to leverage NPM packages and integrate with existing JavaScript applications.
//!
//! ### Supported Targets
//! - `wasm32-unknown-unknown`
#![cfg_attr(
    feature = "std_web",
    doc = "\
 - `wasm32-unknown-emscripten`
 - `asmjs-unknown-emscripten`"
)]
//!
//! ### Important Notes
//! - Yew is not (yet) production ready but is great for side projects and internal tools
#![cfg_attr(
    feature = "web_sys",
    doc = " - If your app is built with `stdweb`, we recommend using [`yew-stdweb`](https://docs.rs/yew-stdweb) instead."
)]
#![cfg_attr(
    feature = "std_web",
    doc = "\
 - We recommend aliasing `yew-stdweb` to `yew` in your Cargo.toml: `yew = { package = \"yew-stdweb\", .. }`
 - If your app is built with `web-sys`, we recommend using [`yew`](https://docs.rs/yew) instead."
)]
//!
//! ## Example
//!
//! ```rust
//! use yew::prelude::*;
//!
//! struct Model {
//!     link: ComponentLink<Self>,
//!     value: i64,
//! }
//!
//! enum Msg {
//!     AddOne,
//! }
//!
//! impl Component for Model {
//!     type Message = Msg;
//!     type Properties = ();
//!     fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
//!         Self {
//!             link,
//!             value: 0,
//!         }
//!     }
//!
//!     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//!         match msg {
//!             Msg::AddOne => self.value += 1
//!         }
//!         true
//!     }
//!
//!     fn change(&mut self, _: Self::Properties) -> ShouldRender {
//!         false
//!     }
//!
//!     fn view(&self) -> Html {
//!         html! {
//!             <div>
//!                 <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
//!                 <p>{ self.value }</p>
//!             </div>
//!         }
//!     }
//! }
//!
//!# fn dont_execute() {
//! fn main() {
//!     yew::initialize();
//!     App::<Model>::new().mount_to_body();
//! }
//!# }
//! ```
//!

#![deny(
    missing_docs,
    missing_debug_implementations,
    bare_trait_objects,
    anonymous_parameters,
    elided_lifetimes_in_paths
)]
#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]
#![recursion_limit = "512"]
extern crate self as yew;

/// This macro implements JSX-like templates.
pub use yew_macro::html;

/// This macro is similar to [`html!`], but returns actual component type instead
/// of generic [`Html`].
///
/// That macro is useful when, for example, in typical implementation of `List`
/// component. In the typical implementation you have two connected components --
/// `List` and `ListItem`, and `List` allowing only `ListItem`s inside them.
/// Example of such component you can find in the [`nested_list`] example.
///
/// Manually you create list by just nesting `ListItem`'s into `List`:
///
/// ```
/// # use yew::prelude::*;
/// use yew::html::ChildrenRenderer;
/// use yew::virtual_dom::VChild;
///
/// #[derive(Clone, Properties)]
/// struct List {
///   children: ChildrenRenderer<ListItem>,
/// }
/// impl Component for List {
///   // ...
/// #   type Message = ();
/// #   type Properties = Self;
/// #   fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self { props }
/// #   fn update(&mut self, _: Self::Message) -> ShouldRender { false }
/// #   fn change(&mut self, _: Self::Properties) -> ShouldRender { false }
/// #   fn view(&self) -> Html { unimplemented!() }
/// }
///
/// #[derive(Clone)]
/// struct ListItem;
/// impl Component for ListItem {
/// #   type Message = ();
///   type Properties = ();
///   // ...
/// #   fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self { Self }
/// #   fn update(&mut self, _: Self::Message) -> ShouldRender { false }
/// #   fn change(&mut self, _: Self::Properties) -> ShouldRender { false }
/// #   fn view(&self) -> Html { unimplemented!() }
/// }
///
/// // Required for ChildrenRenderer
/// impl From<VChild<ListItem>> for ListItem {
///   fn from(child: VChild<ListItem>) -> Self { Self }
/// }
///
/// impl From<ListItem> for Html {
///   fn from(item: ListItem) -> Html { item.view() }
/// }
///
/// # fn test() -> Html {
/// html! {
///   <List>
///     <ListItem/>
///     <ListItem/>
///     <ListItem/>
///   </List>
/// }
/// # }
/// ```
///
/// But in most real cases you need to forming content of you list programmatically.
/// That's mean, that you would write code like this:
/// ```
/// # use yew::prelude::*;
/// # use yew::html::ChildrenRenderer;
/// # use yew::virtual_dom::VChild;
/// # #[derive(Clone, Properties)]
/// # struct List { children: ChildrenRenderer<ListItem> }
/// # impl Component for List {
/// #   type Message = ();
/// #   type Properties = Self;
/// #   fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self { props }
/// #   fn update(&mut self, _: Self::Message) -> ShouldRender { false }
/// #   fn change(&mut self, _: Self::Properties) -> ShouldRender { false }
/// #   fn view(&self) -> Html { unimplemented!() }
/// # }
/// # #[derive(Clone)]
/// # struct ListItem;
/// # impl Component for ListItem {
/// #   type Message = ();
/// #   type Properties = ();
/// #   fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self { Self }
/// #   fn update(&mut self, _: Self::Message) -> ShouldRender { false }
/// #   fn change(&mut self, _: Self::Properties) -> ShouldRender { false }
/// #   fn view(&self) -> Html { unimplemented!() }
/// # }
/// # impl From<VChild<ListItem>> for ListItem {
/// #   fn from(child: VChild<ListItem>) -> Self { Self }
/// # }
/// # impl From<ListItem> for Html {
/// #   fn from(item: ListItem) -> Html { item.view() }
/// # }
/// # fn test() -> Html {
/// # let some_iter = (0..10);
/// html! {
///   <List>
///     { for some_iter.map(|_| html_nested!{ <ListItem/> }) }
///   </List>
/// }
/// # }
/// ```
///
/// If you used the [`html!`] macro instead of `html_nested!`, the code would
/// not be compiled because we explicitly indicated to the compiler that `List`
/// can only contain elements of type `ListItem` using [`ChildrenRenderer<ListItem>`],
/// while [`html!`] wraps their contents into [`Html`].
///
///
/// [`html!`]: ./macro.html.html
/// [`Html`]: ./html/type.Html.html
/// [`nested_list`]: https://github.com/yewstack/yew/tree/master/examples/nested_list
/// [`ChildrenRenderer<ListItem>`]: ./html/struct.ChildrenRenderer.html
pub use yew_macro::html_nested;

/// This module contains macros which implements html! macro and JSX-like templates
pub mod macros {
    pub use crate::html;
    pub use crate::html_nested;
    pub use yew_macro::Properties;
}

pub mod app;
pub mod callback;
pub mod format;
pub mod html;
mod scheduler;
pub mod utils;
pub mod virtual_dom;

#[cfg(feature = "agent")]
pub mod agent;
#[cfg(feature = "services")]
pub mod services;

#[cfg(feature = "web_sys")]
pub use web_sys;

/// The module that contains all events available in the framework.
pub mod events {
    use cfg_if::cfg_if;

    pub use crate::html::{ChangeData, InputData};

    cfg_if! {
        if #[cfg(feature = "std_web")] {
            #[doc(no_inline)]
            pub use stdweb::web::event::{
                BlurEvent, ClickEvent, ContextMenuEvent, DoubleClickEvent, DragDropEvent, DragEndEvent,
                DragEnterEvent, DragEvent, DragExitEvent, DragLeaveEvent, DragOverEvent, DragStartEvent,
                FocusEvent, GotPointerCaptureEvent, IKeyboardEvent, IMouseEvent, IPointerEvent,
                KeyDownEvent, KeyPressEvent, KeyUpEvent, LostPointerCaptureEvent, MouseDownEvent,
                MouseEnterEvent, MouseLeaveEvent, MouseMoveEvent, MouseOutEvent, MouseOverEvent,
                MouseUpEvent, MouseWheelEvent, PointerCancelEvent, PointerDownEvent, PointerEnterEvent,
                PointerLeaveEvent, PointerMoveEvent, PointerOutEvent, PointerOverEvent, PointerUpEvent,
                ScrollEvent, SubmitEvent, TouchCancel, TouchEnd, TouchEnter, TouchMove, TouchStart,
            };
        } else if #[cfg(feature = "web_sys")] {
            #[doc(no_inline)]
            pub use web_sys::{
                AnimationEvent, DragEvent, ErrorEvent, Event, FocusEvent, InputEvent, KeyboardEvent,
                MouseEvent, PointerEvent, ProgressEvent, TouchEvent, TransitionEvent, UiEvent, WheelEvent,
            };
        }
    }
}

use cfg_match::cfg_match;

/// Initializes yew framework. It should be called first.
pub fn initialize() {
    cfg_match! {
        feature = "std_web" => stdweb::initialize(),
        feature = "web_sys" => std::panic::set_hook(Box::new(console_error_panic_hook::hook)),
    };
}

/// Starts event loop.
pub fn run_loop() {
    #[cfg(feature = "std_web")]
    stdweb::event_loop();
}

/// Starts an app mounted to a body of the document.
pub fn start_app<COMP>()
where
    COMP: Component,
    COMP::Properties: Default,
{
    initialize();
    App::<COMP>::new().mount_to_body();
    run_loop();
}

/// Starts an app mounted to a body of the document.
pub fn start_app_with_props<COMP>(props: COMP::Properties)
where
    COMP: Component,
{
    initialize();
    App::<COMP>::new().mount_to_body_with_props(props);
    run_loop();
}

/// The Yew Prelude
///
/// The purpose of this module is to alleviate imports of many common types:
///
/// ```
/// # #![allow(unused_imports)]
/// use yew::prelude::*;
/// ```
pub mod prelude {
    #[cfg(feature = "agent")]
    pub use crate::agent::{Bridge, Bridged, Dispatched, Threaded};
    pub use crate::app::App;
    pub use crate::callback::Callback;
    pub use crate::events::*;
    pub use crate::html::{
        Children, ChildrenWithProps, Component, ComponentLink, Html, NodeRef, Properties,
        ShouldRender,
    };
    pub use crate::macros::*;
    pub use crate::virtual_dom::Classes;

    /// Prelude module for creating worker.
    #[cfg(feature = "agent")]
    pub mod worker {
        pub use crate::agent::{
            Agent, AgentLink, Bridge, Bridged, Context, HandlerId, Job, Private, Public,
        };
    }
}

pub use self::prelude::*;
