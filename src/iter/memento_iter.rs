pub trait MementoIterAdapter: Iterator {
    fn memento(self) -> MementoIter<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        MementoIter::new(self)
    }
}

impl<I> MementoIterAdapter for I where I: Iterator {}

/// Iterator with memory. Knows the actual and previous elements.
///
/// # Examples
///
/// ```
/// use xstd::prelude::*;
///
/// let mut iter = std::iter::once(2).memento();
///
/// assert_eq!(Some(2), iter.next());
/// assert_eq!(None, iter.prev());
/// assert_eq!(Some(&2), iter.cur());
///
/// assert_eq!(None, iter.next());
/// assert_eq!(Some(&2), iter.prev());
/// assert_eq!(None, iter.cur());
/// ```
#[derive(Debug)]
pub struct MementoIter<I>
where
    I: Iterator,
{
    iter: I,
    cur: Option<I::Item>,
    prev: Option<I::Item>,
}

impl<I> MementoIter<I>
where
    I: Iterator,
    I::Item: Clone,
{
    pub(super) fn new(iter: I) -> Self {
        Self {
            iter,
            cur: None,
            prev: None,
        }
    }

    /// Returns the current element
    pub fn cur(&self) -> Option<&I::Item> {
        self.cur.as_ref()
    }

    /// Returns the previous element
    pub fn prev(&self) -> Option<&I::Item> {
        self.prev.as_ref()
    }
}

impl<I> Iterator for MementoIter<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(_) = self.cur {
            self.prev = self.cur.take();
        }

        let cur = self.iter.next();
        self.cur = cur.clone();
        cur
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let mut iter = (1..=2).memento();
        assert_eq!((None, None), (iter.prev(), iter.cur()));
        assert_eq!(Some(1), iter.next());
        assert_eq!((None, Some(&1)), (iter.prev(), iter.cur()));
        assert_eq!(Some(2), iter.next());
        assert_eq!((Some(&1), Some(&2)), (iter.prev(), iter.cur()));
        assert_eq!(None, iter.next());
        assert_eq!((Some(&2), None), (iter.prev(), iter.cur()));
    }
}
