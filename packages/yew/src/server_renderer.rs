use std::fmt;

use futures::stream::{Stream, StreamExt};

use crate::html::{BaseComponent, Scope};
use crate::platform::io::{self, DEFAULT_BUF_SIZE};
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
/// This renderer spawns the rendering task to an internal worker pool and receives result when
/// the rendering process has finished.
///
/// See [`yew::platform`] for more information.
#[cfg_attr(documenting, doc(cfg(feature = "ssr")))]
pub struct ServerRenderer<COMP>
where
    COMP: BaseComponent,
{
    create_props: Box<dyn Send + FnOnce() -> COMP::Properties>,
    hydratable: bool,
    capacity: usize,
}

impl<COMP> fmt::Debug for ServerRenderer<COMP>
where
    COMP: BaseComponent,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ServerRenderer<_>")
    }
}

impl<COMP> Default for ServerRenderer<COMP>
where
    COMP: BaseComponent,
    COMP::Properties: Default,
{
    fn default() -> Self {
        Self::with_props(Default::default)
    }
}

impl<COMP> ServerRenderer<COMP>
where
    COMP: BaseComponent,
    COMP::Properties: Default,
{
    /// Creates a [ServerRenderer] with default properties.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<COMP> ServerRenderer<COMP>
where
    COMP: BaseComponent,
{
    /// Creates a [ServerRenderer] with custom properties.
    ///
    /// # Note
    ///
    /// The properties does not have to implement `Send`.
    /// However, the function to create properties needs to be `Send`.
    pub fn with_props<F>(create_props: F) -> Self
    where
        F: 'static + Send + FnOnce() -> COMP::Properties,
    {
        Self {
            create_props: Box::new(create_props),
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
                create_props,
                hydratable,
                capacity,
            } = self;

            let props = create_props();

            LocalServerRenderer::<COMP>::with_props(props)
                .hydratable(hydratable)
                .capacity(capacity)
                .render_stream()
        })
        .await
    }
}
