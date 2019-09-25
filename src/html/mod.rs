//! The main module which contents aliases to necessary items
//! to create a template and implement `update` and `view` functions.
//! Also this module contains declaration of `Component` trait which used
//! to create own UI-components.

mod listener;
mod scope;

pub use listener::*;
pub(crate) use scope::ComponentUpdate;
pub use scope::{NodeCell, Scope};

use crate::callback::Callback;
use crate::virtual_dom::{VChild, VList, VNode};
use std::fmt;

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
    /// component is recreated from scratch. It defaults
    /// to true if not implemented.
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }
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
/// In this example, the Wrapper component is used to wrap other elements.
/// ```
/// html!{
///   <Wrapper>
///     <h4> {"Hi"} </h4>
///     <div> {"Hello"} </div>
///   </Wrapper>
/// }
/// ```
///
/// **`wrapper.rs`**
///
/// The Wrapper component must define a `children` property in order to wrap other elements. The
/// children property can be used to render the wrapped elements.
/// ```
/// #[derive(Properties)]
/// struct WrapperProps {
///   children: Children<Wrapper>,
/// }
///
/// html!{
///   <div id="container">
///     { self.props.children.view() }
///   </div>
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
/// html!{
///   <List>
///     <ListItem value="a" />
///     <ListItem value="b" />
///     <ListItem value="c" />
///   </List>
/// }
/// ```
///
/// **`list.rs`**
///
/// The `List` component must define a `children` property in order to wrap the list items. The
/// `children` property can be used to filter, mutate, and render the items.
/// ```
/// #[derive(Properties)]
/// struct ListProps {
///   children: ChildrenWithProps<ListItem, List>,
/// }
///
/// html!{{
///   for self.props.children.iter().map(|mut item| {
///     item.props.value = format!("item-{}", item.props.value);
///     item
///   })
/// }}
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
        Self {
            len: 0,
            boxed_render: Box::new(|| Vec::new()),
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
    fn view(&self) -> Html<COMP> {
        VList {
            childs: self.iter().map(|c| c.into()).collect(),
        }
        .into()
    }
}

/// Should be rendered relative to context and component environment.
pub trait Renderable<COMP: Component> {
    /// Called by rendering loop.
    fn view(&self) -> Html<COMP>;
}

/// Trait for building properties for a component
pub trait Properties {
    /// Builder that will be used to construct properties
    type Builder;

    /// Entrypoint for building properties
    fn builder() -> Self::Builder;
}

/// Builder for when a component has no properties
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
    COMP: Component + Renderable<COMP>,
{
    /// Create link for a scope.
    fn connect(scope: &Scope<COMP>) -> Self {
        ComponentLink {
            scope: scope.clone(),
        }
    }

    /// This method sends messages back to the component's loop.
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

    /// This method sends a message to this component immediately.
    pub fn send_self(&mut self, msg: COMP::Message) {
        self.scope.send_message(msg);
    }
}

impl<COMP: Component> fmt::Debug for ComponentLink<COMP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ComponentLink<_>")
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
