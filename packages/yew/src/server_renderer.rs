use std::fmt;
use std::future::Future;

use futures::pin_mut;
use futures::stream::{Stream, StreamExt};
use tracing::Instrument;

use crate::html::{BaseComponent, Scope};
use crate::platform::fmt::BufStream;
use crate::platform::{LocalHandle, Runtime};

/// A Yew Server-side Renderer that renders on the current thread.
///
/// # Note
///
/// This renderer does not spawn its own runtime and can only be used when:
///
/// - `wasm-bindgen-futures` is selected as the backend of Yew runtime.
/// - running within a [`Runtime`](crate::platform::Runtime).
/// - running within a tokio [`LocalSet`](struct@tokio::task::LocalSet).
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
        let s = self.render_stream();
        futures::pin_mut!(s);

        s.collect().await
    }

    /// Renders Yew Application to a String.
    pub async fn render_to_string(self, w: &mut String) {
        let s = self.render_stream();
        futures::pin_mut!(s);

        while let Some(m) = s.next().await {
            w.push_str(&m);
        }
    }

    /// Renders Yew Application into a string Stream
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
/// This renderer spawns the rendering task to a Yew [`Runtime`]. and receives result when
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
    rt: Option<Runtime>,
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
            rt: None,
        }
    }

    /// Sets the runtime the ServerRenderer will run the rendering task with.
    pub fn with_runtime(mut self, rt: Runtime) -> Self {
        self.rt = Some(rt);

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
        let Self {
            create_props,
            hydratable,
            rt,
        } = self;

        let (tx, rx) = futures::channel::oneshot::channel();
        let create_task = move || async move {
            let props = create_props();
            let s = LocalServerRenderer::<COMP>::with_props(props)
                .hydratable(hydratable)
                .render()
                .await;

            let _ = tx.send(s);
        };

        Self::spawn_rendering_task(rt, create_task);

        rx.await.expect("failed to render application")
    }

    /// Renders Yew Application to a String.
    pub async fn render_to_string(self, w: &mut String) {
        let mut s = self.render_stream();

        while let Some(m) = s.next().await {
            w.push_str(&m);
        }
    }

    #[inline]
    fn spawn_rendering_task<F, Fut>(rt: Option<Runtime>, create_task: F)
    where
        F: 'static + Send + FnOnce() -> Fut,
        Fut: Future<Output = ()> + 'static,
    {
        match rt {
            // If a runtime is specified, spawn to the specified runtime.
            Some(m) => m.spawn_pinned(create_task),
            None => match LocalHandle::try_current() {
                // If within a Yew Runtime, spawn to the current runtime.
                Some(m) => m.spawn_local(create_task()),
                // Outside of Yew Runtime, spawn to the default runtime.
                None => Runtime::default().spawn_pinned(create_task),
            },
        }
    }

    /// Renders Yew Application into a string Stream.
    pub fn render_stream(self) -> impl Send + Stream<Item = String> {
        let Self {
            create_props,
            hydratable,
            rt,
        } = self;

        let (tx, rx) = futures::channel::mpsc::unbounded();
        let create_task = move || async move {
            let props = create_props();
            let s = LocalServerRenderer::<COMP>::with_props(props)
                .hydratable(hydratable)
                .render_stream();
            pin_mut!(s);

            while let Some(m) = s.next().await {
                let _ = tx.unbounded_send(m);
            }
        };

        Self::spawn_rendering_task(rt, create_task);

        rx
    }
}
