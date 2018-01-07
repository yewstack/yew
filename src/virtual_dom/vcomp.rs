use std::marker::PhantomData;
use stdweb::web::Element;
use html::App;
use component::Component;

pub struct VComp<MSG> {
    mounter: Box<Fn(Element)>,
    _msg: PhantomData<MSG>,
}

impl<MSG: 'static> VComp<MSG> {
    pub fn lazy<T: Component + 'static>() -> Self {
        let generator = |element| {
            let component = T::default();
            let mut app = App::new();
            app.mount_to(element, (), component);
        };
        VComp {
            mounter: Box::new(generator),
            _msg: PhantomData,
        }
    }
}

impl<MSG> VComp<MSG> {
    pub fn mount(&self, element: &Element) {
        (self.mounter)(element.clone());
    }
}

