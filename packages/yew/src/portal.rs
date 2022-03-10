//! a module to create a "Portal" that attaches its children to the specified host element.

use crate::functional::{use_effect_with_deps, use_state};
use crate::html::{Children, Html, Properties};
use crate::virtual_dom::{VNode, VPortal};
use crate::{function_component, html};

use web_sys::Element;

/// Properties of [Portal].
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct PortalProps {
    /// Children to be rendered to the host element.
    #[prop_or_default]
    pub children: Children,

    /// The host element of the portal.
    pub host: Element,
}

/// Render children into a DOM node that exists outside the hierarchy of the parent
/// component.
/// ## Relevant examples
/// - [Portals](https://github.com/yewstack/yew/tree/master/examples/portals)
#[function_component]
pub fn Portal(props: &PortalProps) -> Html {
    let rendered = use_state(|| false);

    // Delay render of portals until after first render.
    //
    // This automatically excludes portals during server-side rendering and defers
    // it to be attached after hydration is completed.
    {
        let rendered = rendered.clone();
        use_effect_with_deps(
            move |_| {
                rendered.set(true);

                || {}
            },
            (),
        );
    }

    if *rendered {
        let PortalProps { children, host } = props.clone();
        let children = html! {<>{children}</>};

        VNode::VPortal(VPortal::new(children, host))
    } else {
        Html::default()
    }
}
