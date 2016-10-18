use futures::{Future, Poll};
use futures::stream::Stream;

/// Yields the first value of a stream
pub struct First<T> {
    stream: T,
}

impl<T> First<T> {
    pub fn new(stream: T) -> First<T> {
        First { stream: stream }
    }
}

impl<T: Stream> Future for First<T> {
    type Item = Option<T::Item>;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.stream.poll()
    }
}
