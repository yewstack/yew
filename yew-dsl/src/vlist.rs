use crate::BoxedVNodeProducer;
use yew::virtual_dom::VList;
use yew::Component;

///
pub struct VListProducer<COMP: Component> {
    children: Vec<BoxedVNodeProducer<COMP>>,
}

impl<COMP: Component> Default for VListProducer<COMP> {
    fn default() -> Self {
        VListProducer::<COMP> { children: vec![] }
    }
}

impl<COMP: Component> VListProducer<COMP> {
    pub fn new() -> Self {
        VListProducer::<COMP> { children: vec![] }
    }

    pub fn child<T: Into<BoxedVNodeProducer<COMP>>>(mut self, child: T) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn populated_new(children: Vec<BoxedVNodeProducer<COMP>>) -> Self {
        VListProducer::<COMP> { children }
    }
}

impl<COMP: Component> From<VListProducer<COMP>> for BoxedVNodeProducer<COMP> {
    fn from(vlist_prod: VListProducer<COMP>) -> Self {
        BoxedVNodeProducer(Box::new(move |scope| {
            let mut vlist = VList::new();
            for child in vlist_prod.children {
                let child = child.execute(&scope);
                vlist.add_child(child);
            }
            vlist.into()
        }))
    }
}
