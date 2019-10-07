pub mod ops;
/*
impl<K, Q: ?Sized, V, S> SuperIndex<&'_ Q> for std::collections::HashMap<K, V, S> where
    K: Eq + std::hash::Hash + std::borrow::Borrow<Q>,
    Q: Eq + std::hash::Hash,
    S: std::hash::BuildHasher
{
    type IndexIter = std::collections::hash_map::Keys<'_, K, V>;
    type ItemIter = std::collections::hash_map::Values<K,V>;

    fn enumerate(&self) -> Enumerate<<Self::IndexIter as IntoIterator>::IntoIter, 
                                     <Self::ItemIter as IntoIterator>::IntoIter>
    {
        Enumerate{ index: self.keys(), item: self.values() }
    }
}
*/


/// A simple graph implementation for any collection that implements [`std::ops::Index`] and [`std::iter::IntoIterator`].
/// This is implemented mainly with [`std::vec::Vec`] and [`std::hash::HashMap`].
pub trait SimpleGraph {

    /// type used as an index to refer to nodes. For a [`std::vec::Vec`] this would be a [`usize`], for a [`std::hash::HashMap`]
    /// the type of the keys.
    type I : Eq + std::hash::Hash;

    /// container type. The elements of the containers are the nodes of the graph
    type C : std::ops::Index<Self::I> + std::iter::IntoIterator<Item=Self::I>;

    /// returns a reference to the node container.
    fn nodes(&self) -> &Self::C;

    /// returns an iterator over the indices
    //fn indices<T: std::iter::Iterator>(&self) -> T;

    /// gets the indices of the children of a node with index `index`.
    fn children(&self, index: Self::I) -> Vec::<Self::I>;

/*
    /// gets all ancestors of a node
    /// 
    /// This function is slow without any additional assumptions on the generic type and memory management. 
    /// Usually, this function can be implemented more efficiently and it is recommended
    /// to explicitly implement it.
    //TODO can this be cached??
    fn ancestors(&self, i: Self::I) -> Vec::<Self::I> {
        let mut res = Vec::<Self::I>::new();
        let nodes = self.nodes();
        for (idx, _) in nodes.enumerate() {

            let children = self.children(idx);
            for child_idx in children {
                if child_idx == i {
                    res.push(idx);
                }
            }
        }
        return res;
    }
*/
}

/*
#[cfg(test)]
mod tests { 

    use super::*;

    struct UnidirectionalCircle<T>(std::vec::Vec::<T>);
    struct BidirectionalCircle<T>(std::vec::Vec::<T>);

    /// create a circular graph of a vector
    impl<T> SimpleGraph for UnidirectionalCircle<T> {
        
        type I = usize;
        type C = Vec::<T>;

        fn nodes(&self) -> &std::vec::Vec::<T> {
            &self.0
        }

        fn children(&self, i: Self::I) -> std::vec::Vec::<Self::I> {
            vec![(i+1) % self.0.len()]
        }
    }

    /// create a circular graph of a vector
    impl<T> SimpleGraph for BidirectionalCircle<T> {
        
        type I = usize;
        type C = Vec::<T>;

        fn nodes(&self) -> &std::vec::Vec::<T> {
            &self.0
        }

        fn children(&self, i: Self::I) -> std::vec::Vec::<Self::I> {
            vec![(i-1) % self.0.len(), (i+1) % self.0.len()]
        }
    }

    #[test]
    fn uni_circle_children_ok() {
        let circle = UnidirectionalCircle(vec![0, 1, 2, 3, 4, 5] );
        assert_eq!(circle.children(0)[0], 1);
        assert_eq!(circle.children(1)[0], 2);
        assert_eq!(circle.children(2)[0], 3);
        assert_eq!(circle.children(3)[0], 4);
        assert_eq!(circle.children(4)[0], 5);
    }

    #[test]
    fn bi_circle_children_ok() {
        let circle = UnidirectionalCircle(vec![0, 1, 2, 3, 4, 5] );
        assert_eq!(circle.children(0), vec![5, 1]);
        assert_eq!(circle.children(1), vec![0, 2]);
        assert_eq!(circle.children(2), vec![1, 3]);
        assert_eq!(circle.children(3), vec![2, 4]);
        assert_eq!(circle.children(4), vec![3, 5]);
    }

}
*/