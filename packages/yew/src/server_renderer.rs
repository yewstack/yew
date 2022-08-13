use std::fmt;

use futures::pin_mut;
use futures::stream::{Stream, StreamExt};
use tracing::Instrument;

use crate::html::{BaseComponent, Scope};
use crate::platform::fmt::BufStream;
use crate::platform::{run_pinned, spawn_local};

/// A Yew Server-side Renderer that renders on the current thread.
#[cfg(feature = "ssr")]
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
        let s = self.render_stream();
        futures::pin_mut!(s);

        while let Some(m) = s.next().await {
            w.push_str(&m);
        }
    }

    /// Renders Yew Applications into a string Stream
    #[tracing::instrument(
        level = tracing::Level::DEBUG,
        name = "render",
        skip(self),
        fields(hydratable = self.hydratable),
    )]
    pub fn render_stream(self) -> impl Stream<Item = String> {
        let scope = Scope::<COMP>::new(None);

        let outer_span = tracing::Span::current();
        BufStream::new(move |mut w| async move {
            let render_span = tracing::debug_span!("render_stream_item");
            render_span.follows_from(outer_span);
            scope
                .render_into_stream(&mut w, self.props.into(), self.hydratable)
                .instrument(render_span)
                .await;
        })
    }
}

/// A Yew Server-side Renderer.
///
/// This renderer spawns the rendering task to an internal worker pool and receives result when
/// the rendering process has finished.
///
/// See [`yew::platform`] for more information.
#[cfg(feature = "ssr")]
pub struct ServerRenderer<COMP>
where
    COMP: BaseComponent,
{
    create_props: Box<dyn Send + FnOnce() -> COMP::Properties>,
    hydratable: bool,
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
        let Self {
            create_props,
            hydratable,
        } = self;

        run_pinned(move || async move {
            let (tx, rx) = futures::channel::mpsc::unbounded();

            spawn_local(async move {
                let props = create_props();
                let s = LocalServerRenderer::<COMP>::with_props(props)
                    .hydratable(hydratable)
                    .render_stream();
                pin_mut!(s);

                while let Some(m) = s.next().await {
                    let _ = tx.unbounded_send(m);
                }
            });

            rx
        })
        .await
    }
}
