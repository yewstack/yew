//! This module contains the implementation of a virtual component (`VComp`).

use super::Key;
use crate::html::{BaseComponent, IntoComponent, NodeRef};
use std::any::TypeId;
use std::fmt;
use std::rc::Rc;

#[cfg(any(feature = "ssr", feature = "render"))]
use crate::html::{AnyScope, Scope};

#[cfg(feature = "render")]
use crate::html::Scoped;
#[cfg(feature = "render")]
use web_sys::Element;

#[cfg(feature = "ssr")]
use futures::future::{FutureExt, LocalBoxFuture};

/// A virtual component.
pub struct VComp {
    pub(crate) type_id: TypeId,
    pub(crate) mountable: Box<dyn Mountable>,
    pub(crate) node_ref: NodeRef,
    pub(crate) key: Option<Key>,
}

impl fmt::Debug for VComp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VComp")
            .field("type_id", &self.type_id)
            .field("node_ref", &self.node_ref)
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
            node_ref: self.node_ref.clone(),
            key: self.key.clone(),
        }
    }
}

pub(crate) trait Mountable {
    fn copy(&self) -> Box<dyn Mountable>;

    #[cfg(feature = "render")]
    fn mount(
        self: Box<Self>,
        node_ref: NodeRef,
        parent_scope: &AnyScope,
        parent: Element,
        next_sibling: NodeRef,
    ) -> Box<dyn Scoped>;

    #[cfg(feature = "render")]
    fn reuse(self: Box<Self>, node_ref: NodeRef, scope: &dyn Scoped, next_sibling: NodeRef);

    #[cfg(feature = "ssr")]
    fn render_to_string<'a>(
        &'a self,
        w: &'a mut String,
        parent_scope: &'a AnyScope,
        hydratable: bool,
    ) -> LocalBoxFuture<'a, ()>;
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

    #[cfg(feature = "render")]
    fn mount(
        self: Box<Self>,
        node_ref: NodeRef,
        parent_scope: &AnyScope,
        parent: Element,
        next_sibling: NodeRef,
    ) -> Box<dyn Scoped> {
        let scope: Scope<COMP> = Scope::new(Some(parent_scope.clone()));
        scope.mount_in_place(parent, next_sibling, node_ref, self.props);

        Box::new(scope)
    }

    #[cfg(feature = "render")]
    fn reuse(self: Box<Self>, node_ref: NodeRef, scope: &dyn Scoped, next_sibling: NodeRef) {
        let scope: Scope<COMP> = scope.to_any().downcast::<COMP>();
        scope.reuse(self.props, node_ref, next_sibling);
    }

    #[cfg(feature = "ssr")]
    fn render_to_string<'a>(
        &'a self,
        w: &'a mut String,
        parent_scope: &'a AnyScope,
        hydratable: bool,
    ) -> LocalBoxFuture<'a, ()> {
        async move {
            let scope: Scope<COMP> = Scope::new(Some(parent_scope.clone()));
            scope
                .render_to_string(w, self.props.clone(), hydratable)
                .await;
        }
        .boxed_local()
    }
}

/// A virtual child component.
pub struct VChild<ICOMP: IntoComponent> {
    /// The component properties
    pub props: Rc<ICOMP::Properties>,
    /// Reference to the mounted node
    node_ref: NodeRef,
    key: Option<Key>,
}

impl<ICOMP: IntoComponent> Clone for VChild<ICOMP> {
    fn clone(&self) -> Self {
        VChild {
            props: Rc::clone(&self.props),
            node_ref: self.node_ref.clone(),
            key: self.key.clone(),
        }
    }
}

impl<ICOMP: IntoComponent> PartialEq for VChild<ICOMP>
where
    ICOMP::Properties: PartialEq,
{
    fn eq(&self, other: &VChild<ICOMP>) -> bool {
        self.props == other.props
    }
}

impl<ICOMP> VChild<ICOMP>
where
    ICOMP: IntoComponent,
{
    /// Creates a child component that can be accessed and modified by its parent.
    pub fn new(props: ICOMP::Properties, node_ref: NodeRef, key: Option<Key>) -> Self {
        Self {
            props: Rc::new(props),
            node_ref,
            key,
        }
    }
}

impl<ICOMP> From<VChild<ICOMP>> for VComp
where
    ICOMP: IntoComponent,
{
    fn from(vchild: VChild<ICOMP>) -> Self {
        VComp::new::<ICOMP>(vchild.props, vchild.node_ref, vchild.key)
    }
}

impl VComp {
    /// Creates a new `VComp` instance.
    pub fn new<ICOMP>(props: Rc<ICOMP::Properties>, node_ref: NodeRef, key: Option<Key>) -> Self
    where
        ICOMP: IntoComponent,
    {
        VComp {
            type_id: TypeId::of::<ICOMP::Component>(),
            node_ref,
            mountable: Box::new(PropsWrapper::<ICOMP::Component>::new(props)),
            key,
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
        pub(crate) async fn render_to_string(
            &self,
            w: &mut String,
            parent_scope: &AnyScope,
            hydratable: bool,
        ) {
            self.mountable
                .as_ref()
                .render_to_string(w, parent_scope, hydratable)
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

        let mut renderer = ServerRenderer::<Comp>::new();
        renderer.set_hydratable(false);

        let s = renderer.render().await;

        assert_eq!(
            s,
            "<div><div>Hello, Jane!</div><div>Hello, John!</div><div>Hello, Josh!</div></div>"
        );
    }
}
