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