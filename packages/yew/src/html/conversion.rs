use std::rc::Rc;

use implicit_clone::unsync::{IArray, IMap};
pub use implicit_clone::ImplicitClone;

use super::super::callback::Callback;
use super::{BaseComponent, Children, ChildrenRenderer, Component, NodeRef, Scope};
use crate::virtual_dom::{AttrValue, VChild, VNode};

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

impl<T> IntoPropValue<ChildrenRenderer<VChild<T>>> for VChild<T>
where
    T: BaseComponent,
{
    #[inline]
    fn into_prop_value(self) -> ChildrenRenderer<VChild<T>> {
        ChildrenRenderer::new(vec![self])
    }
}

impl<T> IntoPropValue<Option<ChildrenRenderer<VChild<T>>>> for VChild<T>
where
    T: BaseComponent,
{
    #[inline]
    fn into_prop_value(self) -> Option<ChildrenRenderer<VChild<T>>> {
        Some(ChildrenRenderer::new(vec![self]))
    }
}

impl<T> IntoPropValue<Option<ChildrenRenderer<VChild<T>>>> for Option<VChild<T>>
where
    T: BaseComponent,
{
    #[inline]
    fn into_prop_value(self) -> Option<ChildrenRenderer<VChild<T>>> {
        self.map(|m| ChildrenRenderer::new(vec![m]))
    }
}

impl<T> IntoPropValue<ChildrenRenderer<VChild<T>>> for Vec<VChild<T>>
where
    T: BaseComponent,
{
    #[inline]
    fn into_prop_value(self) -> ChildrenRenderer<VChild<T>> {
        ChildrenRenderer::new(self)
    }
}

impl<T> IntoPropValue<Option<ChildrenRenderer<VChild<T>>>> for Vec<VChild<T>>
where
    T: BaseComponent,
{
    #[inline]
    fn into_prop_value(self) -> Option<ChildrenRenderer<VChild<T>>> {
        Some(ChildrenRenderer::new(self))
    }
}

impl<T> IntoPropValue<Option<ChildrenRenderer<VChild<T>>>> for Option<Vec<VChild<T>>>
where
    T: BaseComponent,
{
    #[inline]
    fn into_prop_value(self) -> Option<ChildrenRenderer<VChild<T>>> {
        self.map(ChildrenRenderer::new)
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
impl_into_prop!(|value: VNode| -> Children { Children::new(vec![value]) });

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

        let header = VChild::new((), None);
        let footer = html_nested! { <Comp /> };
        let children = html! { <div>{"main"}</div> };

        let _ = html! {
            <App {header} {footer}>
                {children}
            </App>
        };
    }
}
