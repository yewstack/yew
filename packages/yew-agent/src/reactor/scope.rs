use std::convert::Infallible;
use std::fmt;
use std::pin::Pin;

use futures::stream::{FusedStream, Stream};
use futures::task::{Context, Poll};
use futures::Sink;

/// A handle to communicate with bridges.
pub struct ReactorScope<I, O> {
    input_stream: Pin<Box<dyn FusedStream<Item = I>>>,
    output_sink: Pin<Box<dyn Sink<O, Error = Infallible>>>,
}

impl<I, O> fmt::Debug for ReactorScope<I, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReactorScope<_>").finish()
    }
}

impl<I, O> Stream for ReactorScope<I, O> {
    type Item = I;

    #[inline(always)]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.input_stream).poll_next(cx)
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.input_stream.size_hint()
    }
}

impl<I, O> FusedStream for ReactorScope<I, O> {
    #[inline(always)]
    fn is_terminated(&self) -> bool {
        self.input_stream.is_terminated()
    }
}

/// A helper trait to extract the input and output type from a [ReactorStream].
pub trait ReactorScoped: Stream + FusedStream {
    /// The Input Message.
    type Input;
    /// The Output Message.
    type Output;

    /// Creates a ReactorReceiver.
    fn new<IS, OS>(input_stream: IS, output_sink: OS) -> Self
    where
        IS: Stream<Item = Self::Input> + FusedStream + 'static,
        OS: Sink<Self::Output, Error = Infallible> + 'static;
}

impl<I, O> ReactorScoped for ReactorScope<I, O> {
    type Input = I;
    type Output = O;

    #[inline]
    fn new<IS, OS>(input_stream: IS, output_sink: OS) -> Self
    where
        IS: Stream<Item = Self::Input> + FusedStream + 'static,
        OS: Sink<Self::Output, Error = Infallible> + 'static,
    {
        Self {
            input_stream: Box::pin(input_stream),
            output_sink: Box::pin(output_sink),
        }
    }
}

impl<I, O> Sink<O> for ReactorScope<I, O> {
    type Error = Infallible;

    fn start_send(mut self: Pin<&mut Self>, item: O) -> Result<(), Self::Error> {
        Pin::new(&mut self.output_sink).start_send(item)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.output_sink).poll_close(cx)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.output_sink).poll_flush(cx)
    }

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.output_sink).poll_flush(cx)
    }
}
