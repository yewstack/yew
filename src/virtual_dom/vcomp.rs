//! This module contains the implementation of a virtual component `VComp`.

use std::marker::PhantomData;
use std::any::TypeId;
use serde::Serialize;
use bincode::{serialize, deserialize, Infinite};
use stdweb::web::Element;
use html::{ScopeBuilder, SharedContext, Component, ComponentUpdate};

/// A virtual component.
pub struct VComp<CTX, COMP: Component<CTX>> {
    type_id: TypeId,
    props: Option<Vec<u8>>,
    blind_sender: Box<FnMut(Vec<u8>)>,
    generator: Box<FnMut(SharedContext<CTX>, Element)>,
    _parent: PhantomData<COMP>,
}

impl<CTX: 'static, COMP: Component<CTX>> VComp<CTX, COMP> {
    /// This method prepares a generator to make a new instance of the `Component`.
    pub fn lazy<CHILD: Component<CTX>>() -> (CHILD::Properties, Self) {
        let builder: ScopeBuilder<CTX, CHILD> = ScopeBuilder::new();
        let mut sender = builder.sender();
        let mut builder = Some(builder);
        let generator = move |context, element| {
            let builder = builder.take().expect("tried to mount component twice");
            builder.build(context).mount(element);
        };
        let mut previous_props = None;
        let blind_sender = move |raw: Vec<u8>| {
            let props: CHILD::Properties = deserialize(raw.as_ref())
                .expect("can't deserialize properties");
            let new_props = Some(props);
            // Ignore update till properties changed
            if previous_props != new_props {
                let props = new_props.as_ref().unwrap().clone();
                sender.send(ComponentUpdate::Properties(props));
                previous_props = new_props;
            }
        };
        let properties = Default::default();
        let comp = VComp {
            type_id: TypeId::of::<CHILD>(),
            props: None,
            blind_sender: Box::new(blind_sender),
            generator: Box::new(generator),
            _parent: PhantomData,
        };
        (properties, comp)
    }

    /// Attach properties associated with the component.
    pub fn set_props<T: Serialize>(&mut self, props: &T) {
        let data = serialize(props, Infinite)
            .expect("can't serialize properties");
        self.props = Some(data);
    }

    pub(crate) fn send_props(&mut self) {
        let props = self.props.take()
            .expect("tried to send same properties twice");
        (self.blind_sender)(props);
    }

    /// This methods gives sender from older node.
    pub(crate) fn grab_sender_of(&mut self, other: Self) {
        assert_eq!(self.type_id, other.type_id);
        // Grab a sender to reuse it later
        self.blind_sender = other.blind_sender;
    }
}

impl<CTX: 'static, COMP: Component<CTX>> VComp<CTX, COMP> {
    /// This methods mount a virtual component with a generator created with `lazy` call.
    pub fn mount(&mut self, element: &Element, context: SharedContext<CTX>) {
        (self.generator)(context, element.clone());
    }
}

impl<CTX, COMP: Component<CTX>> PartialEq for VComp<CTX, COMP> {
    fn eq(&self, other: &VComp<CTX, COMP>) -> bool {
        self.type_id == other.type_id
    }
}
