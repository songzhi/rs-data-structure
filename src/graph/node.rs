use std::fmt::Debug;
use std::hash::Hash;
use std::iter::Cloned;
use indexmap::map::Keys;
use super::edge::Direction;

/// A trait group for `Graph`'s node identifier.
pub trait NodeTrait: Copy + Debug + Hash + Ord {}

/// Implement the `NodeTrait` for all types satisfying bounds.
impl<N> NodeTrait for N where N: Copy + Debug + Hash + Ord {}

/// Iterator over Nodes.
pub struct Nodes<'a, N: 'a + NodeTrait> {
    iter: Cloned<Keys<'a, N, Vec<(N, Direction)>>>,
}

impl<'a, N: 'a + NodeTrait> Nodes<'a, N> {
    pub fn new(iter: Cloned<Keys<'a, N, Vec<(N, Direction)>>>) -> Self {
        Self { iter }
    }
}

impl<'a, N: 'a + NodeTrait> Iterator for Nodes<'a, N> {
    type Item = N;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::edge::Direction as CompactDirection;
    use crate::graph::node::Nodes;
    use indexmap::IndexMap;

    #[test]
    fn new() {
        let nodes: IndexMap<i32, Vec<(i32, CompactDirection)>> = IndexMap::new();

        Nodes::new(nodes.keys().cloned());
    }

    #[test]
    fn next() {
        let mut nodes: IndexMap<i32, Vec<(i32, CompactDirection)>> = IndexMap::with_capacity(3);
        nodes.insert(1, vec![]);
        nodes.insert(2, vec![]);
        nodes.insert(3, vec![]);

        let mut nodes = Nodes::new(nodes.keys().cloned());

        // Test node `1`.
        assert_eq!(nodes.next(), Some(1));
        assert_eq!(nodes.next(), Some(2));
        assert_eq!(nodes.next(), Some(3));

        // Test the end of iteration.
        assert_eq!(nodes.next(), None);
    }

    #[test]
    fn size_hint() {
        let mut nodes: IndexMap<i32, Vec<(i32, CompactDirection)>> = IndexMap::with_capacity(3);
        nodes.insert(1, vec![]);
        nodes.insert(2, vec![]);
        nodes.insert(3, vec![]);

        let mut nodes = Nodes::new(nodes.keys().cloned());

        assert_eq!(nodes.size_hint(), (3, Some(3)));

        // Lower the length of the iterator.
        nodes.next();

        assert_eq!(nodes.size_hint(), (2, Some(2)));

        // Lower the length of the iterator.
        nodes.next();

        assert_eq!(nodes.size_hint(), (1, Some(1)));

        // Lower the length of the iterator.
        nodes.next();

        assert_eq!(nodes.size_hint(), (0, Some(0)));
    }
}