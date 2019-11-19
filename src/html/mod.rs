//! The main module which contents aliases to necessary items
//! to create a template and implement `update` and `view` functions.
//! Also this module contains declaration of `Component` trait which used
//! to create own UI-components.

mod listener;
mod scope;

pub use listener::*;
pub(crate) use scope::ComponentUpdate;
pub use scope::Scope;

use crate::callback::Callback;
use crate::virtual_dom::{VChild, VList, VNode};
use std::any::TypeId;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use stdweb::unstable::TryFrom;
use stdweb::web::Node;

#[cfg(all(target_arch = "wasm32", not(cargo_web)))]
use std::future::Future;

/// This type indicates that component should be rendered again.
pub type ShouldRender = bool;

/// An interface of a UI-component. Uses `self` as a model.
pub trait Component: Sized + 'static {
    /// Control message type which `update` loop get.
    type Message: 'static;
    /// Properties type of component implementation.
    type Properties: Properties;
    /// Initialization routine which could use a context.
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self;
    /// Called after the component has been attached to the VDOM and it is safe to receive messages
    /// from agents but before the browser updates the screen. If true is returned, the view will
    /// be re-rendered and the user will not see the initial render.
    fn mounted(&mut self) -> ShouldRender {
        false
    }
    /// Called everytime when a messages of `Msg` type received. It also takes a
    /// reference to a context.
    fn update(&mut self, msg: Self::Message) -> ShouldRender;
    /// Called when the component's parent component re-renders and the
    /// component's place in the DOM tree remains unchanged. If the component's
    /// place in the DOM tree changes, calling this method is unnecessary as the
    /// component is recreated from scratch. It defaults to true if not implemented
    /// and Self::Properties is not the unit type `()`.
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        TypeId::of::<Self::Properties>() != TypeId::of::<()>()
    }
    /// Called by rendering loop.
    fn view(&self) -> Html<Self>;
    /// Called for finalization on the final point of the component's lifetime.
    fn destroy(&mut self) {} // TODO Replace with `Drop`
}

/// A type which expected as a result of `view` function implementation.
pub type Html<MSG> = VNode<MSG>;

/// A type used for accepting children elements in Component::Properties.
///
/// # Example
/// **`model.rs`**
///
/// In this example, the `Wrapper` component is used to wrap other elements.
/// ```
///# use yew::{Children, Html, Properties, Component, ComponentLink, html};
///# #[derive(Properties)]
///# struct WrapperProps {
///#     children: Children<Wrapper>,
///# }
///# struct Wrapper;
///# impl Component for Wrapper{
///#     type Message = ();
///#     type Properties = WrapperProps;
///#     fn create(props: Self::Properties,link: ComponentLink<Self>) -> Self {unimplemented!()}
///#     fn update(&mut self,msg: Self::Message) -> bool {unimplemented!()}
///#     // This is not a valid implementation.  This is done for space convenience.
///#     fn view(&self) -> Html<Self> {
/// html! {
///     <Wrapper>
///         <h4>{ "Hi" }</h4>
///         <div>{ "Hello" }</div>
///     </Wrapper>
/// }
///#     }
///# }
/// ```
///
/// **`wrapper.rs`**
///
/// The Wrapper component must define a `children` property in order to wrap other elements. The
/// children property can be used to render the wrapped elements.
/// ```
///# use yew::{Children, Html, Properties, Renderable, Component, ComponentLink, html};
/// #[derive(Properties)]
/// struct WrapperProps {
///     children: Children<Wrapper>,
/// }
///
///# struct Wrapper {props: WrapperProps};
/// impl Component for Wrapper {
///     // ...
///#     type Message = ();
///#     type Properties = WrapperProps;
///#     fn create(props: Self::Properties,link: ComponentLink<Self>) -> Self {unimplemented!()}
///#     fn update(&mut self,msg: Self::Message) -> bool {unimplemented!()}
///     fn view(&self) -> Html<Wrapper> {
///         html! {
///             <div id="container">
///                 { self.props.children.render() }
///             </div>
///         }
///     }
/// }
/// ```
pub type Children<T> = ChildrenRenderer<Html<T>>;

/// A type used for accepting children elements in Component::Properties and accessing their props.
///
/// # Example
/// **`model.rs`**
///
/// In this example, the `List` component can wrap `ListItem` components.
/// ```
///# use yew::{html, Component, Renderable, Html, ComponentLink, ChildrenWithProps, Properties};
///#
///# #[derive(Properties)]
///# struct ListProps {
///#     children: ChildrenWithProps<ListItem, List>,
///# }
///# struct List;
///# impl Component for List {
///#     type Message = ();
///#     type Properties = ListProps;
///#     fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {unimplemented!()}
///#     fn update(&mut self, msg: Self::Message) -> bool {unimplemented!()}
///#     fn view(&self) -> Html<List> {unimplemented!()}
///# }
///# #[derive(Properties)]
///# struct ListItemProps {
///#     value: String
///# }
///# struct ListItem;
///# impl Component for ListItem {
///#     type Message = ();
///#     type Properties = ListItemProps;
///#     fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {unimplemented!()}
///#     fn update(&mut self, msg: Self::Message) -> bool {unimplemented!()}
///#     fn view(&self) -> Html<Self> {unimplemented!()}
///# }
///# fn view() -> Html<List> {
/// html!{
///   <List>
///     <ListItem value="a" />
///     <ListItem value="b" />
///     <ListItem value="c" />
///   </List>
/// }
///# }
/// ```
///
/// **`list.rs`**
///
/// The `List` component must define a `children` property in order to wrap the list items. The
/// `children` property can be used to filter, mutate, and render the items.
/// ```
///# use yew::{html, Component, Html, ChildrenWithProps, ComponentLink, Properties};
///#
/// #[derive(Properties)]
/// struct ListProps {
///   children: ChildrenWithProps<ListItem, List>,
/// }
///
///# struct List {props: ListProps};
/// impl Component for List {
///#     type Message = ();
///#     type Properties = ListProps;
///#     fn create(props: Self::Properties,link: ComponentLink<Self>) -> Self {unimplemented!()}
///#     fn update(&mut self,msg: Self::Message) -> bool {unimplemented!()}
///     // ...
///     fn view(&self) -> Html<Self> {
///         html!{{
///             for self.props.children.iter().map(|mut item| {
///                 item.props.value = format!("item-{}", item.props.value);
///                 item
///             })
///         }}
///     }
/// }
///#
///# #[derive(Properties)]
///# struct ListItemProps {
///#     value: String
///# }
///#
///# struct ListItem;
///# impl Component for ListItem {
///#     type Message = ();
///#     type Properties = ListItemProps;
///#     fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {unimplemented!()}
///#     fn update(&mut self, msg: Self::Message) -> bool {unimplemented!()}
///#     fn view(&self) -> Html<ListItem> {unimplemented!()}
///# }
/// ```
pub type ChildrenWithProps<C, P> = ChildrenRenderer<VChild<C, P>>;

/// A type used for rendering children html.
pub struct ChildrenRenderer<T> {
    len: usize,
    boxed_render: Box<dyn Fn() -> Vec<T>>,
}

impl<T> ChildrenRenderer<T> {
    /// Create children
    pub fn new(len: usize, boxed_render: Box<dyn Fn() -> Vec<T>>) -> Self {
        Self { len, boxed_render }
    }

    /// Children list is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Number of children elements
    pub fn len(&self) -> usize {
        self.len
    }

    /// Build children components and return `Vec`
    pub fn to_vec(&self) -> Vec<T> {
        (&self.boxed_render)()
    }

    /// Render children components and return `Iterator`
    pub fn iter(&self) -> impl Iterator<Item = T> {
        (&self.boxed_render)().into_iter()
    }
}

impl<T> Default for ChildrenRenderer<T> {
    fn default() -> Self {
        // False positive: https://github.com/rust-lang/rust-clippy/issues/4002
        #[allow(clippy::redundant_closure)]
        let boxed_render = Box::new(|| Vec::new());
        Self {
            len: 0,
            boxed_render,
        }
    }
}

impl<T> fmt::Debug for ChildrenRenderer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ChildrenRenderer<_>")
    }
}

impl<T, COMP: Component> Renderable<COMP> for ChildrenRenderer<T>
where
    T: Into<VNode<COMP>>,
{
    fn render(&self) -> Html<COMP> {
        VList {
            children: self.iter().map(|c| c.into()).collect(),
        }
        .into()
    }
}

/// Wrapped Node reference for later use in Component lifecycle methods.
///
/// # Example
/// Focus an `<input>` element on mount.
/// ```
/// use stdweb::web::html_element::InputElement;
/// use stdweb::web::IHtmlElement;
///# use yew::*;
///
/// pub struct Input {
///     node_ref: NodeRef,
/// }
///
/// impl Component for Input {
///     type Message = ();
///     type Properties = ();
///
///     fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
///         Input {
///             node_ref: NodeRef::default(),
///         }
///     }
///
///     fn mounted(&mut self) -> ShouldRender {
///         if let Some(input) = self.node_ref.try_into::<InputElement>() {
///             input.focus();
///         }
///         false
///     }
///
///     fn update(&mut self, _: Self::Message) -> ShouldRender {
///         false
///     }
///
///     fn view(&self) -> Html<Self> {
///         html! {
///             <input ref=self.node_ref.clone() type="text" />
///         }
///     }
/// }
#[derive(PartialEq, Debug, Default, Clone)]
pub struct NodeRef(Rc<RefCell<Option<Node>>>);

impl NodeRef {
    /// Get the wrapped Node reference if it exists
    pub fn get(&self) -> Option<Node> {
        self.0.borrow().clone()
    }

    /// Try converting the node reference into another form
    pub fn try_into<INTO: TryFrom<Node>>(&self) -> Option<INTO> {
        self.get().and_then(|node| INTO::try_from(node).ok())
    }

    /// Place a Node in a reference for later use
    pub(crate) fn set(&self, node: Option<Node>) {
        *self.0.borrow_mut() = node;
    }
}

/// Should be rendered relative to context and component environment.
pub trait Renderable<COMP: Component> {
    /// Called by rendering loop.
    fn render(&self) -> Html<COMP>;
}

impl<COMP: Component> Renderable<COMP> for COMP {
    fn render(&self) -> Html<COMP> {
        self.view()
    }
}

/// Trait for building properties for a component
pub trait Properties {
    /// Builder that will be used to construct properties
    type Builder;

    /// Entrypoint for building properties
    fn builder() -> Self::Builder;
}

/// Builder for when a component has no properties
#[derive(Debug)]
pub struct EmptyBuilder;

impl Properties for () {
    type Builder = EmptyBuilder;

    fn builder() -> Self::Builder {
        EmptyBuilder
    }
}

impl EmptyBuilder {
    /// Build empty properties
    pub fn build(self) {}
}

/// Link to component's scope for creating callbacks.
pub struct ComponentLink<COMP: Component> {
    scope: Scope<COMP>,
}

impl<COMP> ComponentLink<COMP>
where
    COMP: Component,
{
    /// Create link for a scope.
    fn connect(scope: &Scope<COMP>) -> Self {
        ComponentLink {
            scope: scope.clone(),
        }
    }

    /// This method sends batch of messages back to the component's loop when the
    /// returned callback is called.
    pub fn send_back_batch<F, IN>(&mut self, function: F) -> Callback<IN>
    where
        F: Fn(IN) -> Vec<COMP::Message> + 'static,
    {
        let scope = self.scope.clone();
        let closure = move |input| {
            let messages = function(input);
            scope.clone().send_message_batch(messages);
        };
        closure.into()
    }

    /// This method sends messages back to the component's loop when the returned callback is called.
    pub fn send_back<F, IN>(&mut self, function: F) -> Callback<IN>
    where
        F: Fn(IN) -> COMP::Message + 'static,
    {
        let scope = self.scope.clone();
        let closure = move |input| {
            let output = function(input);
            scope.clone().send_message(output);
        };
        closure.into()
    }

    #[cfg(all(target_arch = "wasm32", not(target_os = "wasi"), not(cargo_web)))]
    /// This method processes a Future that returns a message and sends it back to the component's
    /// loop.
    ///
    /// # Panics
    /// If the future panics, then the promise will not resolve, and will leak.
    pub fn send_future<F>(&self, future: F)
    where
        F: Future<Output = COMP::Message> + 'static,
    {
        use wasm_bindgen::JsValue;
        use wasm_bindgen_futures::future_to_promise;

        let mut scope = self.scope.clone();

        let js_future = async {
            let message: COMP::Message = future.await;
            // Force movement of the cloned scope into the async block.
            let scope_send = move || scope.send_message(message);
            scope_send();
            Ok(JsValue::NULL)
        };
        future_to_promise(js_future);
    }

    /// This method sends a message to this component to be processed immediately after the
    /// component has been updated and/or rendered.
    pub fn send_self(&mut self, msg: COMP::Message) {
        self.scope.send_message(msg);
    }

    /// Sends a batch of messages to the component to be processed immediately after
    /// the component has been updated and/or rendered..
    ///
    /// All messages will first be processed by `update`, and if _any_ of them return `true`,
    /// then re-render will occur.
    pub fn send_self_batch(&mut self, msgs: Vec<COMP::Message>) {
        self.scope.send_message_batch(msgs)
    }
}

impl<COMP: Component> fmt::Debug for ComponentLink<COMP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ComponentLink<_>")
    }
}

impl<COMP: Component> Clone for ComponentLink<COMP> {
    fn clone(&self) -> Self {
        ComponentLink {
            scope: self.scope.clone(),
        }
    }
}

/// A bridging type for checking `href` attribute value.
#[derive(Debug)]
pub struct Href {
    link: String,
}

impl From<String> for Href {
    fn from(link: String) -> Self {
        Href { link }
    }
}

impl<'a> From<&'a str> for Href {
    fn from(link: &'a str) -> Self {
        Href {
            link: link.to_owned(),
        }
    }
}

impl ToString for Href {
    fn to_string(&self) -> String {
        self.link.to_owned()
    }
}
