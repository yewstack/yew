use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;

use super::{VChild, VList, VNode, VText};
use crate::html::{ChildrenRenderer, IntoPropValue};
use crate::{AttrValue, BaseComponent, Html};

/// A trait implemented for types be rendered as a part of a Html.
///
/// Previously, a blanket implementation is given to `std::fmt::Display`
/// and that is always converted to a `VText`.
/// This trait allows types to define a virtual dom layout that itself should be rendered into via
/// `html!`.
pub trait Renderable {
    /// Converts this type to a [`Html`].
    fn to_html(&self) -> Html;

    /// Converts this type into a [`Html`].
    fn into_html(self) -> Html
    where
        Self: Sized,
    {
        self.to_html()
    }
}

// Implementations for common data types.

impl<T> Renderable for Option<T>
where
    T: Renderable,
{
    #[inline(always)]
    fn to_html(&self) -> Html {
        self.as_ref().map(Renderable::to_html).unwrap_or_default()
    }

    #[inline(always)]
    fn into_html(self) -> Html {
        self.map(Renderable::into_html).unwrap_or_default()
    }
}

impl<T> Renderable for Vec<T>
where
    T: Renderable,
{
    #[inline(always)]
    fn to_html(&self) -> Html {
        Html::VList(VList::with_children(
            self.iter().map(Renderable::to_html).collect(),
            None,
        ))
    }

    #[inline(always)]
    fn into_html(self) -> Html {
        Html::VList(VList::with_children(
            self.into_iter().map(Renderable::into_html).collect(),
            None,
        ))
    }
}

impl Renderable for Option<VNode> {
    #[inline(always)]
    fn to_html(&self) -> Html {
        self.clone().into_html()
    }

    #[inline(always)]
    fn into_html(self) -> Html {
        self.unwrap_or_default()
    }
}

impl Renderable for Vec<VNode> {
    #[inline(always)]
    fn to_html(&self) -> Html {
        self.clone().into_html()
    }

    #[inline(always)]
    fn into_html(self) -> Html {
        Html::VList(VList::with_children(self, None))
    }
}

impl Renderable for VText {
    #[inline(always)]
    fn to_html(&self) -> Html {
        self.clone().into()
    }

    #[inline(always)]
    fn into_html(self) -> Html {
        Html::VText(self)
    }
}

impl Renderable for VList {
    #[inline(always)]
    fn to_html(&self) -> Html {
        self.clone().into()
    }

    #[inline(always)]
    fn into_html(self) -> Html {
        Html::VList(self)
    }
}

impl Renderable for ChildrenRenderer<VNode> {
    #[inline(always)]
    fn to_html(&self) -> Html {
        self.clone().into()
    }

    #[inline(always)]
    fn into_html(self) -> Html {
        self.into()
    }
}

impl<T> Renderable for VChild<T>
where
    T: BaseComponent,
{
    #[inline(always)]
    fn to_html(&self) -> Html {
        self.clone().into()
    }

    #[inline(always)]
    fn into_html(self) -> Html {
        VNode::VComp(self.into())
    }
}

impl Renderable for () {
    #[inline(always)]
    fn to_html(&self) -> Html {
        VNode::default()
    }

    #[inline(always)]
    fn into_html(self) -> Html {
        VNode::default()
    }
}

impl<T> Renderable for &'_ T
where
    T: Renderable,
{
    fn to_html(&self) -> Html {
        (*self).to_html()
    }
}

macro_rules! impl_renderable_via_display {
    ($from_ty: ty) => {
        impl Renderable for $from_ty {
            #[inline(always)]
            fn to_html(&self) -> Html {
                Html::VText(self.into())
            }
        }

        // Mirror Renderable to Children implementation.
        impl IntoPropValue<ChildrenRenderer<VNode>> for $from_ty {
            #[inline(always)]
            fn into_prop_value(self) -> ChildrenRenderer<VNode> {
                ChildrenRenderer::new(vec![VText::from(self).into()])
            }
        }
    };
}

// These are a selection of types implemented via display.
impl_renderable_via_display!(bool);
impl_renderable_via_display!(char);
impl_renderable_via_display!(String);
impl_renderable_via_display!(&str);
impl_renderable_via_display!(Rc<str>);
impl_renderable_via_display!(Rc<String>);
impl_renderable_via_display!(Arc<str>);
impl_renderable_via_display!(Arc<String>);
impl_renderable_via_display!(AttrValue);
impl_renderable_via_display!(Cow<'_, str>);
impl_renderable_via_display!(u8);
impl_renderable_via_display!(u16);
impl_renderable_via_display!(u32);
impl_renderable_via_display!(u64);
impl_renderable_via_display!(u128);
impl_renderable_via_display!(usize);
impl_renderable_via_display!(i8);
impl_renderable_via_display!(i16);
impl_renderable_via_display!(i32);
impl_renderable_via_display!(i64);
impl_renderable_via_display!(i128);
impl_renderable_via_display!(isize);
impl_renderable_via_display!(f32);
impl_renderable_via_display!(f64);
