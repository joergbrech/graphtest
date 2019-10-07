// Modified from the accepted answer in:
// https://stackoverflow.com/questions/30630810/using-generic-iterators-instead-of-specific-list-types


// An Enumerate struct that has two iterators, one for the index and one for the associated type
struct Enumerate<IndexIter, ItemIter> {
    index: IndexIter,
    item: ItemIter,
}

// Implement iterator trait for the new struct
impl<IndexIter, ItemIter> Iterator for Enumerate<IndexIter, ItemIter> 
where IndexIter: Iterator,
      ItemIter: Iterator
{
    type Item = (IndexIter::Item, ItemIter::Item);

    #[inline]
    fn next(&mut self) -> Option<(IndexIter::Item, ItemIter::Item)> {
        self.index.next().map(|idx| {
            // CAUTION! We need to make sure that the index and item iterators are ordered consistently.
            // We are really just incrementing to iterators simultaneously here...
            (idx, self.item.next().unwrap())
        })
    }
}

// Add an enumerate function that returns the new iterator for a collection that implements std::ops::Index
trait SuperIndex<'a, Idx> : std::ops::Index<Idx> {

    type IndexIter : IntoIterator;
    type ItemIter : IntoIterator;

    fn enumerate(&'a self) -> Enumerate<<Self::IndexIter as IntoIterator>::IntoIter, 
                                     <Self::ItemIter as IntoIterator>::IntoIter>;
}


// implement the SuperIndex trait for Vec
// impl<'a, T: 'a, I : std::slice::SliceIndex<[T]>> SuperIndex<'a,  I> for Vec<T> 
impl<'a, T: 'a> SuperIndex<'a,  usize> for Vec<T> 
{
    type IndexIter = std::ops::Range<usize>;
    type ItemIter = std::slice::Iter<'a, T>;

    fn enumerate(&'a self) -> Enumerate<<Self::IndexIter as IntoIterator>::IntoIter, 
                                     <Self::ItemIter as IntoIterator>::IntoIter>
    {
        Enumerate{ index: (0..self.len()).into_iter(), item: self.into_iter() }
    }
}

// implement SuperIndex for HashMap
impl<'a, K : 'a, Q: ?Sized, V : 'a, S> SuperIndex<'a, &'_ Q> for std::collections::HashMap<K, V, S> where
    K: Eq + std::hash::Hash + std::borrow::Borrow<Q>,
    Q: Eq + std::hash::Hash,
    S: std::hash::BuildHasher
{
    type IndexIter = std::collections::hash_map::Keys<'a, K, V>;
    type ItemIter = std::collections::hash_map::Values<'a, K,V>;

    fn enumerate(&'a self) -> Enumerate<<Self::IndexIter as IntoIterator>::IntoIter, 
                                     <Self::ItemIter as IntoIterator>::IntoIter>
    {
        Enumerate{ index: self.keys(), item: self.values() }
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
    }

/*
    #[test]
    fn enumerate_hashmap() {
        let mut capitols = HashMap::<String, String>::new();
        capitols.insert("Italy".to_string(), "Rome".to_string());
        capitols.insert("France".to_string(), "Paris".to_string());
        capitols.insert("Germany".to_string(), "Mallorca".to_string());

        let e = capitols.enumerate();
    }
*/

}