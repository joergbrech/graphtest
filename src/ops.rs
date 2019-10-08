//! a module for iterating over the indices of collections implementing  the [`std::ops::Index`] trait
//!
//! With the help of the accepted answer of
//! [this stackoverflow question](https://stackoverflow.com/questions/30630810/using-generic-iterators-instead-of-specific-list-types).
//!
//!  **Cons:**
//!  - SuperIndex indices cannot be quite as generic as the ones for Index, e.g. slices are not allowed
//!  - Need to explicitly implement SuperIndex for all my types
//!  - In each implementation, I must make sure that the two iterators are ordered consistently

/// an Enumerate struct that has two iterators, one for the index of a collection and one for the item at this index.
///
/// This is the return type of [`SuperIndex::enumerate`].
pub struct Enumerate<IndexIter, ItemIter> {
    index: IndexIter,
    item: ItemIter,
}

/// implements the [`Iterator`] trait for the new struct
impl<IndexIter, ItemIter> Iterator for Enumerate<IndexIter, ItemIter>
where
    IndexIter: Iterator,
    ItemIter: Iterator,
{
    type Item = (IndexIter::Item, ItemIter::Item);

    /// returns the next iterator
    #[inline]
    fn next(&mut self) -> Option<(IndexIter::Item, ItemIter::Item)> {
        self.index.next().map(|idx| {
            // CAUTION! We need to make sure that the index and item iterators are ordered consistently.
            // We are really just incrementing to iterators simultaneously here...
            (idx, self.item.next().unwrap())
        })
    }
}

/// trait for implementing over the indices of collections that implement [`std::ops::Index`].
///
/// It adds the enumerate function that returns an `Enumerate<IndexIter,ItemIter>` as an iterator.
pub trait SuperIndex<'a, Idx>: std::ops::Index<Idx> {
    type IndexIter: Iterator<Item = Idx>;
    type ItemIter: Iterator;

    /// enumerates over the indices and items of a collection
    fn enumerate(&'a self) -> Enumerate<Self::IndexIter, Self::ItemIter>;
}

/// implement the [`SuperIndex`] trait for [`Vec<T>`]
impl<'a, T: 'a> SuperIndex<'a, usize> for Vec<T> {
    type IndexIter = std::ops::Range<usize>;
    type ItemIter = std::slice::Iter<'a, T>;

    fn enumerate(&'a self) -> Enumerate<Self::IndexIter, Self::ItemIter> {
        Enumerate {
            index: 0..self.len(),
            item: self.iter(),
        }
    }
}

/// implement the [`SuperIndex`] trait for [`HashMap<K, V, S>`]
impl<'a, K: 'a, V: 'a, S> SuperIndex<'a, &'a K> for std::collections::HashMap<K, V, S>
where
    K: Eq + std::hash::Hash,
    S: std::hash::BuildHasher,
{
    type IndexIter = std::collections::hash_map::Keys<'a, K, V>;
    type ItemIter = std::collections::hash_map::Values<'a, K, V>;

    fn enumerate(&'a self) -> Enumerate<Self::IndexIter, Self::ItemIter> {
        Enumerate {
            index: self.keys(),
            item: self.values(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn enumerate_vec() {
        let v = vec![10, 20, 30, 40];

        // I expect v.enumerate() to behave exactly like v.iter().enumerate()
        let mut e = v.enumerate();
        assert_eq!(e.next(), Some((0, &10)));
        assert_eq!(e.next(), Some((1, &20)));
        assert_eq!(e.next(), Some((2, &30)));
        assert_eq!(e.next(), Some((3, &40)));
        assert_eq!(e.next(), None);
    }

    #[test]
    fn enumerate_hashmap() {
        let mut capitals = HashMap::new();
        capitals.insert("Italy", "Rome");
        capitals.insert("France", "Paris");
        capitals.insert("Germany", "Mallorca");

        // I expect capitals.enumerate() to behave exactly like capitals.into_iter()
        let mut count = 0;
        for (index, value) in capitals.enumerate() {
            count += 1;
            if index == &"Italy" {
                assert_eq!(value, &"Rome")
            }
            if index == &"France" {
                assert_eq!(value, &"Paris")
            }
            if index == &"Germany" {
                assert_eq!(value, &"Mallorca")
            }
        }
        assert_eq!(count, 3)
    }
}
