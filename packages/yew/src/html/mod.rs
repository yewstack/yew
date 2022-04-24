//! The main html module which defines components, listeners, and class helpers.

mod classes;
mod component;
mod conversion;
mod dom_position;
mod error;
mod html_ref;
mod listener;

pub use classes::*;
pub use component::*;
pub use conversion::*;
pub(crate) use dom_position::*;
pub use error::*;
pub use html_ref::*;
pub use listener::*;
use web_sys::Element;

use crate::sealed::Sealed;
use crate::virtual_dom::{VNode, VPortal};

/// A type which expected as a result of `view` function implementation.
pub type Html = VNode;

/// An enhanced type of `Html` returned in suspendible function components.
pub type HtmlResult = RenderResult<Html>;

impl Sealed for HtmlResult {}
impl Sealed for Html {}

/// A trait to translate into a [`HtmlResult`].
pub trait IntoHtmlResult: Sealed {
    /// Performs the conversion.
    fn into_html_result(self) -> HtmlResult;
}

impl IntoHtmlResult for HtmlResult {
    #[inline(always)]
    fn into_html_result(self) -> HtmlResult {
        self
    }
}
impl IntoHtmlResult for Html {
    #[inline(always)]
    fn into_html_result(self) -> HtmlResult {
        Ok(self)
    }
}

/// Render children into a DOM node that exists outside the hierarchy of the parent
/// component.
/// ## Relevant examples
/// - [Portals](https://github.com/yewstack/yew/tree/master/examples/portals)
pub fn create_portal(child: Html, host: Element) -> Html {
    VNode::VPortal(VPortal::new(child, host))
}
