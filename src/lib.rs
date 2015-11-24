//! **daggy** is a directed acyclic graph data structure library.
//!
//! The most prominent type is [**Dag**](./struct.Dag.html) - a wrapper around [petgraph]
//! (http://bluss.github.io/petulant-avenger-graphlibrary/doc/petgraph/index.html)'s [**Graph**]
//! (http://bluss.github.io/petulant-avenger-graphlibrary/doc/petgraph/graph/struct.Graph.html)
//! data structure, exposing a refined API targeted towards directed acyclic graph related
//! functionality.
//!
//! The [**Walker** trait](./walker/trait.Walker.html) defines a variety of useful methods for
//! traversing any graph type. Its methods behave similarly to iterator types, however **Walker**s
//! do not require borrowing the graph. This means that we can still safely mutably borrow from the
//! graph whilst we traverse it.

#![forbid(unsafe_code)]
#![warn(missing_docs)]


extern crate petgraph as pg;


pub use pg as petgraph;
pub use pg::graph::{EdgeIndex, NodeIndex, EdgeWeightsMut, NodeWeightsMut};
pub use walker::Walker;
use pg::graph::{DefIndex, GraphIndex, IndexType};
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};


pub mod walker;


/// The Petgraph to be used internally within the Dag for storing/managing nodes and edges.
pub type PetGraph<N, E, Ix> = pg::Graph<N, E, pg::Directed, Ix>;

/// Read only access into a **Dag**'s internal node array.
pub type RawNodes<'a, N, Ix> = &'a [pg::graph::Node<N, Ix>];
/// Read only access into a **Dag**'s internal edge array.
pub type RawEdges<'a, E, Ix> = &'a [pg::graph::Edge<E, Ix>];

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


/// A **Walker** type that can be used to step through the children of some parent node.
pub struct Children<N, E, Ix: IndexType> {
    walk_edges: pg::graph::WalkEdges<Ix>,
    _node: PhantomData<N>,
    _edge: PhantomData<E>,
}


/// A **Walker** type that can be used to step through the children of some parent node.
pub struct Parents<N, E, Ix: IndexType> {
    walk_edges: pg::graph::WalkEdges<Ix>,
    _node: PhantomData<N>,
    _edge: PhantomData<E>,
}

/// An iterator yielding multiple `EdgeIndex`s, returned by the `Graph::add_edges` method.
pub struct EdgeIndices<Ix: IndexType> {
    indices: ::std::ops::Range<usize>,
    _phantom: PhantomData<Ix>,
}

/// An alias to simplify the **Recursive** **Walker** type returned by **Dag**.
pub type RecursiveWalk<N, E, Ix, F> = walker::Recursive<Dag<N, E, Ix>, Ix, F>;


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

    /// Removes all nodes and edges from the **Dag**.
    pub fn clear(&mut self) {
        self.graph.clear();
    }

    /// The total number of nodes in the **Dag**.
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// The total number of edgees in the **Dag**.
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
    /// **Panics** if the Graph is at the maximum number of nodes for its index type.
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

    /// Adds the given directed edges to the `Dag`, each with their own given weight.
    ///
    /// The given iterator should yield a `NodeIndex` pair along with a weight for each Edge to be
    /// added in a tuple.
    ///
    /// If we were to describe the tuple as *(a, b, weight)*, the connection would be directed as
    /// follows:
    ///
    /// *a -> b*
    ///
    /// This method behaves similarly to the [`add_edge`](./struct.Dag.html#method.add_edge)
    /// method, however rather than checking whether or not a cycle has been created after adding
    /// each edge, it only checks after all edges have been added. This makes it a slightly more
    /// performant and ergonomic option that repeatedly calling `add_edge`.
    ///
    /// If adding the edges **would not** cause the graph to cycle, the edges will be added and
    /// their indices returned in an `EdgeIndices` iterator, yielding indices for each edge in the
    /// same order that they were given.
    ///
    /// If adding the edges **would** cause the graph to cycle, the edges will not be added and
    /// instead a `WouldCycle<Vec<E>>` error with the unused weights will be returned. The order of
    /// the returned `Vec` will be the reverse of the given order.
    ///
    /// **Note:** Dag allows adding parallel ("duplicate") edges. If you want to avoid this, use
    /// [`update_edges`](./struct.Dag.html#method.update_edges) instead.
    ///
    /// **Note:** If you're adding a series of new nodes and edges to a single node, consider using
    ///  the [add_child](./struct.Dag.html#method.add_child) or [add_parent]
    ///  (./struct.Dag.html#method.add_parent) methods instead for better performance. These
    ///  perform better as there is no need to check for cycles.
    ///
    /// **Panics** if the Graph is at the maximum number of nodes for its index type.
    pub fn add_edges<I>(&mut self, edges: I) -> Result<EdgeIndices<Ix>, WouldCycle<Vec<E>>> where
        I: ::std::iter::IntoIterator<Item=(NodeIndex<Ix>, NodeIndex<Ix>, E)>,
    {
        let mut num_edges = 0;
        for (a, b, weight) in edges {
            self.graph.add_edge(a, b, weight);
            num_edges += 1;
        }

        let total_edges = self.edge_count();
        let new_edges_range = total_edges-num_edges .. total_edges;

        // Check if adding the edges has created a cycle.
        // TODO: Once petgraph adds support for re-using visit stack/maps, use that so that we
        // don't have to re-allocate every time `add_edges` is called.
        if pg::algo::is_cyclic_directed(&self.graph) {
            let removed_edges = new_edges_range.rev().filter_map(|i| {
                let idx = EdgeIndex::new(i);
                self.graph.remove_edge(idx)
            });
            Err(WouldCycle(removed_edges.collect()))
        } else {
            Ok(EdgeIndices { indices: new_edges_range, _phantom: ::std::marker::PhantomData, })
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
        if let Some(edge_idx) = self.find_edge(a, b) {
            if let Some(edge) = self.edge_weight_mut(edge_idx) {
                *edge = weight;
                return Ok(edge_idx);
            }
        }
        self.add_edge(a, b, weight)
    }

    /// Find and return the index to the edge that describes `a` -> `b` if there is one.
    ///
    /// Computes in **O(e')** time, where **e'** is the number of edges connected to the nodes `a`
    /// and `b`.
    pub fn find_edge(&self, a: NodeIndex<Ix>, b: NodeIndex<Ix>) -> Option<EdgeIndex<Ix>> {
        self.graph.find_edge(a, b)
    }

    /// Access the parent and child nodes for the given `EdgeIndex`.
    pub fn edge_endpoints(&self, e: EdgeIndex<Ix>) -> Option<(NodeIndex<Ix>, NodeIndex<Ix>)> {
        self.graph.edge_endpoints(e)
    }

    /// Remove all edges.
    pub fn clear_edges(&mut self) {
        self.graph.clear_edges()
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

    /// Read from the internal node array.
    pub fn raw_nodes(&self) -> RawNodes<N, Ix> {
        self.graph.raw_nodes()
    }

    /// An iterator yielding mutable access to all node weights.
    ///
    /// The order in which weights are yielded matches the order of their node indices.
    pub fn node_weights_mut(&mut self) -> NodeWeightsMut<N, Ix> {
        self.graph.node_weights_mut()
    }

    /// Borrow the weight from the edge at the given index.
    pub fn edge_weight(&self, edge: EdgeIndex<Ix>) -> Option<&E> {
        self.graph.edge_weight(edge)
    }

    /// Mutably borrow the weight from the edge at the given index.
    pub fn edge_weight_mut(&mut self, edge: EdgeIndex<Ix>) -> Option<&mut E> {
        self.graph.edge_weight_mut(edge)
    }

    /// Read from the internal edge array.
    pub fn raw_edges(&self) -> RawEdges<E, Ix> {
        self.graph.raw_edges()
    }

    /// An iterator yielding mutable access to all edge weights.
    ///
    /// The order in which weights are yielded matches the order of their edge indices.
    pub fn edge_weights_mut(&mut self) -> EdgeWeightsMut<E, Ix> {
        self.graph.edge_weights_mut()
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
    /// Note: Calling this may shift (and in turn invalidate) previously returned node indices!
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

    /// A **Walker** type that may be used to step through the parents of the given child node.
    ///
    /// Unlike iterator types, **Walker**s do not require borrowing the internal **Graph**. This
    /// makes them useful for traversing the **Graph** while still being able to mutably borrow it.
    ///
    /// If you require an iterator, use one of the **Walker** methods for converting this
    /// **Walker** into a similarly behaving **Iterator** type.
    ///
    /// See the [**Walker**](./walker/trait.Walker.html) trait for more useful methods.
    pub fn parents(&self, child: NodeIndex<Ix>) -> Parents<N, E, Ix> {
        let walk_edges = self.graph.walk_edges_directed(child, pg::Incoming);
        Parents {
            walk_edges: walk_edges,
            _node: PhantomData,
            _edge: PhantomData,
        }
    }

    /// A "walker" object that may be used to step through the children of the given parent node.
    ///
    /// Unlike iterator types, **Walker**s do not require borrowing the internal **Graph**. This
    /// makes them useful for traversing the **Graph** while still being able to mutably borrow it.
    ///
    /// If you require an iterator, use one of the **Walker** methods for converting this
    /// **Walker** into a similarly behaving **Iterator** type.
    ///
    /// See the [**Walker**](./walker/trait.Walker.html) trait for more useful methods.
    pub fn children(&self, parent: NodeIndex<Ix>) -> Children<N, E, Ix> {
        let walk_edges = self.graph.walk_edges_directed(parent, pg::Outgoing);
        Children {
            walk_edges: walk_edges,
            _node: PhantomData,
            _edge: PhantomData,
        }
    }

    /// A **Walker** type that recursively walks the **Dag** using the given `recursive_fn`.
    ///
    /// See the [**Walker**](./walker/trait.Walker.html) trait for more useful methods.
    pub fn recursive_walk<F>(&self, start: NodeIndex<Ix>, recursive_fn: F)
        -> RecursiveWalk<N, E, Ix, F>
        where F: FnMut(&Self, NodeIndex<Ix>) -> Option<(EdgeIndex<Ix>, NodeIndex<Ix>)>
    {
        walker::Recursive::new(start, recursive_fn)
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


impl<N, E, Ix> Walker<Dag<N, E, Ix>> for Children<N, E, Ix>
    where Ix: IndexType,
{
    type Index = Ix;
    #[inline]
    fn next(&mut self, dag: &Dag<N, E, Ix>) -> Option<(EdgeIndex<Ix>, NodeIndex<Ix>)> {
        self.walk_edges.next_neighbor(&dag.graph)
    }
}

impl<N, E, Ix> Walker<Dag<N, E, Ix>> for Parents<N, E, Ix>
    where Ix: IndexType,
{
    type Index = Ix;
    #[inline]
    fn next(&mut self, dag: &Dag<N, E, Ix>) -> Option<(EdgeIndex<Ix>, NodeIndex<Ix>)> {
        self.walk_edges.next_neighbor(&dag.graph)
    }
}


impl<Ix> Iterator for EdgeIndices<Ix> where Ix: IndexType {
    type Item = EdgeIndex<Ix>;
    fn next(&mut self) -> Option<EdgeIndex<Ix>> {
        self.indices.next().map(|i| EdgeIndex::new(i))
    }
}


impl<E> ::std::fmt::Display for WouldCycle<E> where E: ::std::fmt::Debug {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        writeln!(f, "{:?}", self)
    }
}

impl<E> ::std::error::Error for WouldCycle<E> where E: ::std::fmt::Debug + ::std::any::Any {
    fn description(&self) -> &str {
        "Adding this input would have caused the graph to cycle!"
    }
}
