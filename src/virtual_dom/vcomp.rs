//! This module contains the implementation of a virtual component `VComp`.

use std::marker::PhantomData;
use stdweb::web::Element;
use html::{Scope, SharedContext, Component};

/// A virtual component.
pub struct VComp<CTX, COMP: Component<CTX>> {
    mounter: Box<Fn(Element, SharedContext<CTX>)>,
    _msg: PhantomData<COMP>,
}

impl<CTX: 'static, COMP: Component<CTX>> VComp<CTX, COMP> {
    /// This method prepares a generator to make a new instance of the `Component`.
    pub fn lazy() -> Self {
        let generator = |element, context| {
            let app: Scope<CTX, COMP> = Scope::reuse(context);
            app.mount(element);
        };
        VComp {
            mounter: Box::new(generator),
            _msg: PhantomData,
        }
    }
}

impl<CTX, COMP: Component<CTX>> VComp<CTX, COMP> {
    /// This methods mount a virtual component with a generator created with `lazy` call.
    pub fn mount(&self, element: &Element, context: SharedContext<CTX>) {
        (self.mounter)(element.clone(), context);
    }
}

