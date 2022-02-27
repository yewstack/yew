use super::*;

use crate::html::Scope;

/// A Yew Server-side Renderer.
#[cfg_attr(documenting, doc(cfg(feature = "ssr")))]
#[derive(Debug)]
pub struct ServerRenderer<ICOMP>
where
    ICOMP: IntoComponent,
{
    props: ICOMP::Properties,
}

impl<ICOMP> Default for ServerRenderer<ICOMP>
where
    ICOMP: IntoComponent,
    ICOMP::Properties: Default,
{
    fn default() -> Self {
        Self::with_props(ICOMP::Properties::default())
    }
}

impl<ICOMP> ServerRenderer<ICOMP>
where
    ICOMP: IntoComponent,
    ICOMP::Properties: Default,
{
    /// Creates a [ServerRenderer] with default properties.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<ICOMP> ServerRenderer<ICOMP>
where
    ICOMP: IntoComponent,
{
    /// Creates a [ServerRenderer] with custom properties.
    pub fn with_props(props: ICOMP::Properties) -> Self {
        Self { props }
    }

    /// Renders Yew Application.
    pub async fn render(self) -> String {
        let mut s = String::new();

        self.render_to_string(&mut s).await;

        s
    }

    /// Renders Yew Application to a String.
    pub async fn render_to_string(self, w: &mut String) {
        let scope = Scope::<<ICOMP as IntoComponent>::Component>::new(None);
        scope.render_to_string(w, self.props.into()).await;
    }
}
