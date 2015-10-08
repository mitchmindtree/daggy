//! 
//! **daggy** is a directed acyclic graph data structure library.
//!
//! The most prominent type is [**Dag**](./struct.Dag.html) - a wrapper around [petgraph]
//! (http://bluss.github.io/petulant-avenger-graphlibrary/doc/petgraph/index.html)'s [**Graph**]
//! (http://bluss.github.io/petulant-avenger-graphlibrary/doc/petgraph/graph/struct.Graph.html)
//! data structure, exposing a refined API targeted towards directed acyclic graph related
//! functionality.
//!

#![forbid(unsafe_code)]
#![warn(missing_docs)]


extern crate petgraph as pg;


pub use pg as petgraph;
pub use pg::graph::{EdgeIndex, NodeIndex};
use pg::graph::{DefIndex, GraphIndex, IndexType};
use std::ops::{Index, IndexMut};


/// The Petgraph to be used internally within the Dag for storing/managing nodes and edges.
pub type PetGraph<N, E, Ix> = pg::Graph<N, E, pg::Directed, Ix>;


/// A Directed acyclic graph (DAG) data structure.
///
/// Dag is a thin wrapper around petgraph's `Graph` data structure, providing a refined API for
/// dealing specifically with DAGs.
///
/// Note: The following documentation is adapted from petgraph's [**Graph** documentation]
/// (http://bluss.github.io/petulant-avenger-graphlibrary/doc/petgraph/graph/struct.Graph.html).
///
/// **Dag** is parameterized over the node weight **N**, edge weight **E** and index type **Ix**.
///
/// **NodeIndex** is a type that acts as a reference to nodes, but these are only stable across
/// certain operations. **Removing nodes may shift other indices.** Adding kids to the **Dag**
/// keeps all indices stable, but removing a node will force the last node to shift its index to
/// take its place.
///
/// The fact that the node indices in the **Dag** are numbered in a compact interval from 0 to *n*-1
/// simplifies some graph algorithms.
///
/// The **Ix** parameter is u32 by default. The goal is that you can ignore this parameter
/// completely unless you need a very large **Dag** -- then you can use usize.
///
/// The **Dag** also offers methods for accessing the underlying **Graph**, which can be useful
/// for taking advantage of petgraph's various graph-related algorithms.
#[derive(Clone, Debug)]
pub struct Dag<N, E, Ix: IndexType = DefIndex> {
    graph: PetGraph<N, E, Ix>,
}


/// An iterator yielding indices to the children of some node.
pub type Children<'a, E, Ix> = pg::graph::Neighbors<'a, E, Ix>;

/// A "walker" object that can be used to step through the children of some parent node.
pub struct WalkChildren<Ix: IndexType> {
    walk_edges: pg::graph::WalkEdges<Ix>,
}


/// An iterator yielding indices to the parents of some node.
pub type Parents<'a, E, Ix> = pg::graph::Neighbors<'a, E, Ix>;

/// A "walker" object that can be used to step through the children of some parent node.
pub struct WalkParents<Ix: IndexType> {
    walk_edges: pg::graph::WalkEdges<Ix>,
}


/// An error returned by the `Dag::add_edge` method in the case that adding an edge would have
/// caused the graph to cycle.
#[derive(Copy, Clone, Debug)]
pub struct WouldCycle<E>(pub E);


impl<N, E, Ix = DefIndex> Dag<N, E, Ix> where Ix: IndexType {

    /// Create a new, empty `Dag`.
    pub fn new() -> Self {
        Self::with_capacity(1, 1)
    }

    /// Create a new `Dag` with estimated capacity for its node and edge Vecs.
    pub fn with_capacity(nodes: usize, edges: usize) -> Self {
        Dag { graph: PetGraph::with_capacity(nodes, edges) }
    }

    /// The total number of nodes in the Dag.
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// The total number of edgees in the Dag.
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// Borrow the `Dag`'s underlying `PetGraph<N, Ix>`.
    /// All existing indices may be used to index into this `PetGraph` the same way they may be
    /// used to index into the `Dag`.
    pub fn graph(&self) -> &PetGraph<N, E, Ix> {
        &self.graph
    }

    /// Take ownership of the `Dag` and return the internal `PetGraph`.
    /// All existing indices may be used to index into this `PetGraph` the same way they may be
    /// used to index into the `Dag`.
    pub fn into_graph(self) -> PetGraph<N, E, Ix> {
        let Dag { graph } = self;
        graph
    }

    /// Add a new node to the `Dag` with the given weight.
    ///
    /// Computes in **O(1)** time.
    ///
    /// Returns the index of the new node.
    ///
    /// **Note:** If you're adding a new node and immediately adding a single edge to that node from
    /// some other node, consider using the [add_child](./struct.Dag.html#method.add_child) or
    /// [add_parent](./struct.Dag.html#method.add_parent) methods instead for better performance.
    ///
    /// **Panics** if the Graph is at the maximum number of nodes for its index type.
    pub fn add_node(&mut self, weight: N) -> NodeIndex<Ix> {
        self.graph.add_node(weight)
    }

    /// Add a new directed edge to the `Dag` with the given weight.
    ///
    /// The added edge will be in the direction `a` -> `b`
    ///
    /// Checks if the edge would create a cycle in the Graph.
    ///
    /// If adding the edge **would not** cause the graph to cycle, the edge will be added and its
    /// `EdgeIndex` returned.
    ///
    /// If adding the edge **would** cause the graph to cycle, the edge will not be added and
    /// instead a `WouldCycle<E>` error with the given weight will be returned.
    ///
    /// Computes in **O(t)** time where "t" is the time taken to check if adding the edge would
    /// cause a cycle in the graph. See petgraph's [`is_cyclic_directed`]
    /// (http://bluss.github.io/petulant-avenger-graphlibrary/doc/petgraph/algo/fn.is_cyclic_directed.html)
    /// function for more details.
    ///
    /// **Note:** Dag allows adding parallel ("duplicate") edges. If you want to avoid this, use
    /// [`update_edge`](./struct.Dag.html#method.update_edge) instead.
    ///
    /// **Note:** If you're adding a new node and immediately adding a single edge to that node from
    /// some other node, consider using the [add_child](./struct.Dag.html#method.add_child) or
    /// [add_parent](./struct.Dag.html#method.add_parent) methods instead for better performance.
    ///
    /// **Panics if the Graph is at the maximum number of nodes for its index type.
    pub fn add_edge(&mut self, a: NodeIndex<Ix>, b: NodeIndex<Ix>, weight: E)
        -> Result<EdgeIndex<Ix>, WouldCycle<E>>
    {
        let idx = self.graph.add_edge(a, b, weight);

        // Check if adding the edge has created a cycle.
        // TODO: Once petgraph adds support for re-using visit stack/maps, use that so that we
        // don't have to re-allocate every time `add_edge` is called.
        if pg::algo::is_cyclic_directed(&self.graph) {
            let weight = self.graph.remove_edge(idx).expect("No edge for index");
            Err(WouldCycle(weight))
        } else {
            Ok(idx)
        }
    }

    /// Update the edge from nodes `a` -> `b` with the given weight.
    ///
    /// If the edge doesn't already exist, it will be added using the `add_edge` method.
    ///
    /// Please read the [`add_edge`](./struct.Dag.html#method.add_edge) for more important details.
    ///
    /// Checks if the edge would create a cycle in the Graph.
    ///
    /// Computes in **O(t + e)** time where "t" is the complexity of `add_edge` and e is the number
    /// of edges connected to the nodes a and b.
    ///
    /// Returns the index of the edge, or a `WouldCycle` error if adding the edge would create a
    /// cycle.
    ///
    /// **Note:** If you're adding a new node and immediately adding a single edge to that node from
    /// some parent node, consider using the [`add_child`](./struct.Dag.html#method.add_child)
    /// method instead for better performance.
    ///
    /// **Panics** if the Graph is at the maximum number of nodes for its index type.
    pub fn update_edge(&mut self, a: NodeIndex<Ix>, b: NodeIndex<Ix>, weight: E)
        -> Result<EdgeIndex<Ix>, WouldCycle<E>>
    {
        if let Some(edge_idx) = self.graph.find_edge(a, b) {
            if let Some(edge) = self.graph.edge_weight_mut(edge_idx) {
                *edge = weight;
                return Ok(edge_idx);
            }
        }
        self.add_edge(a, b, weight)
    }

    /// Add a new edge and parent node to the node at the given `NodeIndex`.
    /// Returns both the edge's `EdgeIndex` and the node's `NodeIndex`.
    ///
    /// node -> edge -> child
    ///
    /// Computes in **O(1)** time.
    ///
    /// This is faster than using `add_node` and `add_edge`. This is because we don't have to check
    /// if the graph would cycle when adding an edge to the new node, as we know it it will be the
    /// only edge connected to that node.
    ///
    /// **Panics** if the given child node doesn't exist.
    ///
    /// **Panics** if the Graph is at the maximum number of edges for its index.
    pub fn add_parent(&mut self, child: NodeIndex<Ix>, edge: E, node: N)
        -> (EdgeIndex<Ix>, NodeIndex<Ix>)
    {
        let parent_node = self.graph.add_node(node);
        let parent_edge = self.graph.add_edge(parent_node, child, edge);
        (parent_edge, parent_node)
    }

    /// Add a new edge and child node to the node at the given `NodeIndex`.
    /// Returns both the edge's `EdgeIndex` and the node's `NodeIndex`.
    ///
    /// child -> edge -> node
    ///
    /// Computes in **O(1)** time.
    ///
    /// This is faster than using `add_node` and `add_edge`. This is because we don't have to check
    /// if the graph would cycle when adding an edge to the new node, as we know it it will be the
    /// only edge connected to that node.
    ///
    /// **Panics** if the given parent node doesn't exist.
    ///
    /// **Panics** if the Graph is at the maximum number of edges for its index.
    pub fn add_child(&mut self, parent: NodeIndex<Ix>, edge: E, node: N)
        -> (EdgeIndex<Ix>, NodeIndex<Ix>)
    {
        let child_node = self.graph.add_node(node);
        let child_edge = self.graph.add_edge(parent, child_node, edge);
        (child_edge, child_node)
    }

    /// Borrow the weight from the node at the given index.
    pub fn node_weight(&self, node: NodeIndex<Ix>) -> Option<&N> {
        self.graph.node_weight(node)
    }

    /// Mutably borrow the weight from the node at the given index.
    pub fn node_weight_mut(&mut self, node: NodeIndex<Ix>) -> Option<&mut N> {
        self.graph.node_weight_mut(node)
    }

    /// Borrow the weight from the edge at the given index.
    pub fn edge_weight(&self, edge: EdgeIndex<Ix>) -> Option<&E> {
        self.graph.edge_weight(edge)
    }

    /// Mutably borrow the weight from the edge at the given index.
    pub fn edge_weight_mut(&mut self, edge: EdgeIndex<Ix>) -> Option<&mut E> {
        self.graph.edge_weight_mut(edge)
    }

    /// Index the `Dag` by two indices.
    /// 
    /// Both indices can be either `NodeIndex`s, `EdgeIndex`s or a combination of the two.
    ///
    /// **Panics** if the indices are equal or if they are out of bounds.
    pub fn index_twice_mut<A, B>(&mut self, a: A, b: B)
        -> (&mut <PetGraph<N, E, Ix> as Index<A>>::Output,
            &mut <PetGraph<N, E, Ix> as Index<B>>::Output) where
        PetGraph<N, E, Ix>: IndexMut<A> + IndexMut<B>,
        A: GraphIndex,
        B: GraphIndex,
    {
        self.graph.index_twice_mut(a, b)
    }

    /// Remove the node at the given index from the `Dag` and return it if it exists.
    ///
    /// Note: Calling this may shift (and in turn invalidate) other previously returned node
    /// indices!
    pub fn remove_node(&mut self, node: NodeIndex<Ix>) -> Option<N> {
        self.graph.remove_node(node)
    }

    /// Remove an edge and return its weight, or `None` if it didn't exist.
    /// 
    /// Computes in **O(e')** time, where **e'** is the size of four particular edge lists, for the
    /// nodes of **e** and the nodes of another affected edge.
    pub fn remove_edge(&mut self, e: EdgeIndex<Ix>) -> Option<E> {
        self.graph.remove_edge(e)
    }

    /// An iterator over all nodes that are parents to the node at the given index.
    ///
    /// The returned iterator yields `EdgeIndex<Ix>`s.
    pub fn parents(&self, child: NodeIndex<Ix>) -> Parents<E, Ix> {
        self.graph.neighbors_directed(child, pg::Incoming)
    }

    /// A "walker" object that may be used to step through the parents of the given child node.
    ///
    /// Unlike the `Parents` type, `WalkParents` does not borrow the `Dag`'s `PetGraph`.
    pub fn walk_parents(&self, child: NodeIndex<Ix>) -> WalkParents<Ix> {
        let walk_edges = self.graph.walk_edges_directed(child, pg::Incoming);
        WalkParents { walk_edges: walk_edges }
    }

    /// An iterator over all nodes that are children to the node at the given index.
    ///
    /// The returned iterator yields `EdgeIndex<Ix>`s.
    pub fn children(&self, parent: NodeIndex<Ix>) -> Children<E, Ix> {
        self.graph.neighbors_directed(parent, pg::Outgoing)
    }

    /// A "walker" object that may be used to step through the children of the given parent node.
    ///
    /// Unlike the `Children` type, `WalkChildren` does not borrow the `Dag`'s `PetGraph`.
    pub fn walk_children(&self, parent: NodeIndex<Ix>) -> WalkChildren<Ix> {
        let walk_edges = self.graph.walk_edges_directed(parent, pg::Outgoing);
        WalkChildren { walk_edges: walk_edges }
    }

}


impl<N, E, Ix> Index<NodeIndex<Ix>> for Dag<N, E, Ix> where Ix: IndexType {
    type Output = N;
    fn index(&self, index: NodeIndex<Ix>) -> &N {
        &self.graph[index]
    }
}

impl<N, E, Ix> IndexMut<NodeIndex<Ix>> for Dag<N, E, Ix> where Ix: IndexType {
    fn index_mut(&mut self, index: NodeIndex<Ix>) -> &mut N {
        &mut self.graph[index]
    }
}

impl<N, E, Ix> Index<EdgeIndex<Ix>> for Dag<N, E, Ix> where Ix: IndexType {
    type Output = E;
    fn index(&self, index: EdgeIndex<Ix>) -> &E {
        &self.graph[index]
    }
}

impl<N, E, Ix> IndexMut<EdgeIndex<Ix>> for Dag<N, E, Ix> where Ix: IndexType {
    fn index_mut(&mut self, index: EdgeIndex<Ix>) -> &mut E {
        &mut self.graph[index]
    }
}


impl<Ix> WalkChildren<Ix> where Ix: IndexType {

    /// Fetch the next child edge index in the walk for the given `Dag`.
    pub fn next<N, E>(&mut self, dag: &Dag<N, E, Ix>) -> Option<EdgeIndex<Ix>> {
        self.walk_edges.next(&dag.graph)
    }

    /// Fetch the `EdgeIndex` and `NodeIndex` to the next child in the walk for the given `Dag`.
    pub fn next_child<N, E>(&mut self, dag: &Dag<N, E, Ix>)
        -> Option<(EdgeIndex<Ix>, NodeIndex<Ix>)>
    {
        self.walk_edges.next_neighbor(&dag.graph)
    }

}

impl<Ix> WalkParents<Ix> where Ix: IndexType {

    /// Fetch the next parent edge index in the walk for the given `Dag`.
    pub fn next<N, E>(&mut self, dag: &Dag<N, E, Ix>) -> Option<EdgeIndex<Ix>> {
        self.walk_edges.next(&dag.graph)
    }

    /// Fetch the `EdgeIndex` and `NodeIndex` to the next parent in the walk for the given `Dag`.
    pub fn next_parent<N, E>(&mut self, dag: &Dag<N, E, Ix>)
        -> Option<(EdgeIndex<Ix>, NodeIndex<Ix>)>
    {
        self.walk_edges.next_neighbor(&dag.graph)
    }

}

