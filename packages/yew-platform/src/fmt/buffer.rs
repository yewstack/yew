use std::cell::UnsafeCell;
use std::fmt::{self, Write};
use std::marker::PhantomData;
use std::rc::Rc;
use std::task::{Poll, Waker};

use futures::stream::{FusedStream, Stream};

static BUF_SIZE: usize = 1024;

enum BufStreamState {
    Ready,
    Pending(Waker),
    Done,
}

struct Inner {
    buf: String,
    state: BufStreamState,

    // This type is not send or sync.
    _marker: PhantomData<Rc<()>>,
}

impl Inner {
    #[inline]
    const fn new() -> Self {
        Self {
            buf: String::new(),
            state: BufStreamState::Ready,
            _marker: PhantomData,
        }
    }

    #[inline]
    fn wake(&mut self) {
        if let BufStreamState::Pending(ref waker) = self.state {
            waker.wake_by_ref();
            self.state = BufStreamState::Ready;
        }
    }

    #[inline]
    fn buf_reserve(&mut self) {
        if self.buf.is_empty() {
            self.buf.reserve(BUF_SIZE);
        }
    }
}

impl Write for Inner {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s.is_empty() {
            return Ok(());
        }

        self.wake();
        if s.len() < BUF_SIZE {
            self.buf_reserve();
        }

        self.buf.write_str(s)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.wake();
        self.buf_reserve();

        self.buf.write_char(c)
    }

    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        self.wake();
        self.buf_reserve();

        self.buf.write_fmt(args)
    }
}

/// An asynchronous [`String`] writer.
///
/// This type implements [`fmt::Write`] and can be used with [`write!`] and [`writeln!`].
#[derive(Debug)]
pub struct BufWriter {
    inner: Rc<UnsafeCell<Inner>>,
}

impl Write for BufWriter {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // SAFETY:
        //
        // We can acquire a mutable reference without checking as:
        //
        // - This type is !Sync and !Send.
        // - This function is not used by any other functions that has access to the inner type.
        // - The mutable reference is dropped at the end of this function.
        let inner = unsafe { &mut *self.inner.get() };

        inner.write_str(s)
    }

    #[inline]
    fn write_char(&mut self, c: char) -> fmt::Result {
        // SAFETY:
        //
        // We can acquire a mutable reference without checking as:
        //
        // - This type is !Sync and !Send.
        // - This function is not used by any other functions that has access to the inner type.
        // - The mutable reference is dropped at the end of this function.
        let inner = unsafe { &mut *self.inner.get() };

        inner.write_char(c)
    }

    #[inline]
    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        // SAFETY:
        //
        // We can acquire a mutable reference without checking as:
        //
        // - This type is !Sync and !Send.
        // - This function is not used by any other functions that has access to the inner type.
        // - The mutable reference is dropped at the end of this function.
        let inner = unsafe { &mut *self.inner.get() };

        inner.write_fmt(args)
    }
}

impl Drop for BufWriter {
    fn drop(&mut self) {
        // SAFETY:
        //
        // We can acquire a mutable reference without checking as:
        //
        // - This type is !Sync and !Send.
        // - This function is not used by any other functions that has access to the inner type.
        // - The mutable reference is dropped at the end of this function.
        let inner = unsafe { &mut *self.inner.get() };

        inner.wake();
        inner.state = BufStreamState::Done;
    }
}

/// An asynchronous [`String`] reader.
#[derive(Debug)]
pub struct BufReader {
    inner: Rc<UnsafeCell<Inner>>,
}

impl Stream for BufReader {
    type Item = String;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        // SAFETY:
        //
        // We can acquire a mutable reference without checking as:
        //
        // - This type is !Sync and !Send.
        // - This function is not used by any other functions that has access to the inner type.
        // - The mutable reference is dropped at the end of this function.
        let inner = unsafe { &mut *self.inner.get() };

        if !inner.buf.is_empty() {
            let buf = std::mem::take(&mut inner.buf);
            return Poll::Ready(Some(buf));
        }

        if let BufStreamState::Done = inner.state {
            return Poll::Ready(None);
        }

        inner.state = BufStreamState::Pending(cx.waker().clone());
        Poll::Pending
    }
}

impl FusedStream for BufReader {
    fn is_terminated(&self) -> bool {
        // SAFETY:
        //
        // We can acquire a mutable reference without checking as:
        //
        // - This type is !Sync and !Send.
        // - This function is not used by any other functions that has access to the inner type.
        // - The mutable reference is dropped at the end of this function.
        let inner = unsafe { &*self.inner.get() };

        matches!(
            (&inner.state, inner.buf.is_empty()),
            (BufStreamState::Done, true)
        )
    }
}

/// Creates an asynchronous buffer that operates over String.
pub fn buffer() -> (BufWriter, BufReader) {
    let inner = Rc::new(UnsafeCell::new(Inner::new()));

    let w = {
        let inner = inner.clone();
        BufWriter { inner }
    };

    let r = BufReader { inner };

    (w, r)
}
