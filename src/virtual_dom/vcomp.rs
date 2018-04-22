//! This module contains the implementation of a virtual component `VComp`.

use super::{Reform, VDiff, VNode};
use html::{self, Component, ComponentUpdate, NodeCell, Renderable, ScopeBuilder, Env,
           SharedContext};
use callback::Callback;
use std::any::TypeId;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use stdweb::unstable::TryInto;
use stdweb::web::{document, Element, INode, Node};

struct Hidden;

type AnyProps = (TypeId, *mut Hidden);

type Generator<CTX> = FnMut(SharedContext<CTX>, Element, Option<Node>, AnyProps);

type Rider<CTX, COMP> = html::Activator<CTX, COMP>;

type Activator<CTX, COMP> = Rc<RefCell<Option<Rider<CTX, COMP>>>>;

/// A virtual component.
pub struct VComp<CTX, COMP: Component<CTX>> {
    type_id: TypeId,
    cell: NodeCell,
    props: Option<(TypeId, *mut Hidden)>,
    blind_sender: Box<FnMut(AnyProps)>,
    generator: Box<Generator<CTX>>,
    activators: Vec<Activator<CTX, COMP>>,
    //destroyer: Option<ScopeDestroyer>,
    _parent: PhantomData<COMP>,
}

impl<CTX: 'static, COMP: Component<CTX>> VComp<CTX, COMP> {
    /// This method prepares a generator to make a new instance of the `Component`.
    pub fn lazy<CHILD>() -> (CHILD::Properties, Self)
    where
        CHILD: Component<CTX> + Renderable<CTX, CHILD>,
    {
        let cell: NodeCell = Rc::new(RefCell::new(None));
        let mut builder: ScopeBuilder<CTX, CHILD> = ScopeBuilder::new();
        //let destroyer = Some(builder.destroyer());
        let mut activator = builder.activator();
        let mut builder = Some(builder);
        let occupied = cell.clone();
        // This function creates and mounts a new component instance
        let generator =
            move |context, element, obsolete: Option<Node>, (type_id, raw): AnyProps| {
                if type_id != TypeId::of::<CHILD>() {
                    panic!("tried to unpack properties of the other component");
                }
                let props = unsafe {
                    let raw: *mut CHILD::Properties = ::std::mem::transmute(raw);
                    *Box::from_raw(raw)
                };

                let builder = builder.take().expect("tried to mount component twice");
                let opposite = obsolete.map(VNode::VRef);
                let (_env, scope) = builder.build(context);
                scope.mount_in_place(
                    element,
                    opposite,
                    Some(occupied.clone()),
                    Some(props),
                );
            };
        let mut previous_props = None;
        let blind_sender = move |(type_id, raw): AnyProps| {
            if type_id != TypeId::of::<CHILD>() {
                panic!("tried to send properties of the other component");
            }
            let props = unsafe {
                let raw: *mut CHILD::Properties = ::std::mem::transmute(raw);
                *Box::from_raw(raw)
            };
            let new_props = Some(props);
            // Ignore update till properties changed
            if previous_props != new_props {
                let props = new_props.as_ref().unwrap().clone();
                activator.send(ComponentUpdate::Properties(props));
                previous_props = new_props;
            }
        };
        let properties = Default::default();
        let comp = VComp {
            type_id: TypeId::of::<CHILD>(),
            cell,
            props: None,
            blind_sender: Box::new(blind_sender),
            generator: Box::new(generator),
            activators: Vec::new(),
            //destroyer,
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

    /// This method attach sender to a listeners, because created properties
    /// know nothing about a parent.
    fn activate_props(&mut self, sender: &Rider<CTX, COMP>) -> AnyProps {
        for activator in &self.activators {
            *activator.borrow_mut() = Some(sender.clone());
        }
        self.props
            .take()
            .expect("tried to activate properties twice")
    }

    /// This methods gives sender from older node.
    pub(crate) fn grab_sender_of(&mut self, other: Self) {
        assert_eq!(self.type_id, other.type_id);
        // Grab a sender and a cell (element's reference) to reuse it later
        self.blind_sender = other.blind_sender;
        self.cell = other.cell;
        //self.destroyer = other.destroyer;
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
    /// This methods mount a virtual component with a generator created with `lazy` call.
    fn mount<T: INode>(
        &mut self,
        context: SharedContext<CTX>,
        parent: &T,
        opposite: Option<Node>,
        props: AnyProps,
    ) {
        let element: Element = parent
            .as_node()
            .as_ref()
            .to_owned()
            .try_into()
            .expect("element expected to mount VComp");
        (self.generator)(context, element, opposite, props);
    }

    fn send_props(&mut self, props: AnyProps) {
        (self.blind_sender)(props);
    }
}

impl<CTX, COMP> VDiff for VComp<CTX, COMP>
where
    CTX: 'static,
    COMP: Component<CTX> + 'static,
{
    type Context = CTX;
    type Component = COMP;

    /// Remove VComp from parent.
    fn detach(&mut self, parent: &Node) -> Option<Node> {
        // Destroy the loop. It's impossible to use `Drop`,
        // because parts can be reused with `grab_sender_of`.
        /* TODO Replace with the activator!
        if let Some(destroyer) = self.destroyer {
            destroyer.destroy();
        }
        */
        // Keep the sibling in the cell and send a message `Drop` to a loop
        self.cell.borrow_mut().take().and_then(|node| {
            let sibling = node.next_sibling();
            parent
                .remove_child(&node)
                .expect("can't remove the component");
            sibling
        })
    }

    /// Renders independent component over DOM `Element`.
    /// It also compares this with an opposite `VComp` and inherits sender of it.
    fn apply(
        &mut self,
        parent: &Node,
        _: Option<&Node>,
        opposite: Option<VNode<Self::Context, Self::Component>>,
        env: &Env<Self::Context, Self::Component>,
    ) -> Option<Node> {
        let reform = {
            match opposite {
                Some(VNode::VComp(mut vcomp)) => {
                    if self.type_id == vcomp.type_id {
                        self.grab_sender_of(vcomp);
                        Reform::Keep
                    } else {
                        let node = vcomp.detach(parent);
                        Reform::Before(node)
                    }
                }
                Some(mut vnode) => {
                    let node = vnode.detach(parent);
                    Reform::Before(node)
                }
                None => Reform::Before(None),
            }
        };
        let any_props = self.activate_props(&env.activator());
        match reform {
            Reform::Keep => {
                // Send properties update when component still be rendered.
                // But for the first initialization mount gets initial
                // properties directly without this channel.
                self.send_props(any_props);
            }
            Reform::Before(node) => {
                // This is a workaround, because component should be mounted
                // over opposite element if it exists.
                // There is created an empty text node to be replaced with mount call.
                let node = node.map(|sibling| {
                    let element = document().create_text_node("");
                    parent
                        .insert_before(&element, &sibling)
                        .expect("can't insert dummy element for a component");
                    element.as_node().to_owned()
                });
                self.mount(env.context_rc(), parent, node, any_props);
            }
        }
        self.cell.borrow().as_ref().map(|node| node.to_owned())
    }
}

impl<CTX, COMP: Component<CTX>> PartialEq for VComp<CTX, COMP> {
    fn eq(&self, other: &VComp<CTX, COMP>) -> bool {
        self.type_id == other.type_id
    }
}
