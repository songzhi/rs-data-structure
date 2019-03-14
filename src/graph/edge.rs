use super::graph::{Directed, Undirected};
use crate::graph::node::NodeTrait;
use indexmap::IndexMap;
use crate::graph::traverse::Neighbors;
use crate::graph::graph::Graph;
use std::marker::PhantomData;
use indexmap::map::Iter as IndexMapIter;

/// A graph's edge type determines whether is has directed edges or not.
pub trait EdgeType {
    fn is_directed() -> bool;
}

impl EdgeType for Directed {
    #[inline]
    fn is_directed() -> bool {
        true
    }
}

impl EdgeType for Undirected {
    #[inline]
    fn is_directed() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Outgoing,
    Incoming,
}

impl Direction {
    #[inline]
    pub fn opposite(self) -> Direction {
        match self {
            Direction::Outgoing => Direction::Incoming,
            Direction::Incoming => Direction::Outgoing,
        }
    }

    /// Return `0` for `Outgoing` and `1` for `Incoming`.
    #[inline]
    pub fn index(self) -> usize {
        match self {
            Direction::Outgoing => 0,
            Direction::Incoming => 1,
        }
    }
}


pub struct Edges<'a, N, E: 'a, Ty>
    where
        N: 'a + NodeTrait,
        Ty: EdgeType,
{
    from: N,
    edges: &'a IndexMap<(N, N), E>,
    iter: Neighbors<'a, N, Ty>,
}

impl<'a, N, E, Ty> Edges<'a, N, E, Ty>
    where
        N: 'a + NodeTrait,
        Ty: EdgeType,
{
    pub fn new(from: N, edges: &'a IndexMap<(N, N), E>, iter: Neighbors<'a, N, Ty>) -> Self {
        Self { from, edges, iter }
    }
}

impl<'a, N, E, Ty> Iterator for Edges<'a, N, E, Ty>
    where
        N: 'a + NodeTrait,
        E: 'a,
        Ty: EdgeType,
{
    type Item = (N, N, &'a E);
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(b) => {
                let a = self.from;
                match self.edges.get(&Graph::<N, E, Ty>::edge_key(a, b)) {
                    None => unreachable!(),
                    Some(edge) => Some((a, b, edge)),
                }
            }
        }
    }
}

pub struct AllEdges<'a, N, E: 'a, Ty> {
    inner: IndexMapIter<'a, (N, N), E>,
    ty: PhantomData<Ty>,
}

impl<'a, N, E, Ty> AllEdges<'a, N, E, Ty>
    where
        N: 'a + NodeTrait,
{
    pub fn new(inner: IndexMapIter<'a, (N, N), E>, ty: PhantomData<Ty>) -> Self {
        Self { inner, ty }
    }
}

impl<'a, N, E, Ty> Iterator for AllEdges<'a, N, E, Ty>
    where
        N: 'a + NodeTrait,
        E: 'a,
        Ty: EdgeType,
{
    type Item = (N, N, &'a E);

    /// Advances the iterator and returns the next value.
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            None => None,
            Some((&(a, b), v)) => Some((a, b, v)),
        }
    }

    /// Returns the bounds on the remaining length of the iterator.
    ///
    /// Specifically, `size_hint()` returns a tuple where the first element
    /// is the lower bound, and the second element is the upper bound.
    ///
    /// The second half of the tuple that is returned is an [`Option`]`<`[`usize`]`>`.
    /// A [`None`] here means that either there is no known upper bound, or the
    /// upper bound is larger than [`usize`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use indexmap::IndexMap;
    /// use safe_graph::edge::AllEdges;
    /// use safe_graph::graph::Directed;
    /// use std::marker::PhantomData;
    ///
    /// let edges = IndexMap::new();
    /// let all_edges: AllEdges<u32, f32, Directed> = AllEdges::new(edges.iter(), PhantomData);
    ///
    /// assert_eq!(all_edges.size_hint(), (0, Some(0)));
    /// ```
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    /// Consumes the iterator, counting the number of iterations and returning it.
    fn count(self) -> usize {
        self.inner.count()
    }

    /// Returns the `n`th element of the iterator.
    ///
    /// Like most indexing operations, the count starts from zero, so `nth(0)`
    /// returns the first value, `nth(1)` the second, and so on.
    ///
    /// Note that all preceding elements, as well as the returned element, will be
    /// consumed from the iterator. That means that the preceding elements will be
    /// discarded, and also that calling `nth(0)` multiple times on the same iterator
    /// will return different elements.
    ///
    /// `nth()` will return [`None`] if `n` is greater than or equal to the length of the
    /// iterator.
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.inner
            .nth(n)
            .map(|(&(n1, n2), weight)| (n1, n2, weight))
    }

    /// Consumes the iterator, returning the last element.
    fn last(self) -> Option<Self::Item> {
        self.inner
            .last()
            .map(|(&(n1, n2), weight)| (n1, n2, weight))
    }
}

impl<'a, N, E, Ty> DoubleEndedIterator for AllEdges<'a, N, E, Ty>
    where
        N: 'a + NodeTrait,
        E: 'a,
        Ty: EdgeType,
{
    /// Removes and returns an element from the end of the iterator.
    ///
    /// Returns `None` when there are no more elements.
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner
            .next_back()
            .map(|(&(n1, n2), weight)| (n1, n2, weight))
    }
}

/// Convert an element like `(i, j)` or `(i, j, w)` into a triple of source, target, edge weight.
///
/// For `Graph::from_edges`.
pub trait IntoWeightedEdge<E> {
    type NodeId;
    fn into_weighted_edge(self) -> (Self::NodeId, Self::NodeId, E);
}

/// Convert an element like `(i, j)` into a triple of source, target, edge weight.
impl<Ix, E> IntoWeightedEdge<E> for (Ix, Ix)
    where
        E: Default,
{
    type NodeId = Ix;

    fn into_weighted_edge(self) -> (Ix, Ix, E) {
        let (s, t) = self;
        (s, t, E::default())
    }
}

/// Convert an element like `(i, j, w)` into a triple of source, target, edge weight.
///
/// Meaning do no change, just return.
impl<Ix, E> IntoWeightedEdge<E> for (Ix, Ix, E) {
    type NodeId = Ix;

    fn into_weighted_edge(self) -> (Ix, Ix, E) {
        self
    }
}

/// Convert an element like `(i, j, w)` into a triple of source, target, edge weight.
///
/// Clone the edge weight from the reference.
impl<'a, Ix, E> IntoWeightedEdge<E> for (Ix, Ix, &'a E)
    where
        E: Clone,
{
    type NodeId = Ix;

    fn into_weighted_edge(self) -> (Ix, Ix, E) {
        let (a, b, c) = self;
        (a, b, c.clone())
    }
}

/// Convert an element like `&(i, j)` into a triple of source, target, edge weight.
///
/// See that the element `&(i, j)` is a reference.
impl<'a, Ix, E> IntoWeightedEdge<E> for &'a (Ix, Ix)
    where
        Ix: Copy,
        E: Default,
{
    type NodeId = Ix;

    fn into_weighted_edge(self) -> (Ix, Ix, E) {
        let (s, t) = *self;
        (s, t, E::default())
    }
}

/// Convert an element like `&(i, j, w)` into a triple of source, target, edge weight.
///
/// Clone the edge weight from the reference.
/// See that the element `&(i, j, w)` is a reference.
impl<'a, Ix, E> IntoWeightedEdge<E> for &'a (Ix, Ix, E)
    where
        Ix: Copy,
        E: Clone,
{
    type NodeId = Ix;
    fn into_weighted_edge(self) -> (Ix, Ix, E) {
        self.clone()
    }
}