use std::pin::Pin;
use pin_utils::{unsafe_pinned, unsafe_unpinned};
use futures_core::{
    ready,
    stream::{
        Stream
    },
    task::{
        Context,
        Poll,
    }
};


pub trait UniqueBy: Stream {
    fn unique_by<F, E>(self, f: F) -> Unique<Self, F, E>
    where Self: Sized,
          F: FnMut(&Self::Item) -> E,
    {
        Unique::new(self, f)
    }
}

impl<T> UniqueBy for T where T: Stream {}

/// Creates a Stream which skips already emitted elements.
///
///
/// # Examples
///
/// ```
/// # futures::executor::block_on(async {
/// use xstd::prelude::*;
/// use futures::{stream::{self, StreamExt}, executor};
///
/// let mut stream = stream::iter(vec![3, 1, 3, 2, 1, 4])
///                     .unique_by(|x| x.clone());
///
/// assert_eq!(Some(3), stream.next().await);
/// assert_eq!(Some(1), stream.next().await);
/// assert_eq!(Some(2), stream.next().await);
/// assert_eq!(Some(4), stream.next().await);
/// # });
/// ```
#[must_use = "streams do nothing unless polled"]
pub struct Unique<St, F, E> {
    stream: St,
    f: F,
    seen: Vec<E>,
}

impl<St, F, E> Unique<St, F, E>
where St: Stream,
      F: FnMut(&St::Item) -> E,
{
    unsafe_pinned!(stream: St);
    unsafe_unpinned!(f: F);
    unsafe_unpinned!(seen: Vec<E>);

    pub fn new(stream: St, f: F) -> Unique<St, F, E>
    {
        Self { stream, f, seen: Vec::new() }
    }
}

impl<St, F, E> Stream for Unique<St, F, E>
where St: Stream,
      F: FnMut(&St::Item) -> E,
      E: PartialEq,
{
    type Item = St::Item;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<St::Item>> {
        match ready!(self.as_mut().stream().poll_next(cx)) {
            Some(item) => {
                let e = self.as_mut().f()(&item);
                if self.as_mut().seen().contains(&e) {
                    self.poll_next(cx)
                } else {
                    self.as_mut().seen().push(e);
                    Poll::Ready(Some(item))
                }
            },
            x => Poll::Ready(x),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use futures::{stream::{self, StreamExt}, executor};

    #[test]
    fn test_unique() {
        executor::block_on(async {
            let stream = stream::iter(vec![3, 1, 2, 3, 4, 1, 6, 3, 8])
                .unique_by(|x| x.clone());
            let res = stream.collect::<Vec<i32>>().await;
            assert_eq!(vec![3, 1, 2, 4, 6, 8], res);
        });
    }
}
