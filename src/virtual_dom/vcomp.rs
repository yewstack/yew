//! This module contains the implementation of a virtual component `VComp`.

use stdweb::web::Element;
use html::{ScopeBuilder, SharedContext, Component};

/// A virtual component.
pub struct VComp<CTX, COMP: Component<CTX>> {
    builder: Option<ScopeBuilder<CTX, COMP>>,
}

impl<CTX: 'static, COMP: Component<CTX>> VComp<CTX, COMP> {
    /// This method prepares a generator to make a new instance of the `Component`.
    pub fn lazy() -> Self {
        let builder: ScopeBuilder<CTX, COMP> = ScopeBuilder::new();
        let prop_sender = builder.sender();
        VComp {
            builder: Some(builder),
        }
    }
}

impl<CTX: 'static, COMP: Component<CTX>> VComp<CTX, COMP> {
    /// This methods mount a virtual component with a generator created with `lazy` call.
    pub fn mount(&mut self, element: &Element, context: SharedContext<CTX>) {
        let builder = self.builder.take().expect("tried to mount component twice");
        builder.build(context).mount(element.clone());
    }
}

