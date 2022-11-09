use std::any::{Any, TypeId};
#[cfg(any(feature = "csr", feature = "ssr"))]
use std::rc::Rc;

#[cfg(feature = "ssr")]
use futures::future::{FutureExt, LocalBoxFuture};
#[cfg(feature = "csr")]
use web_sys::Element;

#[cfg(any(feature = "csr", feature = "ssr"))]
use super::Scope;
use super::{BaseComponent, Context};
#[cfg(feature = "csr")]
use crate::dom_bundle::BSubtree;
#[cfg(feature = "hydration")]
use crate::dom_bundle::Fragment;
use crate::functional::FunctionComponent;
#[cfg(feature = "csr")]
use crate::html::NodeRef;
#[cfg(feature = "ssr")]
use crate::platform::fmt::BufWriter;
#[cfg(any(feature = "hydration", feature = "ssr"))]
use crate::virtual_dom::Collectable;

pub(crate) trait Intrinsical {
    fn type_id(&self) -> TypeId;
    fn any_props(&self) -> &dyn Any;

    fn create_component(&self, ctx: &Context) -> FunctionComponent;
    #[cfg(any(feature = "hydration", feature = "ssr"))]
    fn create_collectable(&self) -> Collectable;

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

pub(crate) struct ComponentIntrinsic<COMP: BaseComponent> {
    props: COMP::Properties,
}

impl<COMP: BaseComponent> ComponentIntrinsic<COMP> {
    pub fn new(props: COMP::Properties) -> Self {
        Self { props }
    }

    pub fn props(&self) -> &COMP::Properties {
        &self.props
    }
}

impl<COMP: BaseComponent> Intrinsical for ComponentIntrinsic<COMP> {
    fn type_id(&self) -> TypeId {
        TypeId::of::<COMP>()
    }

    fn any_props(&self) -> &dyn Any {
        &self.props
    }

    fn create_component(&self, ctx: &Context) -> FunctionComponent {
        COMP::create(ctx)
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
