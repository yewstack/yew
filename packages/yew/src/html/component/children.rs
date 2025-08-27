//! Component children module

use std::fmt;
use std::rc::Rc;

use crate::html::{Html, ImplicitClone};
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
pub struct ChildrenRenderer<T> {
    pub(crate) children: Option<Rc<Vec<T>>>,
}

impl<T> Clone for ChildrenRenderer<T> {
    fn clone(&self) -> Self {
        Self {
            children: self.children.clone(),
        }
    }
}

impl<T> ImplicitClone for ChildrenRenderer<T> {}

impl<T: PartialEq> PartialEq for ChildrenRenderer<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self.children.as_ref(), other.children.as_ref()) {
            (Some(a), Some(b)) => a == b,
            (Some(a), None) => a.is_empty(),
            (None, Some(b)) => b.is_empty(),
            (None, None) => true,
        }
    }
}

impl<T> ChildrenRenderer<T>
where
    T: Clone,
{
    /// Create children
    pub fn new(children: Vec<T>) -> Self {
        if children.is_empty() {
            Self { children: None }
        } else {
            Self {
                children: Some(Rc::new(children)),
            }
        }
    }

    /// Children list is empty
    pub fn is_empty(&self) -> bool {
        self.children.as_ref().map(|x| x.is_empty()).unwrap_or(true)
    }

    /// Number of children elements
    pub fn len(&self) -> usize {
        self.children.as_ref().map(|x| x.len()).unwrap_or(0)
    }

    /// Render children components and return `Iterator`
    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        // clone each child lazily.
        // This way `self.iter().next()` only has to clone a single node.
        self.children.iter().flat_map(|x| x.iter()).cloned()
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
}

impl<T> Default for ChildrenRenderer<T> {
    fn default() -> Self {
        Self {
            children: Default::default(),
        }
    }
}

impl<T> fmt::Debug for ChildrenRenderer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ChildrenRenderer<_>")
    }
}

impl<T: Clone> IntoIterator for ChildrenRenderer<T> {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        if let Some(children) = self.children {
            let children = RcExt::unwrap_or_clone(children);
            children.into_iter()
        } else {
            Vec::new().into_iter()
        }
    }
}

impl From<ChildrenRenderer<Html>> for Html {
    fn from(mut val: ChildrenRenderer<Html>) -> Self {
        if let Some(children) = val.children.as_mut() {
            if children.len() == 1 {
                let children = Rc::make_mut(children);
                if let Some(m) = children.pop() {
                    return m;
                }
            }
        }

        Html::VList(Rc::new(val.into()))
    }
}

impl From<ChildrenRenderer<Html>> for VList {
    fn from(val: ChildrenRenderer<Html>) -> Self {
        VList::from(val.children)
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
                .collect::<Vec<_>>(),
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
        let children = Children::new(vec![]);
        let res = children.map(|children| Some(children.clone()));
        assert!(res.is_none());
        let children = Children::new(vec![Default::default()]);
        let res = children.map(|children| Some(children.clone()));
        assert!(res.is_some());
    }
}
