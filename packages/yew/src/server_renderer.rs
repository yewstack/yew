use std::borrow::Cow;

use futures::channel::mpsc::{self, UnboundedSender};
use futures::stream::{Stream, StreamExt};

use crate::html::{BaseComponent, Scope};
use crate::platform::{run_pinned, spawn_local};

// Same as std::io::BufWriter and futures::io::BufWriter.
const DEFAULT_BUF_SIZE: usize = 8 * 1024;

/// A [`futures::io::BufWriter`], but operates over string and yields into a Stream.
pub(crate) struct BufWriter {
    buf: String,
    tx: UnboundedSender<String>,
    capacity: usize,
}

// Implementation Notes:
//
// When jemalloc is used, performance of the BufWriter is related to the number of allocations
// instead of the amount of memory that is allocated.
//
// A bytes::Bytes-based implementation is also tested, and yielded a similar performance.
//
// Having a String-based buffer avoids `unsafe { str::from_utf8_unchecked(..) }` or performance
// penalty by `str::from_utf8(..)` when converting back to String when text based content is needed
// (e.g.: post-processing).
//
// `Bytes::from` can be used to convert a `String` to `Bytes` if the web server asks for an
// `impl Stream<Item = Bytes>`. This conversion incurs no memory allocation.
impl BufWriter {
    pub fn with_capacity(capacity: usize) -> (Self, impl Stream<Item = String>) {
        let (tx, rx) = mpsc::unbounded::<String>();

        let this = Self {
            buf: String::with_capacity(capacity),
            tx,
            capacity,
        };

        (this, rx)
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    fn drain(&mut self) {
        let _ = self.tx.unbounded_send(self.buf.drain(..).collect());
        self.buf.reserve(self.capacity);
    }

    /// Writes a string into the buffer, optionally drains the buffer.
    pub fn write(&mut self, s: Cow<'_, str>) {
        if self.buf.capacity() < s.len() {
            // There isn't enough capacity, we drain the buffer.
            self.drain();
        }

        // It's important to check self.buf.capacity() >= s.len():
        //
        // 1. self.buf.reserve() may choose to over reserve than capacity.
        // 2. When self.buf.capacity() == s.len(), the previous buffer is not drained. So it needs
        //    to push onto the buffer instead of sending.
        if self.buf.capacity() >= s.len() {
            // The next part is going to fit into the buffer, we push it onto the buffer.
            self.buf.push_str(&s);
        } else {
            // if the next part is more than buffer size, we send the next part.

            // We don't need to drain the buffer here as the self.buf.capacity() only changes if
            // the buffer was drained. If the buffer capacity didn't change, then it means
            // self.buf.capacity() > s.len() which will be guaranteed to be matched by
            // self.buf.capacity() >= s.len().
            let _ = self.tx.unbounded_send(s.into_owned());
        }
    }
}

impl Drop for BufWriter {
    fn drop(&mut self) {
        if !self.buf.is_empty() {
            let mut buf = String::new();
            std::mem::swap(&mut buf, &mut self.buf);
            let _ = self.tx.unbounded_send(buf);
        }
    }
}

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
        let mut s = self.render_stream().await;

        while let Some(m) = s.next().await {
            w.push_str(&m);
        }
    }

    /// Renders Yew Applications into a string Stream
    // Whilst not required to be async here, this function is async to keep the same function
    // signature as the ServerRenderer.
    pub async fn render_stream(self) -> impl Stream<Item = String> {
        let (mut w, rx) = BufWriter::with_capacity(self.capacity);

        let scope = Scope::<COMP>::new(None);
        spawn_local(async move {
            scope
                .render_into_stream(&mut w, self.props.into(), self.hydratable)
                .await;
        });

        rx
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
    pub async fn render_stream(self) -> impl Stream<Item = String> {
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
                .await
        })
        .await
    }
}
