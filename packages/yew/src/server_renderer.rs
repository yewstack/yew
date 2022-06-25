use futures::stream::{Stream, StreamExt};

use crate::html::{BaseComponent, Scope};
use crate::io::{self, DEFAULT_BUF_SIZE};
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
    capacity: usize,
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
            capacity: DEFAULT_BUF_SIZE,
        }
    }

    /// Sets the capacity of renderer buffer.
    ///
    /// Default: `8192`
    pub fn capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;

        self
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
        let mut s = self.render_stream();

        while let Some(m) = s.next().await {
            w.push_str(&m);
        }
    }

    /// Renders Yew Applications into a string Stream
    pub fn render_stream(self) -> impl Stream<Item = String> {
        let (mut w, r) = io::buffer(self.capacity);

        let scope = Scope::<COMP>::new(None);
        spawn_local(async move {
            scope
                .render_into_stream(&mut w, self.props.into(), self.hydratable)
                .await;
        });

        r
    }
}

/// A Yew Server-side Renderer.
///
/// The renderer spawns the rendering task with [`run_pinned`] which maintains an internal worker
/// pool.
#[cfg_attr(documenting, doc(cfg(feature = "ssr")))]
#[derive(Debug)]
pub struct ServerRenderer<COMP>
where
    COMP: BaseComponent,
    COMP::Properties: Send,
{
    props: COMP::Properties,
    hydratable: bool,
    capacity: usize,
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
            capacity: DEFAULT_BUF_SIZE,
        }
    }

    /// Sets the capacity of renderer buffer.
    ///
    /// Default: `8192`
    pub fn capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;

        self
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
        let mut s = self.render_stream().await;

        while let Some(m) = s.next().await {
            w.push_str(&m);
        }
    }

    /// Renders Yew Applications into a string Stream.
    ///
    /// # Note
    ///
    /// Unlike [`LocalServerRenderer::render_stream`], this method is `async fn`.
    pub async fn render_stream(self) -> impl Stream<Item = String> {
        // We use run_pinned to switch to our runtime.
        run_pinned(move || async move {
            let Self {
                props,
                hydratable,
                capacity,
            } = self;

            LocalServerRenderer::<COMP>::with_props(props)
                .hydratable(hydratable)
                .capacity(capacity)
                .render_stream()
        })
        .await
    }
}
