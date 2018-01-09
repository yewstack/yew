//! This module contains the implementation of a virtual component `VComp`.

use std::marker::PhantomData;
use stdweb::web::Element;
use html::{Scope, SharedContext, Component};

/// A virtual component.
pub struct VComp<CTX, MSG> {
    mounter: Box<Fn(Element, SharedContext<CTX>)>,
    _msg: PhantomData<MSG>,
}

impl<CTX: 'static, MSG: 'static> VComp<CTX, MSG> {
    /// This method prepares a generator to make a new instance of the `Component`.
    pub fn lazy<T: Component<CTX> + 'static>() -> Self {
        let generator = |element, context| {
            let app: Scope<CTX, T> = Scope::reuse(context);
            app.mount(element);
        };
        VComp {
            mounter: Box::new(generator),
            _msg: PhantomData,
        }
    }
}

impl<CTX, MSG> VComp<CTX, MSG> {
    /// This methods mount a virtual component with a generator created with `lazy` call.
    pub fn mount(&self, element: &Element, context: SharedContext<CTX>) {
        (self.mounter)(element.clone(), context);
    }
}

