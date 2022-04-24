//! Component children module

use std::fmt;

use crate::html::Html;
use crate::virtual_dom::{VChild, VNode};
use crate::Properties;

/// A type used for accepting children elements in Component::Properties.
///
/// # Example
/// **`model.rs`**
///
/// In this example, the `Wrapper` component is used to wrap other elements.
/// ```
/// # use yew::{Children, Html, Properties, Component, Context, html};
/// # #[derive(Clone, Properties, PartialEq)]
/// # struct WrapperProps {
/// #     children: Children,
/// # }
/// # struct Wrapper;
/// # impl Component for Wrapper {
/// #     type Message = ();
/// #     type Properties = WrapperProps;
/// #   fn create(ctx: &Context<Self>) -> Self { Self }
/// #   fn view(&self, ctx: &Context<Self>) -> Html {
/// html! {
///     <Wrapper>
///         <h4>{ "Hi" }</h4>
///         <div>{ "Hello" }</div>
///     </Wrapper>
/// }
/// #     }
/// # }
/// ```
///
/// **`wrapper.rs`**
///
/// The Wrapper component must define a `children` property in order to wrap other elements. The
/// children property can be used to render the wrapped elements.
/// ```
/// # use yew::{Children, Html, Properties, Component, Context, html};
/// #[derive(Clone, Properties, PartialEq)]
/// struct WrapperProps {
///     children: Children,
/// }
///
/// # struct Wrapper;
/// impl Component for Wrapper {
///     // ...
/// #     type Message = ();
/// #     type Properties = WrapperProps;
/// #    fn create(ctx: &Context<Self>) -> Self { Self }
///     fn view(&self, ctx: &Context<Self>) -> Html {
///         html! {
///             <div id="container">
///                 { ctx.props().children.clone() }
///             </div>
///         }
///     }
/// }
/// ```
pub type Children = ChildrenRenderer<Html>;

/// A type used for accepting children elements in Component::Properties and accessing their props.
///
/// # Example
/// **`model.rs`**
///
/// In this example, the `List` component can wrap `ListItem` components.
/// ```
/// # use yew::{html, Component, Html, Context, ChildrenWithProps, Properties};
/// #
/// # #[derive(Clone, Properties, PartialEq)]
/// # struct ListProps {
/// #     children: ChildrenWithProps<ListItem>,
/// # }
/// # struct List;
/// # impl Component for List {
/// #     type Message = ();
/// #     type Properties = ListProps;
/// #   fn create(ctx: &Context<Self>) -> Self { Self }
/// #   fn view(&self, ctx: &Context<Self>) -> Html { unimplemented!() }
/// # }
/// # #[derive(Clone, Properties, PartialEq)]
/// # struct ListItemProps {
/// #     value: String
/// # }
/// # struct ListItem;
/// # impl Component for ListItem {
/// #     type Message = ();
/// #     type Properties = ListItemProps;
/// #   fn create(ctx: &Context<Self>) -> Self { Self }
/// #   fn view(&self, ctx: &Context<Self>) -> Html { unimplemented!() }
/// # }
/// # fn view() -> Html {
/// html! {
///   <List>
///     <ListItem value="a" />
///     <ListItem value="b" />
///     <ListItem value="c" />
///   </List>
/// }
/// # }
/// ```
///
/// **`list.rs`**
///
/// The `List` component must define a `children` property in order to wrap the list items. The
/// `children` property can be used to filter, mutate, and render the items.
/// ```
/// # use yew::{html, Component, Html, ChildrenWithProps, Context, Properties};
/// # use std::rc::Rc;
/// #
/// #[derive(Clone, Properties, PartialEq)]
/// struct ListProps {
///     children: ChildrenWithProps<ListItem>,
/// }
///
/// # struct List;
/// impl Component for List {
/// #     type Message = ();
/// #     type Properties = ListProps;
/// #   fn create(ctx: &Context<Self>) -> Self { Self }
/// #   fn view(&self, ctx: &Context<Self>) -> Html {
///         html!{{
///             for ctx.props().children.iter().map(|mut item| {
///                 let mut props = Rc::make_mut(&mut item.props);
///                 props.value = format!("item-{}", props.value);
///                 item
///             })
///         }}
///     }
/// }
/// #
/// # #[derive(Clone, Properties, PartialEq)]
/// # struct ListItemProps {
/// #     #[prop_or_default]
/// #     value: String
/// # }
/// #
/// # struct ListItem;
/// # impl Component for ListItem {
/// #     type Message = ();
/// #     type Properties = ListItemProps;
/// #   fn create(ctx: &Context<Self>) -> Self { Self }
/// #   fn view(&self, ctx: &Context<Self>) -> Html { unimplemented!() }
/// # }
/// ```
pub type ChildrenWithProps<CHILD> = ChildrenRenderer<VChild<CHILD>>;

/// A type used for rendering children html.
#[derive(Clone)]
pub struct ChildrenRenderer<T> {
    children: Vec<T>,
}

impl<T: PartialEq> PartialEq for ChildrenRenderer<T> {
    fn eq(&self, other: &Self) -> bool {
        self.children == other.children
    }
}

impl<T> ChildrenRenderer<T>
where
    T: Clone + Into<VNode>,
{
    /// Create children
    pub fn new(children: Vec<T>) -> Self {
        Self { children }
    }

    /// Children list is empty
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    /// Number of children elements
    pub fn len(&self) -> usize {
        self.children.len()
    }

    /// Render children components and return `Iterator`
    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        // clone each child lazily.
        // This way `self.iter().next()` only has to clone a single node.
        self.children.iter().cloned()
    }
}

impl<T> Default for ChildrenRenderer<T> {
    fn default() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl<T> fmt::Debug for ChildrenRenderer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ChildrenRenderer<_>")
    }
}

impl<T> IntoIterator for ChildrenRenderer<T> {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.children.into_iter()
    }
}

/// A [Properties] type with Children being the only property.
#[derive(Debug, Properties, PartialEq)]
pub struct ChildrenProps {
    /// The Children of a Component.
    #[prop_or_default]
    pub children: Children,
}
