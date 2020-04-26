use crate::dsl::BoxedVNodeProducer;
use yew::virtual_dom::VText;
use yew::Component;

pub struct VTextProducer<COMP: Component>(Box<dyn FnOnce() -> VText<COMP>>);

impl<COMP: Component> VTextProducer<COMP> {
    pub fn new<T: Into<String> + 'static>(text: T) -> Self {
        VTextProducer(Box::new(move || VText::new(text.into())))
    }
}

impl<COMP: Component> From<VTextProducer<COMP>> for BoxedVNodeProducer<COMP> {
    fn from(vtext_prod: VTextProducer<COMP>) -> Self {
        BoxedVNodeProducer::wrap(move |_scope| (vtext_prod.0)().into())
    }
}
