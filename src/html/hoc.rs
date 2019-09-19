use crate::html::{Component, ComponentLink, Html, Renderable, ShouldRender};
use crate::virtual_dom::vcomp::ScopeHolder;
use crate::virtual_dom::{VComp, VNode};
use crate::Properties as PropertiesTrait;
use serde::export::PhantomData;

// TODO it might make sense to decompose HocData into smaller, interchangeable parts so that props/state behavior can be changed independently of update behavior.
// Not sure how valuable that would be though?

/// Abstracts over the data layer of a Higher Order Component without a conception of the
/// target component it will render.
///
/// Specifying the data for the HOC, requires reimplementing methods that are part of the component lifecycle.
///
/// # Example
/// ```
///# use yew::{Component, ComponentLink, ShouldRender, Renderable, Html, Properties};
///# use yew::html;
///# use yew::html::{HocData, Hoc};
/// pub struct MyComponent {
///     props: Props
/// }
/// #[derive(Debug, Properties, PartialEq, Clone)]
/// pub struct Props {
///     name: String
/// }
///
/// impl Component for MyComponent {
///     type Message = ();
///     type Properties = Props;
///
///     fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
///         MyComponent { props }
///     }
///     fn update(&mut self, msg: Self::Message) -> ShouldRender {
///         true
///     }
///     fn change(&mut self, props: Self::Properties) -> ShouldRender {
///         self.props = props;
///         true
///     }
/// }
/// impl Renderable<MyComponent> for MyComponent {
///     fn view(&self) -> Html<MyComponent> {
///         let s = format!("Hi! My name is {}", self.props.name);
///         html!{ s }
///     }
/// }
///
/// /// An incomplete (for terseness) log annotator for components.
/// pub struct WithLoggingHoc {
///     props: Props
/// }
///
/// impl <T> HocData<Hoc<Props, (), Self, T>, Props, ()> for WithLoggingHoc
/// where
///     T: Component<Properties=Props, Message=()> + Renderable<T>,
/// {
///     fn create(props: Props, link: ComponentLink<Hoc<Props, (), WithLoggingHoc, T>>) -> Self {
///         log::trace!("create: {:?}", props);
///         WithLoggingHoc {
///             props
///         }
///     }
///
///     fn update(&self,msg: ()) -> bool {
///         log::trace!("updating: {:?}", msg);
///         false
///     }
///
///     fn child_props(&self) -> Props {
///         self.props.clone()
///     }
///
///     fn change(&mut self, props: <Hoc<Props, (), WithLoggingHoc, T> as Component>::Properties) -> bool {
///         log::trace!("changing: {:?}", props);
///         self.props = props;
///         true
///     }
/// }
///
/// /// Use this alias in the `html!` macro.
/// type MyComponentWithLogging = Hoc<Props, (), WithLoggingHoc, MyComponent>;
/// ```
pub trait HocData<Parent, ChildProps, Message>
where
    Parent: Component + Renderable<Parent>,
    <Parent as Component>::Properties: PartialEq,
    ChildProps: PropertiesTrait,
{
    /// Creates the data for the HOC.
    fn create(props: Parent::Properties, link: ComponentLink<Parent>) -> Self;
    /// Runs when the HOC is mounted.
    fn mounted(&mut self) -> ShouldRender {
        false
    }
    /// Runs when the HOC updates.
    fn update(&self, msg: Message) -> ShouldRender;
    /// Runs to extract props used to create target components.
    fn child_props(&self) -> ChildProps;
    /// Runs when the HOC changes.
    fn change(&mut self, props: Parent::Properties) -> ShouldRender;
    /// Runs when the HOC is destroyed.
    fn destroy(&mut self) {}
}

/// Higher Order Component.
///
/// Data handling logic can be specified in a `HocData` implementation,
/// while rendering can be shared between a variety of target components.
pub struct Hoc<Properties, Message, Data, Target>
where
    Properties: PropertiesTrait + PartialEq + 'static,
    Message: From<Target::Message> + 'static,
    Data: HocData<Self, Target::Properties, Message> + 'static,
    Target: Component + Renderable<Target>,
{
    data: Data,
    props: PhantomData<Properties>,
    message: PhantomData<Message>,
    target: PhantomData<Target>,
}

impl<Properties, Message, Data, Target> Component for Hoc<Properties, Message, Data, Target>
where
    Properties: PropertiesTrait + PartialEq + 'static,
    Message: From<Target::Message> + 'static,
    Data: HocData<Self, Target::Properties, Message> + 'static,
    Target: Component + Renderable<Target>,
{
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let data = Data::create(props, link);
        Hoc {
            data,
            props: PhantomData,
            message: PhantomData,
            target: PhantomData,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        <Data as HocData<Self, _, _>>::mounted(&mut self.data)
    }

    fn update(&mut self, msg: Message) -> ShouldRender {
        <Data as HocData<Self, _, Message>>::update(&mut self.data, msg)
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        <Data as HocData<Self, Target::Properties, _>>::change(&mut self.data, props)
    }

    fn destroy(&mut self) {
        <Data as HocData<Self, _, _>>::destroy(&mut self.data)
    }
}

impl<Properties, Message, Data, Target> Renderable<Hoc<Properties, Message, Data, Target>>
    for Hoc<Properties, Message, Data, Target>
where
    Properties: PropertiesTrait + PartialEq + 'static,
    Message: From<Target::Message> + 'static,
    Data: HocData<Self, Target::Properties, Message> + 'static,
    Target: Component + Renderable<Target>,
{
    fn view(&self) -> Html<Self> {
        let vcomp_scope: ScopeHolder<_> = Default::default();
        let child_props: Target::Properties = self.data.child_props();

        VNode::VComp(VComp::new::<Target>(child_props, vcomp_scope))
    }
}
