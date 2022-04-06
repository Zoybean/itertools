#[cfg(test)]
pub mod test {
    use crate::*;
    use std::cell::RefCell;

    #[test]
    fn test() {
        test_counter::<0>();
        test_counter::<1>();
        test_counter::<2>();
        test_counter::<3>();
        test_counter::<4>();
        test_counter::<5>();
        test_counter::<1000>();
    }
    fn test_counter<const N: usize>() {
        let c = RefCell::new(0);
        let cc = Counts(&c);
        let _ = [(); N].into_iter().zip_clones(cc).for_each(|_| {});
        // 1 fewer clone than the number of items in the collection
        assert_eq!(N.saturating_sub(1), *c.borrow());
    }

    struct Counts<'a>(&'a RefCell<usize>);
    impl<'a> Counts<'a> {
        fn new(counter: &'a RefCell<usize>) -> Self {
            Self(counter)
        }
    }
    impl<'a> Clone for Counts<'a> {
        fn clone(&self) -> Self {
            *self.0.borrow_mut() += 1;
            Self::new(self.0)
        }
    }
}
use std::iter::Peekable;
use std::mem;

pub enum ZipClone<I, E>
where
    I: Iterator,
{
    /// The underlying iterator was exhausted. There is nothing left
    Empty,
    /// There is at least one element in the underlying iterator. We checked.
    More(E, Peekable<I>),
}
impl<I, E> ZipClone<I, E>
where
    I: Iterator,
{
    pub fn new(it: I, elem: E) -> Self {
        let mut it = it.peekable();
        if it.peek().is_none() {
            Self::Empty
        } else {
            Self::More(elem, it)
        }
    }
}

impl<I, T, E> Iterator for ZipClone<I, E>
where
    E: Clone,
    I: Iterator<Item = T>,
{
    type Item = (T, E);
    fn next(&mut self) -> Option<Self::Item> {
        match mem::replace(self, Self::Empty) {
            Self::Empty => None,
            Self::More(elem, mut it) => {
                let next = it.next().expect("We already checked. This is present");
                let last = it.peek().is_none();
                if last {
                    // leave the iterator empty after this
                    Some((next, elem))
                } else {
                    let cloned = elem.clone();
                    // we checked, and there's more after this
                    *self = Self::More(elem, it);
                    Some((next, cloned))
                }
            }
        }
    }
}
