//! Component children module

use std::fmt;
use std::rc::Rc;

use implicit_clone::{unsync::IArray, ImplicitClone};

use crate::html::Html;
use crate::utils::RcExt;
use crate::virtual_dom::{VChild, VComp, VList, VNode};
use crate::{BaseComponent, Properties};

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
pub struct ChildrenRenderer<T: Clone + 'static> {
    pub(crate) children: IArray<Rc<T>>,
}

impl<T: Clone> ImplicitClone for ChildrenRenderer<T> {}

impl<T: Clone + PartialEq> PartialEq for ChildrenRenderer<T> {
    fn eq(&self, other: &Self) -> bool {
        self.children == other.children
    }
}

impl<T> ChildrenRenderer<T>
where
    T: Clone + 'static,
{
    /// Create children
    pub fn new(children: Vec<T>) -> Self {
        Self {
            children: children.into_iter().map(Rc::new).collect(),
        }
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
        // TODO not sure if I shouldnt keep the Rc here
        self.children.iter().map(RcExt::unwrap_or_clone)
    }

    /// Convert the children elements to another object (if there are any).
    ///
    /// ```
    /// # let children = Children::new(Vec::new());
    /// # use yew::{classes, html, Children};
    /// # let _ =
    /// children.map(|children| {
    ///     html! {
    ///         <div class={classes!("container")}>
    ///             {children}
    ///         </div>
    ///     }
    /// })
    /// # ;
    /// ```
    pub fn map<OUT: Default>(&self, closure: impl FnOnce(&Self) -> OUT) -> OUT {
        if self.is_empty() {
            Default::default()
        } else {
            closure(self)
        }
    }

    pub(crate) fn to_vec(&self) -> Vec<T> {
        self.iter().collect()
    }
}

impl<T: Clone> Default for ChildrenRenderer<T> {
    fn default() -> Self {
        Self {
            children: Default::default(),
        }
    }
}

impl<T: Clone> fmt::Debug for ChildrenRenderer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ChildrenRenderer<_>")
    }
}

#[derive(Debug)]
#[doc(hidden)]
pub struct Iter<T: Clone + 'static> {
    children: implicit_clone::unsync::Iter<Rc<T>>,
}

impl<T: Clone + 'static> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.children.next().map(|x| RcExt::unwrap_or_clone(x))
    }
}

impl<T: ImplicitClone> IntoIterator for ChildrenRenderer<T> {
    type IntoIter = Iter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            children: self.children.iter(),
        }
    }
}

impl<T: Clone> FromIterator<T> for ChildrenRenderer<T> {
    fn from_iter<IT: IntoIterator<Item = T>>(it: IT) -> Self {
        Self {
            children: it.into_iter().map(Rc::new).collect(),
        }
    }
}

impl From<ChildrenRenderer<Html>> for Html {
    fn from(val: ChildrenRenderer<Html>) -> Self {
        if val.children.len() == 1 {
            return RcExt::unwrap_or_clone(val.children[0].clone());
        }

        Html::VList(Rc::new(val.into()))
    }
}

impl From<ChildrenRenderer<Html>> for VList {
    fn from(val: ChildrenRenderer<Html>) -> Self {
        if val.is_empty() {
            return VList::new();
        }
        VList::with_children(val.to_vec(), None)
    }
}

impl<COMP> From<ChildrenRenderer<VChild<COMP>>> for ChildrenRenderer<Html>
where
    COMP: BaseComponent,
{
    fn from(value: ChildrenRenderer<VChild<COMP>>) -> Self {
        Self::new(
            value
                .into_iter()
                .map(VComp::from)
                .map(VNode::from)
                .collect(),
        )
    }
}

/// A [Properties] type with Children being the only property.
#[derive(Debug, Properties, PartialEq)]
pub struct ChildrenProps {
    /// The Children of a Component.
    #[prop_or_default]
    pub children: Html,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn children_map() {
        let children = Children::new(Default::default());
        let res = children.map(|children| Some(children.clone()));
        assert!(res.is_none());
        let children = Children::new(vec![Default::default()]);
        let res = children.map(|children| Some(children.clone()));
        assert!(res.is_some());
    }
}
