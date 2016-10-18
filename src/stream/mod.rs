mod first;

pub use self::first::First;

use futures::stream::Stream;

pub trait StreamExt: Stream {
    fn first(self) -> First<Self> where Self: Sized {
        First::new(self)
    }
}

impl<T: Stream> StreamExt for T {
}
