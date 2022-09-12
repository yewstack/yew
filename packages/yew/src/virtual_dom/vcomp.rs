//! This module contains the implementation of a virtual component (`VComp`).

use std::any::TypeId;
use std::fmt;
use std::rc::Rc;

#[cfg(feature = "ssr")]
use futures::future::{FutureExt, LocalBoxFuture};
#[cfg(feature = "csr")]
use web_sys::Element;

use super::Key;
#[cfg(feature = "csr")]
use crate::dom_bundle::BSubtree;
#[cfg(feature = "hydration")]
use crate::dom_bundle::Fragment;
use crate::html::BaseComponent;
#[cfg(any(feature = "ssr", feature = "csr"))]
use crate::html::{AnyScope, Scope};
#[cfg(feature = "csr")]
use crate::html::{NodeRef, Scoped};
#[cfg(feature = "ssr")]
use crate::platform::fmt::BufWriter;

/// A virtual component.
pub struct VComp {
    pub(crate) type_id: TypeId,
    pub(crate) mountable: Box<dyn Mountable>,
    pub(crate) key: Option<Key>,
    // for some reason, this reduces the bundle size by ~2-3 KBs
    _marker: u32,
}

impl fmt::Debug for VComp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VComp")
            .field("type_id", &self.type_id)
            .field("mountable", &"..")
            .field("key", &self.key)
            .finish()
    }
}

impl Clone for VComp {
    fn clone(&self) -> Self {
        Self {
            type_id: self.type_id,
            mountable: self.mountable.copy(),
            key: self.key.clone(),
            _marker: 0,
        }
    }
}

pub(crate) trait Mountable {
    fn copy(&self) -> Box<dyn Mountable>;

    #[cfg(feature = "csr")]
    fn mount(
        self: Box<Self>,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: Element,
        internal_ref: NodeRef,
        next_sibling: NodeRef,
    ) -> Box<dyn Scoped>;

    #[cfg(feature = "csr")]
    fn reuse(self: Box<Self>, scope: &dyn Scoped, next_sibling: NodeRef);

    #[cfg(feature = "ssr")]
    fn render_into_stream<'a>(
        &'a self,
        w: &'a mut BufWriter,
        parent_scope: &'a AnyScope,
        hydratable: bool,
    ) -> LocalBoxFuture<'a, ()>;

    #[cfg(feature = "hydration")]
    fn hydrate(
        self: Box<Self>,
        root: BSubtree,
        parent_scope: &AnyScope,
        parent: Element,
        internal_ref: NodeRef,
        fragment: &mut Fragment,
    ) -> Box<dyn Scoped>;
}

pub(crate) struct PropsWrapper<COMP: BaseComponent> {
    props: Rc<COMP::Properties>,
}

impl<COMP: BaseComponent> PropsWrapper<COMP> {
    pub fn new(props: Rc<COMP::Properties>) -> Self {
        Self { props }
    }
}

impl<COMP: BaseComponent> Mountable for PropsWrapper<COMP> {
    fn copy(&self) -> Box<dyn Mountable> {
        let wrapper: PropsWrapper<COMP> = PropsWrapper {
            props: Rc::clone(&self.props),
        };
        Box::new(wrapper)
    }

    #[cfg(feature = "csr")]
    fn mount(
        self: Box<Self>,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: Element,
        internal_ref: NodeRef,
        next_sibling: NodeRef,
    ) -> Box<dyn Scoped> {
        let scope: Scope<COMP> = Scope::new(Some(parent_scope.clone()));
        scope.mount_in_place(root.clone(), parent, next_sibling, internal_ref, self.props);

        Box::new(scope)
    }

    #[cfg(feature = "csr")]
    fn reuse(self: Box<Self>, scope: &dyn Scoped, next_sibling: NodeRef) {
        let scope: Scope<COMP> = scope.to_any().downcast::<COMP>();
        scope.reuse(self.props, next_sibling);
    }

    #[cfg(feature = "ssr")]
    fn render_into_stream<'a>(
        &'a self,
        w: &'a mut BufWriter,
        parent_scope: &'a AnyScope,
        hydratable: bool,
    ) -> LocalBoxFuture<'a, ()> {
        let scope: Scope<COMP> = Scope::new(Some(parent_scope.clone()));

        async move {
            scope
                .render_into_stream(w, self.props.clone(), hydratable)
                .await;
        }
        .boxed_local()
    }

    #[cfg(feature = "hydration")]
    fn hydrate(
        self: Box<Self>,
        root: BSubtree,
        parent_scope: &AnyScope,
        parent: Element,
        internal_ref: NodeRef,
        fragment: &mut Fragment,
    ) -> Box<dyn Scoped> {
        let scope: Scope<COMP> = Scope::new(Some(parent_scope.clone()));
        scope.hydrate_in_place(root, parent, fragment, internal_ref, self.props);

        Box::new(scope)
    }
}

/// A virtual child component.
pub struct VChild<COMP: BaseComponent> {
    /// The component properties
    pub props: Rc<COMP::Properties>,
    /// Reference to the mounted node
    key: Option<Key>,
}

impl<COMP: BaseComponent> Clone for VChild<COMP> {
    fn clone(&self) -> Self {
        VChild {
            props: Rc::clone(&self.props),
            key: self.key.clone(),
        }
    }
}

impl<COMP: BaseComponent> PartialEq for VChild<COMP>
where
    COMP::Properties: PartialEq,
{
    fn eq(&self, other: &VChild<COMP>) -> bool {
        self.props == other.props
    }
}

impl<COMP> VChild<COMP>
where
    COMP: BaseComponent,
{
    /// Creates a child component that can be accessed and modified by its parent.
    pub fn new(props: COMP::Properties, key: Option<Key>) -> Self {
        Self {
            props: Rc::new(props),
            key,
        }
    }
}

impl<COMP> From<VChild<COMP>> for VComp
where
    COMP: BaseComponent,
{
    fn from(vchild: VChild<COMP>) -> Self {
        VComp::new::<COMP>(vchild.props, vchild.key)
    }
}

impl VComp {
    /// Creates a new `VComp` instance.
    pub fn new<COMP>(props: Rc<COMP::Properties>, key: Option<Key>) -> Self
    where
        COMP: BaseComponent,
    {
        VComp {
            type_id: TypeId::of::<COMP>(),
            mountable: Box::new(PropsWrapper::<COMP>::new(props)),
            key,
            _marker: 0,
        }
    }
}

impl PartialEq for VComp {
    fn eq(&self, other: &VComp) -> bool {
        self.type_id == other.type_id
    }
}

impl<COMP: BaseComponent> fmt::Debug for VChild<COMP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("VChild<_>")
    }
}

#[cfg(feature = "ssr")]
mod feat_ssr {
    use super::*;
    use crate::html::AnyScope;

    impl VComp {
        #[inline]
        pub(crate) async fn render_into_stream(
            &self,
            w: &mut BufWriter,
            parent_scope: &AnyScope,
            hydratable: bool,
        ) {
            self.mountable
                .as_ref()
                .render_into_stream(w, parent_scope, hydratable)
                .await;
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32"), feature = "ssr"))]
mod ssr_tests {
    use tokio::test;

    use crate::prelude::*;
    use crate::ServerRenderer;

    #[test]
    async fn test_props() {
        #[derive(PartialEq, Properties, Debug)]
        struct ChildProps {
            name: String,
        }

        #[function_component]
        fn Child(props: &ChildProps) -> Html {
            html! { <div>{"Hello, "}{&props.name}{"!"}</div> }
        }

        #[function_component]
        fn Comp() -> Html {
            html! {
                <div>
                    <Child name="Jane" />
                    <Child name="John" />
                    <Child name="Josh" />
                </div>
            }
        }

        let s = ServerRenderer::<Comp>::new()
            .hydratable(false)
            .render()
            .await;

        assert_eq!(
            s,
            "<div><div>Hello, Jane!</div><div>Hello, John!</div><div>Hello, Josh!</div></div>"
        );
    }
}
