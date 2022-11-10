use std::any::{Any, TypeId};
#[cfg(any(feature = "csr", feature = "ssr"))]
use std::rc::Rc;

#[cfg(feature = "ssr")]
use futures::future::{FutureExt, LocalBoxFuture};
#[cfg(feature = "csr")]
use web_sys::Element;

use super::Component;
#[cfg(any(feature = "csr", feature = "ssr"))]
use super::Scope;
#[cfg(feature = "csr")]
use crate::dom_bundle::BSubtree;
#[cfg(feature = "hydration")]
use crate::dom_bundle::Fragment;
use crate::functional::{HookContext, Renderable};
#[cfg(feature = "csr")]
use crate::html::NodeRef;
#[cfg(feature = "ssr")]
use crate::platform::fmt::BufWriter;
#[cfg(any(feature = "hydration", feature = "ssr"))]
use crate::virtual_dom::Collectable;
use crate::HtmlResult;

pub(crate) trait Intrinsical {
    fn as_any(&self) -> &dyn Any;
    fn type_id(&self) -> TypeId;
    #[cfg(any(feature = "hydration", feature = "ssr"))]
    fn create_collectable(&self) -> Collectable;

    fn intrinsic_eq(&self, other: &dyn Intrinsical) -> bool;
    fn render(&self, ctx: &mut HookContext) -> HtmlResult;

    #[cfg(feature = "csr")]
    fn mount(
        self: Rc<Self>,
        root: &BSubtree,
        parent_scope: &Scope,
        parent: Element,
        internal_ref: NodeRef,
        next_sibling: NodeRef,
    ) -> Scope;

    #[cfg(feature = "csr")]
    fn reuse(self: Rc<Self>, scope: &Scope, next_sibling: NodeRef);

    #[cfg(feature = "ssr")]
    fn render_into_stream<'a>(
        self: Rc<Self>,
        w: &'a mut BufWriter,
        parent_scope: &'a Scope,
        hydratable: bool,
    ) -> LocalBoxFuture<'a, ()>;

    #[cfg(feature = "hydration")]
    fn hydrate(
        self: Rc<Self>,
        root: BSubtree,
        parent_scope: &Scope,
        parent: Element,
        internal_ref: NodeRef,
        fragment: &mut Fragment,
    ) -> Scope;
}

pub(crate) struct ComponentIntrinsic<COMP: Component> {
    props: COMP::Properties,
}

impl<COMP: Component> ComponentIntrinsic<COMP> {
    pub fn new(props: COMP::Properties) -> Self {
        Self { props }
    }

    pub fn props(&self) -> &COMP::Properties {
        &self.props
    }
}

impl<COMP: Component> Intrinsical for ComponentIntrinsic<COMP> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<COMP>()
    }

    fn intrinsic_eq(&self, other: &dyn Intrinsical) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .map(|m| self.props() == m.props())
            .unwrap_or(false)
    }

    fn render(&self, ctx: &mut HookContext) -> HtmlResult {
        COMP::render(ctx, self.props())
    }

    #[cfg(any(feature = "hydration", feature = "ssr"))]
    fn create_collectable(&self) -> Collectable {
        Collectable::for_component::<COMP>()
    }

    #[cfg(feature = "csr")]
    fn mount(
        self: Rc<Self>,
        root: &BSubtree,
        parent_scope: &Scope,
        parent: Element,
        internal_ref: NodeRef,
        next_sibling: NodeRef,
    ) -> Scope {
        let scope = Scope::new(self.as_ref(), Some(parent_scope.clone()));
        scope.mount(self, root.clone(), parent, next_sibling, internal_ref);

        scope
    }

    #[cfg(feature = "csr")]
    fn reuse(self: Rc<Self>, scope: &Scope, next_sibling: NodeRef) {
        scope.reuse(self, next_sibling);
    }

    #[cfg(feature = "ssr")]
    fn render_into_stream<'a>(
        self: Rc<Self>,
        w: &'a mut BufWriter,
        parent_scope: &'a Scope,
        hydratable: bool,
    ) -> LocalBoxFuture<'a, ()> {
        let scope: Scope = Scope::new(self.as_ref(), Some(parent_scope.clone()));

        async move {
            scope.render_into_stream(self, w, hydratable).await;
        }
        .boxed_local()
    }

    #[cfg(feature = "hydration")]
    fn hydrate(
        self: Rc<Self>,
        root: BSubtree,
        parent_scope: &Scope,
        parent: Element,
        internal_ref: NodeRef,
        fragment: &mut Fragment,
    ) -> Scope {
        let scope: Scope = Scope::new(self.as_ref(), Some(parent_scope.clone()));

        // This is very helpful to see which component is failing during hydration
        // which means this component may not having a stable layout / differs between
        // client-side and server-side.
        tracing::trace!(
            component.id = scope.id(),
            "hydration(type = {})",
            std::any::type_name::<COMP>()
        );

        scope.hydrate(self, root, parent, fragment, internal_ref);

        scope
    }
}
