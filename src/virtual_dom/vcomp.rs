//! This module contains the implementation of a virtual component `VComp`.

use std::marker::PhantomData;
use stdweb::web::Element;
use html::{ScopeBuilder, SharedContext, Component, ComponentSender, ComponentUpdate};

/// Return sender and empty properties.
pub struct PropConnector<CTX, COMP: Component<CTX>> {
    sender: ComponentSender<CTX, COMP>,
    /// Properties of a `Component` to set.
    properties: COMP::Properties,
}

impl<CTX: 'static, COMP: Component<CTX>> PropConnector<CTX, COMP> {
    /// Applies properties to a component.
    pub fn apply(self) {
        self.sender.send(ComponentUpdate::Properties(self.properties))
            .expect("can't send new properties to a component");
    }

    /// Get properties for updates.
    pub fn prop_mut(&mut self) -> &mut COMP::Properties {
        &mut self.properties
    }
}

/// A virtual component.
pub struct VComp<CTX, COMP: Component<CTX>> {
    generator: Box<FnMut(SharedContext<CTX>, Element)>,
    _parent: PhantomData<COMP>,
}

impl<CTX: 'static, COMP: Component<CTX>> VComp<CTX, COMP> {
    /// This method prepares a generator to make a new instance of the `Component`.
    pub fn lazy<CHILD: Component<CTX>>() -> (PropConnector<CTX, CHILD>, Self) {
        let builder: ScopeBuilder<CTX, CHILD> = ScopeBuilder::new();
        let sender = builder.sender();
        let mut builder = Some(builder);
        let generator = move |context, element| {
            let builder = builder.take().expect("tried to mount component twice");
            builder.build(context).mount(element);
        };
        let connector = PropConnector {
            sender: sender.clone(),
            properties: Default::default(),
        };
        let comp = VComp {
            generator: Box::new(generator),
            _parent: PhantomData,
        };
        (connector, comp)
    }
}

impl<CTX: 'static, COMP: Component<CTX>> VComp<CTX, COMP> {
    /// This methods mount a virtual component with a generator created with `lazy` call.
    pub fn mount(&mut self, element: &Element, context: SharedContext<CTX>) {
        (self.generator)(context, element.clone());
    }
}

