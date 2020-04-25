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
/// It is reasonable practice to use `Pure` as a prefix or `Impl` as a suffix to your pure component model
/// and use an alias to provide a terser name to be used by other components:
///
/// ```
/// use yew::Properties;
/// use yew::Html;
/// use yewtil::{PureComponent, Pure};
///
/// #[derive(Properties, PartialEq)]
/// pub struct PureMyComponent {
///     pub data: String
/// }
///
/// impl PureComponent for PureMyComponent {
/// fn render(&self) -> Html {
///#        unimplemented!()
///        // ...
///     }
/// }
///
/// /// Use this from within `html!` macros.
/// pub type MyComponent = Pure<PureMyComponent>;
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
