//! This module contains the implementation of a virtual component `VComp`.

use std::rc::Rc;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::any::TypeId;
use stdweb::web::{Node, Element};
use virtual_dom::VNode;
use html::{ScopeBuilder, SharedContext, Component, Renderable, ComponentUpdate, ScopeSender, Callback, ScopeEnv};

struct Hidden;

/// A virtual component.
pub struct VComp<CTX, COMP: Component<CTX>> {
    type_id: TypeId,
    /// A reference to the `Element`.
    pub reference: Option<Element>,
    props: Option<(TypeId, *mut Hidden)>,
    blind_sender: Box<FnMut((TypeId, *mut Hidden))>,
    generator: Box<FnMut(SharedContext<CTX>, Element, Option<Node>)>,
    activators: Vec<Rc<RefCell<Option<ScopeSender<CTX, COMP>>>>>,
    _parent: PhantomData<COMP>,
}

impl<CTX: 'static, COMP: Component<CTX>> VComp<CTX, COMP> {
    /// This method prepares a generator to make a new instance of the `Component`.
    pub fn lazy<CHILD>() -> (CHILD::Properties, Self)
    where
        CHILD: Component<CTX> + Renderable<CTX, CHILD>,
    {
        let builder: ScopeBuilder<CTX, CHILD> = ScopeBuilder::new();
        let mut sender = builder.sender();
        let mut builder = Some(builder);
        let generator = move |context, element, obsolete: Option<Node>| {
            let builder = builder.take().expect("tried to mount component twice");
            let opposite = obsolete.map(VNode::VRef);
            builder.build(context).mount_in_place(element, opposite);
        };
        let mut previous_props = None;
        let blind_sender = move |(type_id, raw): (TypeId, *mut Hidden)| {
            if type_id != TypeId::of::<CHILD>() {
                panic!("tried to send properties of other component");
            }
            let props = unsafe {
                let raw: *mut CHILD::Properties = ::std::mem::transmute(raw);
                *Box::from_raw(raw)
            };
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
            reference: None,
            props: None,
            blind_sender: Box::new(blind_sender),
            generator: Box::new(generator),
            activators: Vec::new(),
            _parent: PhantomData,
        };
        (properties, comp)
    }

    /// Attach properties associated with the component.
    pub fn set_props<T>(&mut self, props: T) {
        let boxed = Box::into_raw(Box::new(props));
        let data = unsafe { ::std::mem::transmute(boxed) };
        self.props = Some((self.type_id, data));
    }

    pub(crate) fn send_props(&mut self, sender: ScopeSender<CTX, COMP>) {
        for activator in self.activators.iter_mut() {
            *activator.borrow_mut() = Some(sender.clone());
        }
        let props = self.props.take()
            .expect("tried to send same properties twice");
        (self.blind_sender)(props);
    }

    /// This methods gives sender from older node.
    pub(crate) fn grab_sender_of(&mut self, other: Self) {
        assert_eq!(self.type_id, other.type_id);
        // Grab a sender to reuse it later
        self.blind_sender = other.blind_sender;
        self.reference = other.reference;
    }
}

/// Converts property and attach lazy components to it.
/// This type holds context and components types to store an activatior which
/// will be used later buring rendering state to attach component sender.
pub trait Transformer<CTX, COMP: Component<CTX>, FROM, TO> {
    /// Transforms one type to another.
    fn transform(&mut self, from: FROM) -> TO;
}

impl<CTX, COMP, T> Transformer<CTX, COMP, T, T> for VComp<CTX, COMP>
where
    COMP: Component<CTX>,
{
    fn transform(&mut self, from: T) -> T {
        from
    }
}

impl<'a, CTX, COMP, T> Transformer<CTX, COMP, &'a T, T> for VComp<CTX, COMP>
where
    COMP: Component<CTX>,
    T: Clone,
{
    fn transform(&mut self, from: &'a T) -> T {
        from.clone()
    }
}

impl<'a, CTX, COMP> Transformer<CTX, COMP, &'a str, String> for VComp<CTX, COMP>
where
    COMP: Component<CTX>,
{
    fn transform(&mut self, from: &'a str) -> String {
        from.to_owned()
    }
}

impl<'a, CTX, COMP, F, IN> Transformer<CTX, COMP, F, Option<Callback<IN>>> for VComp<CTX, COMP>
where
    CTX: 'static,
    COMP: Component<CTX>,
    F: Fn(IN) -> COMP::Msg + 'static,
{
    fn transform(&mut self, from: F) -> Option<Callback<IN>> {
        let cell = Rc::new(RefCell::new(None));
        self.activators.push(cell.clone());
        let callback = move |arg| {
            let msg = from(arg);
            if let Some(ref mut sender) = *cell.borrow_mut() {
                sender.send(ComponentUpdate::Message(msg));
            } else {
                panic!("unactivated callback, parent component have to activate it");
            }
        };
        Some(callback.into())
    }
}

impl<CTX, COMP> VComp<CTX, COMP>
where
    CTX: 'static,
    COMP: Component<CTX> + 'static,
{
    /// Remove VComp from parent.
    pub fn remove(self, _: &Element) {
        unimplemented!();
    }

    /// This methods mount a virtual component with a generator created with `lazy` call.
    fn mount(&mut self, context: SharedContext<CTX>, parent: &Element, opposite: Option<Node>) {
        (self.generator)(context, parent.clone(), opposite);
    }

    /// Renders independent component over DOM `Element`.
    /// It also compares this with an opposite `VComp` and inherits sender of it.
    pub fn apply(&mut self, parent: &Element, opposite: Option<VNode<CTX, COMP>>, env: ScopeEnv<CTX, COMP>) {
        match opposite {
            Some(VNode::VComp(vcomp)) => {
                if self.type_id == vcomp.type_id {
                    self.grab_sender_of(vcomp);
                    self.send_props(env.sender());
                } else {
                    let obsolete = vcomp.reference.map(Node::from);
                    self.send_props(env.sender());
                    self.mount(env.context(), parent, obsolete);
                }
            }
            Some(VNode::VTag(vtag)) => {
                let obsolete = vtag.reference.map(Node::from);
                self.send_props(env.sender());
                self.mount(env.context(), parent, obsolete);
            }
            Some(VNode::VText(vtext)) => {
                let obsolete = vtext.reference.map(Node::from);
                self.send_props(env.sender());
                self.mount(env.context(), parent, obsolete);
            }
            Some(VNode::VRef(node)) => {
                self.send_props(env.sender());
                self.mount(env.context(), parent, Some(node));
            }
            None => {
                self.send_props(env.sender());
                self.mount(env.context(), parent, None);
            }
        }
    }
}

impl<CTX, COMP: Component<CTX>> PartialEq for VComp<CTX, COMP> {
    fn eq(&self, other: &VComp<CTX, COMP>) -> bool {
        self.type_id == other.type_id
    }
}
