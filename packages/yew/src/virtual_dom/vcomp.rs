//! This module contains the implementation of a virtual component (`VComp`).

use std::any::TypeId;
use std::fmt;
use std::rc::Rc;

use super::Key;
use crate::html::{BaseComponent, ComponentIntrinsic, Intrinsical};

/// A virtual component.
pub struct VComp {
    pub(crate) type_id: TypeId,
    pub(crate) mountable: Rc<dyn Intrinsical>,
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
            mountable: self.mountable.clone(),
            key: self.key.clone(),
            _marker: 0,
        }
    }
}

/// A virtual child component.
pub struct VChild<COMP: BaseComponent> {
    /// The component properties
    intrinsic: Rc<ComponentIntrinsic<COMP>>,
    /// Reference to the mounted node
    key: Option<Key>,
}

impl<COMP: BaseComponent> Clone for VChild<COMP> {
    fn clone(&self) -> Self {
        VChild {
            intrinsic: Rc::clone(&self.intrinsic),
            key: self.key.clone(),
        }
    }
}

impl<COMP: BaseComponent> PartialEq for VChild<COMP>
where
    COMP::Properties: PartialEq,
{
    fn eq(&self, other: &VChild<COMP>) -> bool {
        self.intrinsic.props() == other.intrinsic.props()
    }
}

impl<COMP> VChild<COMP>
where
    COMP: BaseComponent,
{
    /// Creates a child component that can be accessed and modified by its parent.
    pub fn new(props: COMP::Properties, key: Option<Key>) -> Self {
        Self {
            intrinsic: Rc::new(ComponentIntrinsic::new(props)),
            key,
        }
    }
}

impl<COMP> From<VChild<COMP>> for VComp
where
    COMP: BaseComponent,
{
    fn from(vchild: VChild<COMP>) -> Self {
        VComp::with_intrinsic::<COMP, _>(vchild.intrinsic, vchild.key)
    }
}

impl VComp {
    /// Creates a new `VComp` instance.
    pub fn new<COMP>(props: COMP::Properties, key: Option<Key>) -> Self
    where
        COMP: BaseComponent,
    {
        VComp {
            type_id: TypeId::of::<COMP>(),
            mountable: Rc::new(ComponentIntrinsic::<COMP>::new(props)),
            key,
            _marker: 0,
        }
    }

    /// Creates a new `VComp` instance.
    pub(crate) fn with_intrinsic<COMP, I>(intrinsic: I, key: Option<Key>) -> Self
    where
        COMP: BaseComponent,
        I: Into<Rc<ComponentIntrinsic<COMP>>>,
    {
        VComp {
            type_id: TypeId::of::<COMP>(),
            mountable: intrinsic.into(),
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
    use crate::html::Scope;
    use crate::platform::fmt::BufWriter;

    impl VComp {
        #[inline]
        pub(crate) async fn render_into_stream(
            &self,
            w: &mut BufWriter,
            parent_scope: &Scope,
            hydratable: bool,
        ) {
            self.mountable
                .clone()
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
