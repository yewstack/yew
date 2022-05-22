use std::borrow::Cow;

use futures::channel::mpsc;
use futures::stream::{Stream, StreamExt};

use crate::html::{BaseComponent, Scope};
use crate::platform::{run_pinned, spawn_local};

/// A Yew Server-side Renderer that renders on the current thread.
#[cfg_attr(documenting, doc(cfg(feature = "ssr")))]
#[derive(Debug)]
pub struct LocalServerRenderer<COMP>
where
    COMP: BaseComponent,
{
    props: COMP::Properties,
    hydratable: bool,
}

impl<COMP> Default for LocalServerRenderer<COMP>
where
    COMP: BaseComponent,
    COMP::Properties: Default,
{
    fn default() -> Self {
        Self::with_props(COMP::Properties::default())
    }
}

impl<COMP> LocalServerRenderer<COMP>
where
    COMP: BaseComponent,
    COMP::Properties: Default,
{
    /// Creates a [LocalServerRenderer] with default properties.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<COMP> LocalServerRenderer<COMP>
where
    COMP: BaseComponent,
{
    /// Creates a [LocalServerRenderer] with custom properties.
    pub fn with_props(props: COMP::Properties) -> Self {
        Self {
            props,
            hydratable: true,
        }
    }

    /// Sets whether an the rendered result is hydratable.
    ///
    /// Defaults to `true`.
    ///
    /// When this is sets to `true`, the rendered artifact will include additional information
    /// to assist with the hydration process.
    pub fn hydratable(mut self, val: bool) -> Self {
        self.hydratable = val;

        self
    }

    /// Renders Yew Application.
    pub async fn render(self) -> String {
        let mut s = String::new();

        self.render_to_string(&mut s).await;

        s
    }

    /// Renders Yew Application to a String.
    pub async fn render_to_string(self, w: &mut String) {
        let mut s = self.render_streamed().await;

        while let Some(m) = s.next().await {
            w.push_str(&m);
        }
    }

    /// Renders Yew Applications into a string Stream
    // Whilst not required to be async here, this function is async to keep the same function
    // signature as the ServerRenderer.
    pub async fn render_streamed(self) -> impl Stream<Item = Cow<'static, str>> {
        let (mut tx, rx) = mpsc::unbounded::<Cow<'static, str>>();

        let scope = Scope::<COMP>::new(None);
        spawn_local(async move {
            scope
                .render_streamed(&mut tx, self.props.into(), self.hydratable)
                .await;
        });

        rx
    }
}

/// A Yew Server-side Renderer.
///
/// For runtimes with multi-threading support,
/// Yew manages a default worker pool with the number of worker thread equal to the CPU running
/// cores. You may override the spawning logic with
#[cfg_attr(documenting, doc(cfg(feature = "ssr")))]
#[derive(Debug)]
pub struct ServerRenderer<COMP>
where
    COMP: BaseComponent,
    COMP::Properties: Send,
{
    props: COMP::Properties,
    hydratable: bool,
}

impl<COMP> Default for ServerRenderer<COMP>
where
    COMP: BaseComponent,
    COMP::Properties: Default + Send,
{
    fn default() -> Self {
        Self::with_props(COMP::Properties::default())
    }
}

impl<COMP> ServerRenderer<COMP>
where
    COMP: BaseComponent,
    COMP::Properties: Default + Send,
{
    /// Creates a [ServerRenderer] with default properties.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<COMP> ServerRenderer<COMP>
where
    COMP: BaseComponent,
    COMP::Properties: Send,
{
    /// Creates a [ServerRenderer] with custom properties.
    pub fn with_props(props: COMP::Properties) -> Self {
        Self {
            props,
            hydratable: true,
        }
    }

    /// Sets whether an the rendered result is hydratable.
    ///
    /// Defaults to `true`.
    ///
    /// When this is sets to `true`, the rendered artifact will include additional information
    /// to assist with the hydration process.
    pub fn hydratable(mut self, val: bool) -> Self {
        self.hydratable = val;

        self
    }

    /// Renders Yew Application.
    pub async fn render(self) -> String {
        let mut s = String::new();

        self.render_to_string(&mut s).await;

        s
    }

    /// Renders Yew Application to a String.
    pub async fn render_to_string(self, w: &mut String) {
        let mut s = self.render_streamed().await;

        while let Some(m) = s.next().await {
            w.push_str(&m);
        }
    }

    /// Renders Yew Applications into a string Stream.
    pub async fn render_streamed(self) -> impl Stream<Item = Cow<'static, str>> {
        let Self { props, hydratable } = self;

        run_pinned(move || async move {
            LocalServerRenderer::<COMP>::with_props(props)
                .hydratable(hydratable)
                .render_streamed()
                .await
        })
        .await
    }
}
