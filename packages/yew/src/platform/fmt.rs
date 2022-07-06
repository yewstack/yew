//! This module contains types for I/O functionality.

// This module should remain private until impl trait type alias becomes available so
// `BufReader` can be produced with an existential type.

use std::borrow::Cow;

use crate::platform::pinned;

// Same as std::io::BufWriter and futures::io::BufWriter.
pub(crate) const DEFAULT_BUF_SIZE: usize = 8 * 1024;

pub(crate) trait BufSend {
    fn buf_send(&self, item: String);
}

impl BufSend for pinned::mpsc::UnboundedSender<String> {
    fn buf_send(&self, item: String) {
        let _ = self.send_now(item);
    }
}

impl BufSend for futures::channel::mpsc::UnboundedSender<String> {
    fn buf_send(&self, item: String) {
        let _ = self.unbounded_send(item);
    }
}

pub trait BufWrite {
    fn capacity(&self) -> usize;
    fn write(&mut self, s: Cow<'_, str>);
}

/// A [`futures::io::BufWriter`], but operates over string and yields into a Stream.
pub(crate) struct BufWriter<S>
where
    S: BufSend,
{
    buf: String,
    tx: S,
    capacity: usize,
}

// Implementation Notes:
//
// When jemalloc is used and a reasonable buffer length is chosen,
// performance of this buffer is related to the number of allocations
// instead of the amount of memory that is allocated.
//
// A Bytes-based implementation is also tested, and yielded a similar performance to String-based
// buffer.
//
// Having a String-based buffer avoids unsafe / cost of conversion between String and Bytes
// when text based content is needed (e.g.: post-processing).
//
// `Bytes::from` can be used to convert a `String` to `Bytes` if web server asks for an
// `impl Stream<Item = Bytes>`. This conversion incurs no memory allocation.
//
// Yielding the output with a Stream provides a couple advantages:
//
// 1. All child components of a VList can have their own buffer and be rendered concurrently.
// 2. If a fixed buffer is used, the rendering process can become blocked if the buffer is filled.
//    Using a stream avoids this side effect and allows the renderer to finish rendering
//    without being actively polled.
impl<S> BufWriter<S>
where
    S: BufSend,
{
    pub fn new(tx: S, capacity: usize) -> Self {
        Self {
            buf: String::new(),
            tx,
            capacity,
        }
    }

    #[inline]
    fn drain(&mut self) {
        if !self.buf.is_empty() {
            self.tx.buf_send(self.buf.split_off(0));
        }
    }

    #[inline]
    fn reserve(&mut self) {
        if self.buf.is_empty() {
            self.buf.reserve(self.capacity);
        }
    }

    /// Returns `True` if the internal buffer has capacity to fit a string of certain length.
    #[inline]
    fn has_capacity_of(&self, next_part_len: usize) -> bool {
        self.buf.capacity() >= self.buf.len() + next_part_len
    }
}

impl<S> BufWrite for BufWriter<S>
where
    S: BufSend,
{
    #[inline]
    fn capacity(&self) -> usize {
        self.capacity
    }

    /// Writes a string into the buffer, optionally drains the buffer.
    fn write(&mut self, s: Cow<'_, str>) {
        // Try to reserve the capacity first.
        self.reserve();

        if !self.has_capacity_of(s.len()) {
            // There isn't enough capacity, we drain the buffer.
            self.drain();
        }

        if self.has_capacity_of(s.len()) {
            // The next part is going to fit into the buffer, we push it onto the buffer.
            self.buf.push_str(&s);
        } else {
            // if the next part is more than buffer size, we send the next part.

            // We don't need to drain the buffer here as the result of self.has_capacity_of() only
            // changes if the buffer was drained. If the buffer capacity didn't change,
            // then it means self.has_capacity_of() has returned true the first time which will be
            // guaranteed to be matched by the left hand side of this implementation.
            self.tx.buf_send(s.into_owned());
        }
    }
}

impl<S> Drop for BufWriter<S>
where
    S: BufSend,
{
    fn drop(&mut self) {
        if !self.buf.is_empty() {
            let mut buf = String::new();
            std::mem::swap(&mut buf, &mut self.buf);
            self.tx.buf_send(buf);
        }
    }
}
