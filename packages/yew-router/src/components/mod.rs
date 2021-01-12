//! Components that integrate with the [route agent](agent/struct.RouteAgent.html).
//!
//! At least one bridge to the agent needs to exist for these to work.
//! This can be done transitively by using a `Router` component, which owns a bridge to the agent.

mod router_button;
mod router_link;

use yew::{Children, Properties};

#[allow(deprecated)]
pub use self::{router_button::RouterButton, router_link::RouterAnchor, router_link::RouterLink};
use crate::Switch;

// TODO This should also be PartialEq and Clone. Its blocked on Children not supporting that.
// TODO This should no longer take link & String, and instead take a route: SW implementing Switch
/// Properties for `RouterButton` and `RouterLink`.
#[derive(Properties, Clone, Default, Debug)]
pub struct Props<SW>
where
    SW: Switch + Clone,
{
    /// The Switched item representing the route.
    pub route: SW,
    #[deprecated(note = "Use children field instead (nested html)")]
    /// The text to display.
    #[prop_or_default]
    pub text: String,
    /// Html inside the component.
    #[prop_or_default]
    pub children: Children,
    /// Disable the component.
    #[prop_or_default]
    pub disabled: bool,
    /// Classes to be added to component.
    #[prop_or_default]
    pub classes: String,
}

/// Message for `RouterButton` and `RouterLink`.
#[derive(Clone, Copy, Debug)]
pub enum Msg {
    /// Tell the router to navigate the application to the Component's pre-defined route.
    Clicked,
}
