use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;

use implicit_clone::unsync::{IArray, IMap};
pub use implicit_clone::ImplicitClone;

use crate::callback::Callback;
use crate::html::{BaseComponent, ChildrenRenderer, Component, NodeRef, Scope};
use crate::virtual_dom::{AttrValue, VChild, VList, VNode, VText};

impl ImplicitClone for NodeRef {}
impl<Comp: Component> ImplicitClone for Scope<Comp> {}
// TODO there are still a few missing

/// A trait similar to `Into<T>` which allows conversion to a value of a `Properties` struct.
pub trait IntoPropValue<T> {
    /// Convert `self` to a value of a `Properties` struct.
    fn into_prop_value(self) -> T;
}

impl<T> IntoPropValue<T> for T {
    #[inline]
    fn into_prop_value(self) -> T {
        self
    }
}

impl<T> IntoPropValue<T> for &T
where
    T: ImplicitClone,
{
    #[inline]
    fn into_prop_value(self) -> T {
        self.clone()
    }
}

impl<T> IntoPropValue<Option<T>> for T {
    #[inline]
    fn into_prop_value(self) -> Option<T> {
        Some(self)
    }
}

impl<T> IntoPropValue<Option<T>> for &T
where
    T: ImplicitClone,
{
    #[inline]
    fn into_prop_value(self) -> Option<T> {
        Some(self.clone())
    }
}

impl<I, O, F> IntoPropValue<Callback<I, O>> for F
where
    F: 'static + Fn(I) -> O,
{
    #[inline]
    fn into_prop_value(self) -> Callback<I, O> {
        Callback::from(self)
    }
}

impl<I, O, F> IntoPropValue<Option<Callback<I, O>>> for F
where
    F: 'static + Fn(I) -> O,
{
    #[inline]
    fn into_prop_value(self) -> Option<Callback<I, O>> {
        Some(Callback::from(self))
    }
}

impl<I, O, F> IntoPropValue<Option<Callback<I, O>>> for Option<F>
where
    F: 'static + Fn(I) -> O,
{
    #[inline]
    fn into_prop_value(self) -> Option<Callback<I, O>> {
        self.map(Callback::from)
    }
}

impl<T, C> IntoPropValue<ChildrenRenderer<C>> for VChild<T>
where
    T: BaseComponent,
    C: Clone + Into<VNode>,
    VChild<T>: Into<C>,
{
    #[inline]
    fn into_prop_value(self) -> ChildrenRenderer<C> {
        ChildrenRenderer::new(vec![self.into()])
    }
}

impl<T, C> IntoPropValue<Option<ChildrenRenderer<C>>> for VChild<T>
where
    T: BaseComponent,
    C: Clone + Into<VNode>,
    VChild<T>: Into<C>,
{
    #[inline]
    fn into_prop_value(self) -> Option<ChildrenRenderer<C>> {
        Some(ChildrenRenderer::new(vec![self.into()]))
    }
}

impl<T, C> IntoPropValue<Option<ChildrenRenderer<C>>> for Option<VChild<T>>
where
    T: BaseComponent,
    C: Clone + Into<VNode>,
    VChild<T>: Into<C>,
{
    #[inline]
    fn into_prop_value(self) -> Option<ChildrenRenderer<C>> {
        self.map(|m| ChildrenRenderer::new(vec![m.into()]))
    }
}

impl<T, R> IntoPropValue<ChildrenRenderer<R>> for Vec<T>
where
    T: Into<R>,
    R: Clone + Into<VNode>,
{
    #[inline]
    fn into_prop_value(self) -> ChildrenRenderer<R> {
        ChildrenRenderer::new(self.into_iter().map(|m| m.into()).collect())
    }
}

impl<T> IntoPropValue<VNode> for VChild<T>
where
    T: BaseComponent,
{
    #[inline]
    fn into_prop_value(self) -> VNode {
        VNode::from(self)
    }
}

impl IntoPropValue<VNode> for VList {
    #[inline]
    fn into_prop_value(self) -> VNode {
        VNode::VList(Rc::new(self))
    }
}
impl IntoPropValue<VNode> for VText {
    #[inline]
    fn into_prop_value(self) -> VNode {
        VNode::VText(self)
    }
}

impl IntoPropValue<VNode> for () {
    #[inline]
    fn into_prop_value(self) -> VNode {
        VNode::default()
    }
}

impl IntoPropValue<VNode> for ChildrenRenderer<VNode> {
    #[inline]
    fn into_prop_value(self) -> VNode {
        VNode::VList(Rc::new(self.into()))
    }
}

impl IntoPropValue<ChildrenRenderer<VNode>> for VNode {
    #[inline]
    fn into_prop_value(self) -> ChildrenRenderer<VNode> {
        ChildrenRenderer::new(vec![self])
    }
}

impl IntoPropValue<ChildrenRenderer<VNode>> for VText {
    #[inline]
    fn into_prop_value(self) -> ChildrenRenderer<VNode> {
        ChildrenRenderer::new(vec![self.into()])
    }
}

impl IntoPropValue<VList> for ChildrenRenderer<VNode> {
    #[inline]
    fn into_prop_value(self) -> VList {
        VList::with_children(self.children, None)
    }
}

impl<C: BaseComponent> IntoPropValue<VList> for VChild<C> {
    #[inline]
    fn into_prop_value(self) -> VList {
        VList::with_children(vec![self.into()], None)
    }
}

impl IntoPropValue<ChildrenRenderer<VNode>> for AttrValue {
    fn into_prop_value(self) -> ChildrenRenderer<VNode> {
        ChildrenRenderer::new(vec![VNode::VText(VText::new(self))])
    }
}

impl IntoPropValue<VNode> for Vec<VNode> {
    #[inline]
    fn into_prop_value(self) -> VNode {
        VNode::VList(Rc::new(VList::with_children(self, None)))
    }
}

impl IntoPropValue<VNode> for Option<VNode> {
    #[inline]
    fn into_prop_value(self) -> VNode {
        self.unwrap_or_default()
    }
}

macro_rules! impl_into_prop {
    (|$value:ident: $from_ty:ty| -> $to_ty:ty { $conversion:expr }) => {
        // implement V -> T
        impl IntoPropValue<$to_ty> for $from_ty {
            #[inline]
            fn into_prop_value(self) -> $to_ty {
                let $value = self;
                $conversion
            }
        }
        // implement V -> Option<T>
        impl IntoPropValue<Option<$to_ty>> for $from_ty {
            #[inline]
            fn into_prop_value(self) -> Option<$to_ty> {
                let $value = self;
                Some({ $conversion })
            }
        }
        // implement Option<V> -> Option<T>
        impl IntoPropValue<Option<$to_ty>> for Option<$from_ty> {
            #[inline]
            fn into_prop_value(self) -> Option<$to_ty> {
                self.map(IntoPropValue::into_prop_value)
            }
        }
    };
}

// implemented with literals in mind
impl_into_prop!(|value: &'static str| -> String { value.to_owned() });
impl_into_prop!(|value: &'static str| -> AttrValue { AttrValue::Static(value) });
impl_into_prop!(|value: String| -> AttrValue { AttrValue::Rc(Rc::from(value)) });
impl_into_prop!(|value: Rc<str>| -> AttrValue { AttrValue::Rc(value) });
impl_into_prop!(|value: Cow<'static, str>| -> AttrValue { AttrValue::from(value) });

impl<T: ImplicitClone + 'static> IntoPropValue<IArray<T>> for &'static [T] {
    fn into_prop_value(self) -> IArray<T> {
        IArray::from(self)
    }
}

impl<T: ImplicitClone + 'static> IntoPropValue<IArray<T>> for Vec<T> {
    fn into_prop_value(self) -> IArray<T> {
        IArray::from(self)
    }
}

impl<K: Eq + std::hash::Hash + ImplicitClone + 'static, V: PartialEq + ImplicitClone + 'static>
    IntoPropValue<IMap<K, V>> for &'static [(K, V)]
{
    fn into_prop_value(self) -> IMap<K, V> {
        IMap::from(self)
    }
}

impl<K: Eq + std::hash::Hash + ImplicitClone + 'static, V: PartialEq + ImplicitClone + 'static>
    IntoPropValue<IMap<K, V>> for indexmap::IndexMap<K, V>
{
    fn into_prop_value(self) -> IMap<K, V> {
        IMap::from(self)
    }
}

macro_rules! impl_into_prop_value_via_display {
    ($from_ty: ty) => {
        impl IntoPropValue<VNode> for $from_ty {
            #[inline(always)]
            fn into_prop_value(self) -> VNode {
                VText::from(self).into()
            }
        }
    };
}

// go through AttrValue::from where possible
macro_rules! impl_into_prop_value_via_attr_value {
    ($from_ty: ty) => {
        impl IntoPropValue<VNode> for $from_ty {
            #[inline(always)]
            fn into_prop_value(self) -> VNode {
                VText::new(self).into()
            }
        }
    };
}

// These are a selection of types implemented via display.
impl_into_prop_value_via_display!(bool);
impl_into_prop_value_via_display!(char);
impl_into_prop_value_via_display!(&String);
impl_into_prop_value_via_display!(&str);
impl_into_prop_value_via_display!(Arc<str>);
impl_into_prop_value_via_display!(Arc<String>);
impl_into_prop_value_via_display!(Rc<String>);
impl_into_prop_value_via_display!(u8);
impl_into_prop_value_via_display!(u16);
impl_into_prop_value_via_display!(u32);
impl_into_prop_value_via_display!(u64);
impl_into_prop_value_via_display!(u128);
impl_into_prop_value_via_display!(usize);
impl_into_prop_value_via_display!(i8);
impl_into_prop_value_via_display!(i16);
impl_into_prop_value_via_display!(i32);
impl_into_prop_value_via_display!(i64);
impl_into_prop_value_via_display!(i128);
impl_into_prop_value_via_display!(isize);
impl_into_prop_value_via_display!(f32);
impl_into_prop_value_via_display!(f64);

impl_into_prop_value_via_attr_value!(String);
impl_into_prop_value_via_attr_value!(AttrValue);
impl_into_prop_value_via_attr_value!(Rc<str>);
impl_into_prop_value_via_attr_value!(Cow<'static, str>);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str() {
        let _: String = "foo".into_prop_value();
        let _: Option<String> = "foo".into_prop_value();
        let _: AttrValue = "foo".into_prop_value();
        let _: Option<AttrValue> = "foo".into_prop_value();
        let _: Option<AttrValue> = Rc::<str>::from("foo").into_prop_value();
        let _: Option<AttrValue> = Cow::Borrowed("foo").into_prop_value();
    }

    #[test]
    fn test_callback() {
        let _: Callback<String> = (|_: String| ()).into_prop_value();
        let _: Option<Callback<String>> = (|_: String| ()).into_prop_value();
        let _: Option<Callback<String>> = Some(|_: String| ()).into_prop_value();
        let _: Callback<String, String> = (|s: String| s).into_prop_value();
        let _: Option<Callback<String, String>> = (|s: String| s).into_prop_value();
        let _: Option<Callback<String, String>> = Some(|s: String| s).into_prop_value();
    }

    #[test]
    fn test_html_to_children_compiles() {
        use crate::prelude::*;

        #[derive(Clone, Debug, PartialEq, Properties)]
        pub struct Props {
            #[prop_or_default]
            pub header: Children,
            #[prop_or_default]
            pub children: Children,
            #[prop_or_default]
            pub footer: Children,
        }

        #[function_component]
        pub fn App(props: &Props) -> Html {
            let Props {
                header,
                children,
                footer,
            } = props.clone();

            html! {
                <div>
                    <header>
                        {header}
                    </header>
                    <main>
                        {children}
                    </main>
                    <footer>
                        {footer}
                    </footer>
                </div>
            }
        }

        let header = html! { <div>{"header"}</div> };
        let footer = html! { <div>{"footer"}</div> };
        let children = html! { <div>{"main"}</div> };

        let _ = html! {
            <App {header} {footer}>
                {children}
            </App>
        };
    }

    #[test]
    fn test_vchild_to_children_with_props_compiles() {
        use crate::prelude::*;

        #[function_component]
        pub fn Comp() -> Html {
            Html::default()
        }

        #[derive(Clone, Debug, PartialEq, Properties)]
        pub struct Props {
            #[prop_or_default]
            pub header: ChildrenWithProps<Comp>,
            #[prop_or_default]
            pub children: Children,
            #[prop_or_default]
            pub footer: ChildrenWithProps<Comp>,
        }

        #[function_component]
        pub fn App(props: &Props) -> Html {
            let Props {
                header,
                children,
                footer,
            } = props.clone();

            html! {
                <div>
                    <header>
                        {for header}
                    </header>
                    <main>
                        {children}
                    </main>
                    <footer>
                        {for footer}
                    </footer>
                </div>
            }
        }

        let header = VChild::new((), None);
        let footer = html_nested! { <Comp /> };
        let children = html! { <div>{"main"}</div> };

        let _ = html! {
            <App {header} {footer}>
                {children}
            </App>
        };
    }

    #[test]
    fn test_vlist_to_children_compiles() {
        use crate::prelude::*;
        use crate::virtual_dom::VList;

        #[function_component]
        fn Foo() -> Html {
            todo!()
        }

        #[derive(PartialEq, Properties)]
        pub struct ChildProps {
            #[prop_or_default]
            pub children: Html,
        }

        #[function_component]
        fn Child(_props: &ChildProps) -> Html {
            html!()
        }

        #[derive(PartialEq, Properties)]
        pub struct ParentProps {
            pub children: VList,
        }

        #[function_component]
        fn Parent(_props: &ParentProps) -> Html {
            todo!()
        }

        let _ = html! {
            <Parent>
                <Child></Child>
            </Parent>
        };

        let _ = html! {
            <Parent>
                <Child />
                <Child />
            </Parent>
        };

        let _ = html! {
            <Parent>
                <Child>
                    <Foo />
                </Child>
            </Parent>
        };
    }

    #[test]
    fn attr_value_children() {
        use crate::prelude::*;

        #[derive(PartialEq, Properties)]
        pub struct ChildProps {
            #[prop_or_default]
            pub children: AttrValue,
        }

        #[function_component]
        fn Child(_props: &ChildProps) -> Html {
            html!()
        }
        {
            let attr_value = AttrValue::from("foo");

            let _ = html! { <Child>{attr_value}</Child> };
        }
        {
            let attr_value = AttrValue::from("foo");

            let _ = html! { <Child>{&attr_value}</Child> };
        }
    }
}
