//! Shortcut for terse component definitions.
use crate::NeqAssign;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender};

/// Allows immutable components to be declared using a single struct and a single method.
pub trait PureComponent: Properties + PartialEq + Sized + 'static {
    /// Renders self to `Html`.
    fn render(&self) -> Html;
}

/// Wrapper component for pure components.
///
/// Due to constraints in Rust's coherence rules, `Component` can't be implemented for any `T` that implements
/// `PureComponent`, so instead this struct wraps a `T: PureComponent` and `Component` is implemented
/// for this instead.
///
/// # Example
///
/// ```
/// # use yew::{html, Callback, Html, MouseEvent, Properties};
/// use yewtil::{Pure, PureComponent};
///
/// /// Alias to improve usability.
/// pub type Button = Pure<PureButton>;
///
/// #[derive(Clone, PartialEq, Properties)]
/// pub struct PureButton {
///     pub callback: Callback<MouseEvent>,
///     #[prop_or_default]
///     pub text: String,
/// }
/// impl PureComponent for PureButton {
///     fn render(&self) -> Html {
///         html! {
///             <button onclick=&self.callback>{ &self.text }</button>
///         }
///     }
/// }
///
/// # fn view() -> Html {
/// // Pure components can be used like normal components
/// html! { <Button callback=Callback::from(|_| println!("clicked")) text="Click me!" /> }
/// # }
/// ```
#[derive(Debug)]
pub struct Pure<T>(T);

impl<T: PureComponent + 'static> Component for Pure<T> {
    type Message = ();
    type Properties = T;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Pure(props)
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.0.neq_assign(props)
    }

    fn view(&self) -> Html {
        self.0.render()
    }
}
