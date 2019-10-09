//! an examplatory graph module with minimal functionality

pub mod ops;

use crate::ops::SuperIndex;

use std::vec::Vec;
use std::hash::Hash;
use std::collections::HashMap;

/// A simple graph implementation for any collection that implements [`SuperIndex`] and [`IntoIterator`].
pub trait SimpleGraph<'a, I : Eq + Hash + Copy>
{
    type C : 'a + SuperIndex<'a, I> + IntoIterator;

    /// gets a reference to the node container
    fn nodes(&'a self) -> &Self::C;

    /// gets the indices of the children of a node with index `index`.
    fn children(&'a self, index: I) -> Vec<I>;

    /// gets all ancestors of a node
    /// 
    /// This function is slow without any additional assumptions on the generic type and memory management. 
    /// Usually, this function can be implemented more efficiently and it is recommended
    /// to explicitly implement it.
    //TODO can this be cached??
    fn ancestors(&'a self, i: I) -> Vec<I> {
        let mut res = Vec::<I>::new();
        for (idx, _) in self.nodes().enumerate() {
            for child_idx in self.children(idx) {
                if child_idx == i {
                    res.push(idx);
                }
            }
        }
        return res;
    }

    /// get topologically sorted vector of indices for a graph
    fn get_topological_order(&'a self) -> Result<Vec<I>, &'static str> {

        // This is an implementation of Kahn's method

        // calculate in-degree
        let mut in_degree = HashMap::<I, usize>::new(); // using a HashMap for usize keys maybe not ideal
        for (i,_) in self.nodes().enumerate() {
            in_degree.insert(i, self.ancestors(i).len());
        }
        let n = in_degree.len();

        // initialize candidate list and return list
        let mut candidates = Vec::<I>::new();
        let mut list = Vec::<I>::new();

        // candidates are all nodes with zero in-degree
        for (i,_) in self.nodes().enumerate() {
            if in_degree[&i] == 0 {
                candidates.push(i);
            }
        }

        while candidates.len() > 0 {
            // greedyly push candidates `current` to the list and add children as new candidates, if their in_degree
            // reduces to zero by removing `current`
            let current = candidates.pop().unwrap();
            list.push(current);

            for child in self.children(current) {
                if let Some(x) = in_degree.get_mut(&child) {
                    *x -= 1;
                    if *x == 0 {
                        candidates.push(child);
                    }
                }
            }
        }
        if list.len() == n {
            return Ok(list);
        }
        else {
            return Err("Circular dependency detected!")
        }
    }

}

/// a node for any data with adjacency information
pub struct Node<I,D> {
    data: D,
    adj: Vec<I>
}

/// a graph that stores its nodes as a [`Vec`]
pub struct VGraph<D>(Vec<Node<usize,D>>);

impl<'a, D : 'a> SimpleGraph<'a,usize> for VGraph<D> {

    type C = Vec<Node<usize,D>>;

    fn nodes(&self) -> &Self::C {
        &self.0
    }

    fn children(&self, i : usize) -> Vec<usize> {
        self.0[i].adj.clone() // this is ugly
    }
}


#[cfg(test)]
mod tests { 

    use super::*;

    // create circular graph by directly implementing SimpleGraph on `Vec<usize>`
    struct UnidirectionalCircle(Vec<usize>);

    impl SimpleGraph<'_, usize> for UnidirectionalCircle {

        type C = Vec<usize>;
        
        fn nodes(&self) -> &Self::C {
            &self.0
        }

        fn children(&self, i: usize) -> Vec::<usize> {
            vec![(i+1) % self.0.len()]
        }
    }

    #[test]
    fn trait_uni_circle_children_ok() {
        let circle = UnidirectionalCircle(vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(circle.children(0)[0], 1);
        assert_eq!(circle.children(1)[0], 2);
        assert_eq!(circle.children(2)[0], 3);
        assert_eq!(circle.children(3)[0], 4);
        assert_eq!(circle.children(4)[0], 5);
        assert_eq!(circle.children(5)[0], 0);
    }

    #[test]
    fn trait_uni_circle_ancestors_ok() {
        let circle = UnidirectionalCircle(vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(circle.ancestors(0)[0], 5);
        assert_eq!(circle.ancestors(1)[0], 0);
        assert_eq!(circle.ancestors(2)[0], 1);
        assert_eq!(circle.ancestors(3)[0], 2);
        assert_eq!(circle.ancestors(4)[0], 3);
        assert_eq!(circle.ancestors(5)[0], 4);
    }

    // create circular graph by directly implementing SimpleGraph on `Vec<i32>`
    struct BidirectionalCircle(Vec<usize>);

    impl SimpleGraph<'_, usize> for BidirectionalCircle {

        type C = Vec<usize>;
        
        fn nodes(&self) -> &Self::C {
            &self.0
        }

        fn children(&self, i: usize) -> Vec::<usize> {
            let n = self.0.len();
            vec![(i + n - 1) % n, (i+1) % n]
        }
    }

    #[test]
    fn trait_bi_circle_children_ok() {
        let circle = BidirectionalCircle(vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(circle.children(0), vec![5, 1]);
        assert_eq!(circle.children(1), vec![0, 2]);
        assert_eq!(circle.children(2), vec![1, 3]);
        assert_eq!(circle.children(3), vec![2, 4]);
        assert_eq!(circle.children(4), vec![3, 5]);
        assert_eq!(circle.children(5), vec![4, 0]);

    }

    #[test]
    fn trait_bi_circle_ancestors_ok() {
        let circle = BidirectionalCircle(vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(circle.ancestors(0), vec![1, 5]);
        assert_eq!(circle.ancestors(1), vec![0, 2]);
        assert_eq!(circle.ancestors(2), vec![1, 3]);
        assert_eq!(circle.ancestors(3), vec![2, 4]);
        assert_eq!(circle.ancestors(4), vec![3, 5]);
        assert_eq!(circle.ancestors(5), vec![0, 4]);

    }

}
