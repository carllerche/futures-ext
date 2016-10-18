use futures::{Future, IntoFuture, Poll};
use futures::stream::{Stream};

pub enum Either<A, B> {
    A(A),
    B(B),
}

impl<A, B> Either<A, B>
    where A: Future,
          B: Future<Item = A::Item, Error = A::Error>,
{
    pub fn a<T>(f: T) -> Either<A, B>
        where T: IntoFuture<Item = A::Item, Error = A::Error, Future = A>,
    {
        Either::A(f.into_future())
    }

    pub fn b<T>(f: T) -> Either<A, B>
        where T: IntoFuture<Item = B::Item, Error = B::Error, Future = B>,
    {
        Either::B(f.into_future())
    }
}

impl<A, B> Future for Either<A, B>
    where A: Future,
          B: Future<Item = A::Item, Error = A::Error>,
{
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match *self {
            Either::A(ref mut f) => f.poll(),
            Either::B(ref mut f) => f.poll(),
        }
    }
}

impl<A, B> Stream for Either<A, B> where
    A: Stream,
    B: Stream<Item=A::Item, Error=A::Error>,
{
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match *self {
            Either::A(ref mut s) => s.poll(),
            Either::B(ref mut s) => s.poll(),
        }
    }
}
