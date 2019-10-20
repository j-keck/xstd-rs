pub trait IntersperseIterAdapter: Iterator {
    fn intersperse(self, elem: Self::Item) -> IntersperseIter<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        IntersperseIter::new(self, elem)
    }
}
impl<I> IntersperseIterAdapter for I where I: Iterator {}

/// Intersperse the given element between the elements of the Iterator.
///
/// # Examples
///
/// ```
/// use xstd::prelude::*;
//
/// let iter = (1 ..= 3).intersperse(100);
/// assert_eq!(vec![1, 100, 2, 100, 3], iter.collect::<Vec<_>>());
/// ```
#[derive(Debug)]
pub struct IntersperseIter<I>
where
    I: Iterator,
    I::Item: Clone,
{
    iter: std::iter::Peekable<I>,
    elem: I::Item,
    emit_elem: bool,
}

impl<I> IntersperseIter<I>
where
    I: Iterator,
    I::Item: Clone,
{
    pub(super) fn new(iter: I, elem: I::Item) -> Self {
        let iter = iter.peekable();
        IntersperseIter {
            iter,
            elem,
            emit_elem: false,
        }
    }
}

impl<I> Iterator for IntersperseIter<I>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.iter.peek().is_some() {
            if self.emit_elem {
                self.emit_elem = false;
                Some(self.elem.clone())
            } else {
                self.emit_elem = true;
                self.iter.next()
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn keep_empty_iter_empty() {
        let mut iter = std::iter::empty::<i32>().intersperse(100);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn keep_singleton_iter_singleton() {
        let mut iter = std::iter::once(2).intersperse(100);
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn intersperse_between_elements() {
        let iter = (1..=3).intersperse(100);

        assert_eq!(vec![1, 100, 2, 100, 3], iter.collect::<Vec<_>>());
    }
}
