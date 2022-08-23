//! This module contains types for I/O functionality.

// This module should remain private until impl trait type alias becomes available so
// `BufReader` can be produced with an existential type.

use std::borrow::Cow;

use futures::stream::Stream;

use crate::platform::sync::mpsc::{self, UnboundedReceiverStream, UnboundedSender};

// Same as std::io::BufWriter and futures::io::BufWriter.
pub(crate) const DEFAULT_BUF_SIZE: usize = 8 * 1024;

/// A [`futures::io::BufWriter`], but operates over string and yields into a Stream.
pub(crate) struct BufWriter {
    buf: String,
    tx: UnboundedSender<String>,
    capacity: usize,
}

/// Creates a Buffer pair.
pub(crate) fn buffer(capacity: usize) -> (BufWriter, impl Stream<Item = String>) {
    let (tx, rx) = mpsc::unbounded_channel::<String>();

    let tx = BufWriter {
        buf: String::with_capacity(capacity),
        tx,
        capacity,
    };

    (tx, UnboundedReceiverStream::new(rx))
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
impl BufWriter {
    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    fn drain(&mut self) {
        let _ = self.tx.send(self.buf.drain(..).collect());
        self.buf.reserve(self.capacity);
    }

    /// Returns `True` if the internal buffer has capacity to fit a string of certain length.
    #[inline]
    fn has_capacity_of(&self, next_part_len: usize) -> bool {
        self.buf.capacity() >= self.buf.len() + next_part_len
    }

    /// Writes a string into the buffer, optionally drains the buffer.
    pub fn write(&mut self, s: Cow<'_, str>) {
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
            let _ = self.tx.send(s.into_owned());
        }
    }
}

impl Drop for BufWriter {
    fn drop(&mut self) {
        if !self.buf.is_empty() {
            let mut buf = String::new();
            std::mem::swap(&mut buf, &mut self.buf);
            let _ = self.tx.send(buf);
        }
    }
}
