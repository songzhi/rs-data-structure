use indexmap::IndexMap;
use std::marker::PhantomData;

/// Marker type for a directed graph.
#[derive(Copy, Debug,Clone)]
pub enum Directed {}

/// Marker type for an undirected graph.
#[derive(Copy, Debug,Clone)]
pub enum Undirected {}



#[derive(Clone)]
pub struct Graph<V, E, Ty=Undirected> {
    vertices: IndexMap<V, Vec<(V)>>,
    edges: IndexMap<(V, V), E>,
    ty: PhantomData<Ty>,
}