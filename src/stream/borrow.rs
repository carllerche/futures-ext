use futures::{Poll, Async};
use futures::stream::Stream;

pub trait BorrowStream<'a> {

    /// The type of item this stream will yield on success.
    type Item;

    /// The type of error this stream may generate.
    type Error;

    fn poll(&'a mut self) -> Poll<Option<Self::Item>, Self::Error>;
}
