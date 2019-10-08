//! an examplatory graph module with minimal functionality

pub mod ops;
use crate::ops::SuperIndex;


/// A simple graph implementation for any collection that implements [`std::ops::Index`] and [`std::iter::IntoIterator`].
/// This is implemented mainly with [`std::vec::Vec`] and [`std::hash::HashMap`].
pub trait SimpleGraph<'a> 
{
    type I: Eq + Copy;
    
    type C: crate::ops::SuperIndex<'a, Self::I> + IntoIterator;

    /// returns a reference to the node container.
    fn nodes(&'a self) -> &Self::C;

    /// gets the indices of the children of a node with index `index`.
    fn children(&'a self, index: Self::I) -> Vec::<Self::I>;

    /// gets all ancestors of a node
    /// 
    /// This function is slow without any additional assumptions on the generic type and memory management. 
    /// Usually, this function can be implemented more efficiently and it is recommended
    /// to explicitly implement it.
    //TODO can this be cached??
    fn ancestors(&'a self, i: Self::I) -> Vec::<Self::I> {
        let mut res = Vec::<Self::I>::new();
        for (idx, _) in self.nodes().enumerate() {

            let children = self.children(idx);
            for child_idx in children {
                if child_idx == i {
                    res.push(idx);
                }
            }
        }
        return res;
    }

}


#[cfg(test)]
mod tests { 

    use super::*;

    // create circular DAGs using vectors of nodes
    struct UnidirectionalCircle<T>(std::vec::Vec::<T>);
    struct BidirectionalCircle<T>(std::vec::Vec::<T>);

    impl<'a, T: 'a> SimpleGraph<'a> for UnidirectionalCircle<T> {
        
        type I = usize;
        type C = Vec::<T>;

        fn nodes(&self) -> &std::vec::Vec::<T> {
            &self.0
        }

        fn children(&self, i: Self::I) -> std::vec::Vec::<Self::I> {
            vec![(i+1) % self.0.len()]
        }
    }

    impl<'a, T : 'a> SimpleGraph<'a> for BidirectionalCircle<T> {
        
        type I = usize;
        type C = Vec::<T>;

        fn nodes(&self) -> &std::vec::Vec::<T> {
            &self.0
        }

        fn children(&self, i: Self::I) -> std::vec::Vec::<Self::I> {
            let n = self.0.len();
            vec!((i + n - 1) % n, (i+1) % n) // avoid underflow with usize
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
        assert_eq!(circle.children(5)[0], 0);
    }

    #[test]
    fn uni_circle_ancestors_ok() {
        let circle = UnidirectionalCircle(vec![0, 1, 2, 3, 4, 5] );
        assert_eq!(circle.ancestors(0)[0], 5);
        assert_eq!(circle.ancestors(1)[0], 0);
        assert_eq!(circle.ancestors(2)[0], 1);
        assert_eq!(circle.ancestors(3)[0], 2);
        assert_eq!(circle.ancestors(4)[0], 3);
        assert_eq!(circle.ancestors(5)[0], 4);
    }

    #[test]
    fn bi_circle_children_ok() {
        let circle = BidirectionalCircle(vec![0, 1, 2, 3, 4, 5] );
        assert_eq!(circle.children(0), vec![5, 1]);
        assert_eq!(circle.children(1), vec![0, 2]);
        assert_eq!(circle.children(2), vec![1, 3]);
        assert_eq!(circle.children(3), vec![2, 4]);
        assert_eq!(circle.children(4), vec![3, 5]);
        assert_eq!(circle.children(5), vec![4, 0]);
    }

}
