use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;

use crate::html::{ChildrenRenderer, IntoPropValue};
use crate::virtual_dom::{VChild, VList, VNode, VText};
use crate::{AttrValue, BaseComponent, Html};

/// A trait implemented for types be rendered as a part of a Html.
///
/// Types that implements this trait can define a virtual dom layout that itself should be rendered
/// into via `html!` and can be referenced / consumed as `{value}` in an `html!` macro invocation.
pub trait ToHtml {
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

impl<T> ToHtml for Option<T>
where
    T: ToHtml,
{
    #[inline(always)]
    fn to_html(&self) -> Html {
        self.as_ref().map(ToHtml::to_html).unwrap_or_default()
    }

    #[inline(always)]
    fn into_html(self) -> Html {
        self.map(ToHtml::into_html).unwrap_or_default()
    }
}

impl<T> ToHtml for Vec<T>
where
    T: ToHtml,
{
    #[inline(always)]
    fn to_html(&self) -> Html {
        Html::VList(VList::with_children(
            self.iter().map(ToHtml::to_html).collect(),
            None,
        ))
    }

    #[inline(always)]
    fn into_html(self) -> Html {
        Html::VList(VList::with_children(
            self.into_iter().map(ToHtml::into_html).collect(),
            None,
        ))
    }
}

impl ToHtml for Option<VNode> {
    #[inline(always)]
    fn to_html(&self) -> Html {
        self.clone().into_html()
    }

    #[inline(always)]
    fn into_html(self) -> Html {
        self.unwrap_or_default()
    }
}

impl ToHtml for Vec<VNode> {
    #[inline(always)]
    fn to_html(&self) -> Html {
        self.clone().into_html()
    }

    #[inline(always)]
    fn into_html(self) -> Html {
        Html::VList(VList::with_children(self, None))
    }
}

impl ToHtml for VText {
    #[inline(always)]
    fn to_html(&self) -> Html {
        self.clone().into()
    }

    #[inline(always)]
    fn into_html(self) -> Html {
        Html::VText(self)
    }
}

impl ToHtml for VList {
    #[inline(always)]
    fn to_html(&self) -> Html {
        self.clone().into()
    }

    #[inline(always)]
    fn into_html(self) -> Html {
        Html::VList(self)
    }
}

impl ToHtml for ChildrenRenderer<VNode> {
    #[inline(always)]
    fn to_html(&self) -> Html {
        self.clone().into()
    }

    #[inline(always)]
    fn into_html(self) -> Html {
        self.into()
    }
}

impl<T> ToHtml for VChild<T>
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

impl ToHtml for () {
    #[inline(always)]
    fn to_html(&self) -> Html {
        VNode::default()
    }

    #[inline(always)]
    fn into_html(self) -> Html {
        VNode::default()
    }
}

impl<T> ToHtml for &'_ T
where
    T: ToHtml,
{
    fn to_html(&self) -> Html {
        (*self).to_html()
    }
}

macro_rules! impl_to_html_via_display {
    ($from_ty: ty) => {
        impl ToHtml for $from_ty {
            #[inline(always)]
            fn to_html(&self) -> Html {
                Html::VText(VText::from(self))
            }
        }

        // Mirror ToHtml to Children implementation.
        impl IntoPropValue<ChildrenRenderer<VNode>> for $from_ty {
            #[inline(always)]
            fn into_prop_value(self) -> ChildrenRenderer<VNode> {
                ChildrenRenderer::new(vec![VText::from(self).into()])
            }
        }
    };
}

// These are a selection of types implemented via display.
impl_to_html_via_display!(bool);
impl_to_html_via_display!(char);
impl_to_html_via_display!(String);
impl_to_html_via_display!(&str);
impl_to_html_via_display!(Rc<str>);
impl_to_html_via_display!(Rc<String>);
impl_to_html_via_display!(Arc<str>);
impl_to_html_via_display!(Arc<String>);
impl_to_html_via_display!(AttrValue);
impl_to_html_via_display!(Cow<'_, str>);
impl_to_html_via_display!(u8);
impl_to_html_via_display!(u16);
impl_to_html_via_display!(u32);
impl_to_html_via_display!(u64);
impl_to_html_via_display!(u128);
impl_to_html_via_display!(usize);
impl_to_html_via_display!(i8);
impl_to_html_via_display!(i16);
impl_to_html_via_display!(i32);
impl_to_html_via_display!(i64);
impl_to_html_via_display!(i128);
impl_to_html_via_display!(isize);
impl_to_html_via_display!(f32);
impl_to_html_via_display!(f64);
