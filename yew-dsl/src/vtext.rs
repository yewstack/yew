use crate::BoxedVNodeProducer;
use yew::virtual_dom::VText;
use yew::Component;

/// A wrapper around a function which produces `VText` nodes.
pub struct VTextProducer(Box<dyn FnOnce() -> VText>);

impl VTextProducer {
    pub fn new<TEXT: Into<String> + 'static>(text: TEXT) -> Self {
        VTextProducer(Box::new(move || VText::new(text.into())))
    }
}

impl<COMP: Component> From<VTextProducer> for BoxedVNodeProducer<COMP> {
    fn from(vtext_prod: VTextProducer) -> Self {
        BoxedVNodeProducer::wrap(move |_scope| (vtext_prod.0)().into())
    }
}
