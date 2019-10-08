//! an examplatory graph module with minimal functionality

pub mod ops;

use crate::ops::SuperIndex;

use std::vec::Vec;

/// A simple graph implementation for any collection that implements [`std::ops::Index`] and [`std::iter::IntoIterator`].
pub trait SimpleGraph<'a, I : Eq + Copy> : SuperIndex<'a, I> + IntoIterator
{
    /// gets the indices of the children of a node with index `index`.
    fn children(&'a self, index: I) -> Vec::<I>;

    /// gets all ancestors of a node
    /// 
    /// This function is slow without any additional assumptions on the generic type and memory management. 
    /// Usually, this function can be implemented more efficiently and it is recommended
    /// to explicitly implement it.
    //TODO can this be cached??
    fn ancestors(&'a self, i: I) -> Vec::<I> {
        let mut res = Vec::<I>::new();
        for (idx, _) in self.enumerate() {

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

    // create circular graph by directly implementing SimpleGraph on `Vec<usize>`
    type UnidirectionalCircle = Vec<usize>;

    impl<'a> SimpleGraph<'a, usize> for UnidirectionalCircle {

        fn children(&self, i: usize) -> Vec::<usize> {
            vec![(i+1) % self.len()]
        }
    }

    #[test]
    fn uni_circle_children_ok() {
        let circle : UnidirectionalCircle = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(circle.children(0)[0], 1);
        assert_eq!(circle.children(1)[0], 2);
        assert_eq!(circle.children(2)[0], 3);
        assert_eq!(circle.children(3)[0], 4);
        assert_eq!(circle.children(4)[0], 5);
        assert_eq!(circle.children(5)[0], 0);
    }

    #[test]
    fn uni_circle_ancestors_ok() {
        let circle : UnidirectionalCircle = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(circle.ancestors(0)[0], 5);
        assert_eq!(circle.ancestors(1)[0], 0);
        assert_eq!(circle.ancestors(2)[0], 1);
        assert_eq!(circle.ancestors(3)[0], 2);
        assert_eq!(circle.ancestors(4)[0], 3);
        assert_eq!(circle.ancestors(5)[0], 4);
    }

    // create circular graph by directly implementing SimpleGraph on `Vec<i32>`
    type BidirectionalCircle = Vec<i32>;

    impl<'a> SimpleGraph<'a, usize> for BidirectionalCircle {

        fn children(&self, i: usize) -> Vec::<usize> {
            let n = self.len();
            vec![(i + n - 1) % n, (i+1) % n]
        }
    }

    #[test]
    fn bi_circle_children_ok() {
        let circle : BidirectionalCircle = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(circle.children(0), vec![5, 1]);
        assert_eq!(circle.children(1), vec![0, 2]);
        assert_eq!(circle.children(2), vec![1, 3]);
        assert_eq!(circle.children(3), vec![2, 4]);
        assert_eq!(circle.children(4), vec![3, 5]);
        assert_eq!(circle.children(5), vec![4, 0]);

    }

    #[test]
    fn bi_circle_ancestors_ok() {
        let circle : BidirectionalCircle = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(circle.ancestors(0), vec![1, 5]);
        assert_eq!(circle.ancestors(1), vec![0, 2]);
        assert_eq!(circle.ancestors(2), vec![1, 3]);
        assert_eq!(circle.ancestors(3), vec![2, 4]);
        assert_eq!(circle.ancestors(4), vec![3, 5]);
        assert_eq!(circle.ancestors(5), vec![0, 4]);

    }

}
