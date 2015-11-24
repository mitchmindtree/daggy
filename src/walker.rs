//! **Walker** is a trait providing a variety of useful methods for traversing graph types.


use ::{EdgeIndex, NodeIndex};
use pg::graph::IndexType;
use std::marker::PhantomData;
use std::ops::Index;


/// Short-hand for an edge node index pair.
pub type IndexPair<Ix> = (EdgeIndex<Ix>, NodeIndex<Ix>);


/// A trait providing a variety of useful methods for traversing some graph type **G**.
///
/// **Walker** can be likened to the std **Iterator** trait. It's methods behave similarly, but it
/// is different in that it takes a reference to some graph as an argument to its "next" method.
///
/// **Walker** method return types (besides the iterators) never borrow the graph. This means that
/// we can still safely mutably borrow from the graph whilst we traverse it.
pub trait Walker<G> {
    /// The unsigned integer type used for node and edge indices.
    type Index: IndexType;

    /// Fetch the `EdgeIndex` and `NodeIndex` to the next neighbour in our walk through the given
    /// **Graph**.
    fn next(&mut self, graph: &G) -> Option<IndexPair<Self::Index>>;

    /// The next edge in our walk for the given **Graph**.
    #[inline]
    fn next_edge(&mut self, graph: &G) -> Option<EdgeIndex<Self::Index>> {
        self.next(graph).map(|(e, _)| e)
    }

    /// The next node in our walk for the given **Graph**.
    #[inline]
    fn next_node(&mut self, graph: &G) -> Option<NodeIndex<Self::Index>> {
        self.next(graph).map(|(_, n)| n)
    }

    /// Counts all the steps in the entire walk of the given graph.
    #[inline]
    fn count(mut self, graph: &G) -> usize where Self: Sized {
        let mut count = 0;
        while let Some(_) = self.next(graph) {
            count += 1;
        }
        count
    }

    /// Walks the whole walk until reaching and returning the last edge node pair.
    #[inline]
    fn last(mut self, graph: &G) -> Option<IndexPair<Self::Index>> where Self: Sized {
        let mut maybe_last_pair = None;
        while let Some(pair) = self.next(graph) {
            maybe_last_pair = Some(pair);
        }
        maybe_last_pair
    }

    /// Walks the whole walk until reaching and returning the last edge.
    #[inline]
    fn last_edge(self, graph: &G) -> Option<EdgeIndex<Self::Index>> where Self: Sized {
        self.last(graph).map(|(e, _)| e)
    }

    /// Walks the whole walk until reaching and returning the last node.
    #[inline]
    fn last_node(self, graph: &G) -> Option<NodeIndex<Self::Index>> where Self: Sized {
        self.last(graph).map(|(_, n)| n)
    }

    /// Walks "n" number of steps and produces the resulting edge node pair.
    #[inline]
    fn nth(mut self, graph: &G, n: usize) -> Option<IndexPair<Self::Index>>
        where Self: Sized
    {
        (0..n+1)
            .map(|_| self.next(graph))
            .nth(n)
            .and_then(|maybe_pair| maybe_pair)
    }

    /// Walks "n" number of steps and produces the resulting edge.
    #[inline]
    fn nth_edge(self, graph: &G, n: usize) -> Option<EdgeIndex<Self::Index>>
        where Self: Sized
    {
        self.nth(graph, n).map(|(e, _)| e)
    }

    /// Walks "n" number of steps and produces the resulting node.
    #[inline]
    fn nth_node(self, graph: &G, n: usize) -> Option<NodeIndex<Self::Index>>
        where Self: Sized
    {
        self.nth(graph, n).map(|(_, n)| n)
    }

    /// Produces a walker that will walk the entirey of `self` before walking the entirey of other.
    #[inline]
    fn chain<O>(self, other: O) -> Chain<G, Self::Index, Self, O>
        where Self: Sized,
              O: Walker<G, Index=Self::Index>,
    {
        Chain {
            a: Some(self),
            b: other,
            _graph: PhantomData,
            _index: PhantomData,
        }
    }

    /// Creates a walker that applies the predicate to each element returned by this walker.
    /// The only elements that will be yielded are those that make the predicate evaluate to true.
    #[inline]
    fn filter<P>(self, predicate: P) -> Filter<Self, P>
        where Self: Sized,
              P: FnMut(&G, EdgeIndex<Self::Index>, NodeIndex<Self::Index>) -> bool,
    {
        Filter {
            walker: self,
            predicate: predicate,
        }
    }

    /// Creates a walker that has a `.peek(&graph)` method that returns an optional next neighbor.
    #[inline]
    fn peekable(self) -> Peekable<G, Self::Index, Self> where Self: Sized {
        Peekable {
            walker: self,
            maybe_next: None,
            _graph: PhantomData,
        }
    }

    /// Creates a walker that invokes the predicate on elements until it returns false. Once the
    /// predicate returns false, that element and all further elements are yielded.
    #[inline]
    fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P>
        where Self: Sized,
              P: FnMut(&G, EdgeIndex<Self::Index>, NodeIndex<Self::Index>) -> bool,
    {
        SkipWhile {
            walker: self,
            maybe_predicate: Some(predicate),
        }
    }

    /// Creates a walker that yields elements so long as the predicate returns true. After the
    /// predicate returns false for the first time, no further elements will be yielded.
    #[inline]
    fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>
        where Self: Sized,
              P: FnMut(&G, EdgeIndex<Self::Index>, NodeIndex<Self::Index>) -> bool,
    {
        TakeWhile {
            maybe_walker: Some(self),
            predicate: predicate,
        }
    }

    /// Creates a walker that skips the first n steps of this walk, and then yields all further
    /// steps.
    #[inline]
    fn skip(self, n: usize) -> Skip<G, Self::Index, Self> where Self: Sized {
        Skip {
            walker: self,
            to_skip: n,
            _graph: PhantomData,
            _index: PhantomData,
        }
    }

    /// Creates a walker that yields the first n steps of this walk.
    #[inline]
    fn take(self, n: usize) -> Take<G, Self::Index, Self> where Self: Sized {
        Take {
            walker: self,
            to_take: n,
            _graph: PhantomData,
            _index: PhantomData,
        }
    }

    /// Tests whether the predicate holds true for all steps in the walk.
    #[inline]
    fn all<P>(&mut self, graph: &G, mut predicate: P) -> bool
        where P: FnMut(&G, EdgeIndex<Self::Index>, NodeIndex<Self::Index>) -> bool,
    {
        while let Some((e, n)) = self.next(graph) {
            if !predicate(graph, e, n) {
                return false;
            }
        }
        true
    }

    /// Tests whether any step in the walk satisfies the given predicate.
    ///
    /// Does not step the walker past the first found step.
    #[inline]
    fn any<P>(&mut self, graph: &G, mut predicate: P) -> bool
        where P: FnMut(&G, EdgeIndex<Self::Index>, NodeIndex<Self::Index>) -> bool,
    {
        while let Some((e, n)) = self.next(graph) {
            if predicate(graph, e, n) {
                return true;
            }
        }
        false
    }

    /// Returns the first edge node index pair satisfying the specified predicate.
    /// 
    /// Does not consume the walker past the first found step.
    #[inline]
    fn find<P>(&mut self, graph: &G, mut predicate: P) -> Option<IndexPair<Self::Index>>
        where P: FnMut(&G, EdgeIndex<Self::Index>, NodeIndex<Self::Index>) -> bool
    {
        while let Some((e, n)) = self.next(graph) {
            if predicate(graph, e, n) {
                return Some((e, n));
            }
        }
        None
    }

    /// Returns the edge index satisfying the specified predicate.
    /// 
    /// Does not consume the walker past the first found step.
    #[inline]
    fn find_edge<P>(&mut self, graph: &G, predicate: P) -> Option<EdgeIndex<Self::Index>>
        where P: FnMut(&G, EdgeIndex<Self::Index>, NodeIndex<Self::Index>) -> bool
    {
        self.find(graph, predicate).map(|(e, _)| e)
    }

    /// Returns the node index satisfying the specified predicate.
    /// 
    /// Does not consume the walker past the first found step.
    #[inline]
    fn find_node<P>(&mut self, graph: &G, predicate: P) -> Option<NodeIndex<Self::Index>>
        where P: FnMut(&G, EdgeIndex<Self::Index>, NodeIndex<Self::Index>) -> bool
    {
        self.find(graph, predicate).map(|(_, n)| n)
    }

    /// Repeats the walker endlessly.
    #[inline]
    fn cycle(self) -> Cycle<G, Self::Index, Self> where Self: Clone + Sized {
        let clone = self.clone();
        Cycle {
            walker: self,
            clone: clone,
            _graph: PhantomData,
            _index: PhantomData,
        }
    }

    /// Performs a fold operation over the entire walker, returning the eventual state at the end
    /// of the walk.
    /// 
    /// This operation is sometimes called 'reduce' or 'inject'.
    #[inline]
    fn fold<B, F>(mut self, init: B, graph: &G, mut f: F) -> B
        where Self: Sized,
              F: FnMut(B, &G, EdgeIndex<Self::Index>, NodeIndex<Self::Index>) -> B
    {
        let mut accum = init;
        while let Some((e, n)) = self.next(graph) {
            accum = f(accum, graph, e, n);
        }
        accum
    }

    /// Creates a walker that calls a function with a reference to each index pair before yielding
    /// them. This is often useful for debugging a walker pipeline.
    #[inline]
    fn inspect<F>(self, f: F) -> Inspect<Self, F>
        where Self: Sized,
              F: FnMut(&G, EdgeIndex<Self::Index>, NodeIndex<Self::Index>),
    {
        Inspect {
            walker: self,
            f: f,
        }
    }

    /// Converts the walker into an iterator yielding index pairs.
    ///
    /// The returned iterator borrows the graph.
    #[inline]
    fn iter(self, graph: &G) -> Iter<G, Self::Index, Self>
        where Self: Sized,
    {
        Iter {
            walker: self,
            graph: graph,
            _index: PhantomData,
        }
    }

    /// Converts the walker into an iterator yielding `(&e, &n)`, where `e` is the edge weight for
    /// the next `EdgeIndex` and `n` is the node weight for the next `NodeIndex`.
    ///
    /// The returned iterator borrows the graph.
    #[inline]
    fn iter_weights(self, graph: &G) -> IterWeights<G, Self::Index, Self>
        where Self: Sized,
    {
        IterWeights {
            walker: self,
            graph: graph,
            _index: PhantomData,
        }
    }

}


// /// The **Walker** synonym to the **std::iter::once** function.
// ///
// /// Returns a walker that takes just one step, yielding the given index pair.
// pub fn once<Ix>(e: EdgeIndex<Ix>, n: NodeIndex<Ix>) -> Once<Ix> {
// }


/// Recursively walks a graph using the recursive function `recursive_fn`.
#[derive(Clone, Debug)]
pub struct Recursive<G, Ix, F> {
    next: NodeIndex<Ix>,
    recursive_fn: F,
    _graph: PhantomData<G>,
}

impl<G, Ix, F> Recursive<G, Ix, F> {

    /// Construct a new **Recursive** **Walker** starting from the node at the given index.
    pub fn new(start: NodeIndex<Ix>, recursive_fn: F) -> Self
        where Ix: IndexType,
              F: FnMut(&G, NodeIndex<Ix>) -> Option<IndexPair<Ix>>,
    {
        Recursive {
            next: start,
            recursive_fn: recursive_fn,
            _graph: PhantomData,
        }
    }

}

impl<G, Ix, F> Walker<G> for Recursive<G, Ix, F>
    where Ix: IndexType,
          F: FnMut(&G, NodeIndex<Ix>) -> Option<IndexPair<Ix>>,
{
    type Index = Ix;
    #[inline]
    fn next(&mut self, graph: &G) -> Option<IndexPair<Ix>> {
        let Recursive { ref mut next, ref mut recursive_fn, .. } = *self;
        recursive_fn(graph, *next).map(|(e, n)| {
            *next = n;
            (e, n)
        })
    }
}


/// Walks the entirety of `a` before walking the entirety of `b`.
#[derive(Clone, Debug)]
pub struct Chain<G, Ix, A, B> {
    a: Option<A>,
    b: B,
    _graph: PhantomData<G>,
    _index: PhantomData<Ix>,
}


impl<G, Ix, A, B> Walker<G> for Chain<G, Ix, A, B>
    where Ix: IndexType,
          A: Walker<G, Index=Ix>,
          B: Walker<G, Index=Ix>,
{
    type Index = Ix;
    #[inline]
    fn next(&mut self, graph: &G) -> Option<IndexPair<Ix>> {
        let Chain { ref mut a, ref mut b, .. } = *self;
        match a.as_mut().and_then(|walker| walker.next(graph)) {
            Some(step) => Some(step),
            None => {
                *a = None;
                b.next(graph)
            },
        }
    }
}


/// A walker that applies some given predicate to each element returned by its walker.
/// The only index pairs that will be yielded are those that make the predicate evaluate to true.
#[derive(Clone, Debug)]
pub struct Filter<W, P> {
    walker: W,
    predicate: P,
}


impl<G, Ix, W, P> Walker<G> for Filter<W, P>
    where Ix: IndexType,
          W: Walker<G, Index=Ix>,
          P: FnMut(&G, EdgeIndex<Ix>, NodeIndex<Ix>) -> bool,
{
    type Index = Ix;
    #[inline]
    fn next(&mut self, graph: &G) -> Option<IndexPair<Ix>> {
        while let Some((e, n)) = self.walker.next(graph) {
            if (self.predicate)(graph, e, n) {
                return Some((e, n));
            }
        }
        None
    }
}


/// A walker that has a `.peek(&graph)` method that returns an optional next neighbor.
#[derive(Clone, Debug)]
pub struct Peekable<G, Ix, W> where Ix: IndexType {
    walker: W,
    maybe_next: Option<IndexPair<Ix>>,
    _graph: PhantomData<G>,
}


impl<G, Ix, W> Peekable<G, Ix, W>
    where Ix: IndexType,
          W: Walker<G, Index=Ix>,
{

    /// The edge node index pair of the neighbor at the next step in our walk of the given graph.
    #[inline]
    pub fn peek(&mut self, graph: &G) -> Option<IndexPair<Ix>> {
        self.maybe_next.or_else(|| {
            self.maybe_next = self.walker.next(graph);
            self.maybe_next
        })
    }

    /// The edge index of the neighbor at the next step in our walk of the given graph.
    #[inline]
    pub fn peek_edge(&mut self, graph: &G) -> Option<EdgeIndex<Ix>> {
        self.peek(graph).map(|(e, _)| e)
    }

    /// The node index of the neighbor at the next step in our walk of the given graph.
    #[inline]
    pub fn peek_node(&mut self, graph: &G) -> Option<NodeIndex<Ix>> {
        self.peek(graph).map(|(_, n)| n)
    }

}


impl<G, Ix, W> Walker<G> for Peekable<G, Ix, W>
    where Ix: IndexType,
          W: Walker<G, Index=Ix>,
{
    type Index = Ix;
    #[inline]
    fn next(&mut self, graph: &G) -> Option<IndexPair<Ix>> {
        self.maybe_next.take().or_else(|| {
            self.walker.next(graph)
        })
    }
}


/// A walker that invokes the predicate on elements until it returns false. Once the predicate
/// returns false, that element and all further elements are yielded.
#[derive(Clone, Debug)]
pub struct SkipWhile<W, P> {
    walker: W,
    maybe_predicate: Option<P>,
}


impl<G, Ix, W, P> Walker<G> for SkipWhile<W, P>
    where Ix: IndexType,
          W: Walker<G, Index=Ix>,
          P: FnMut(&G, EdgeIndex<Ix>, NodeIndex<Ix>) -> bool,
{
    type Index = Ix;
    #[inline]
    fn next(&mut self, graph: &G) -> Option<IndexPair<Ix>> {
        match self.maybe_predicate.take() {
            Some(mut predicate) => {
                while let Some((e, n)) = self.walker.next(graph) {
                    if !predicate(graph, e, n) {
                        return Some((e, n));
                    }
                }
                None
            },
            None => self.walker.next(graph),
        }
    }
}


/// A walker that yields elements so long as the predicate returns true. After the
/// predicate returns false for the first time, no further elements will be yielded.
#[derive(Clone, Debug)]
pub struct TakeWhile<W, P> {
    maybe_walker: Option<W>,
    predicate: P,
}


impl<G, Ix, W, P> Walker<G> for TakeWhile<W, P>
    where Ix: IndexType,
          W: Walker<G, Index=Ix>,
          P: FnMut(&G, EdgeIndex<Ix>, NodeIndex<Ix>) -> bool,
{
    type Index = Ix;
    #[inline]
    fn next(&mut self, graph: &G) -> Option<IndexPair<Ix>> {
        let TakeWhile { ref mut maybe_walker, ref mut predicate } = *self;
        let maybe_next_idx_pair = maybe_walker.as_mut().and_then(|walker| walker.next(graph));
        if let Some((e, n)) = maybe_next_idx_pair {
            if predicate(graph, e, n) {
                return Some((e, n));
            } else {
                *maybe_walker = None;
            }
        }
        None
    }
}


/// A walker that skips the first n steps of this walk, and then yields all further steps.
#[derive(Clone, Debug)]
pub struct Skip<G, Ix, W> {
    walker: W,
    to_skip: usize,
    _graph: PhantomData<G>,
    _index: PhantomData<Ix>,
}


impl<G, Ix, W> Walker<G> for Skip<G, Ix, W>
    where Ix: IndexType,
          W: Walker<G, Index=Ix>,
{
    type Index = Ix;
    #[inline]
    fn next(&mut self, graph: &G) -> Option<IndexPair<Ix>> {
        if self.to_skip > 0 {
            for _ in 0..self.to_skip {
                self.walker.next(graph);
            }
            self.to_skip = 0;
        }
        self.walker.next(graph)
    }
}


/// A walker that yields the first n steps of this walk.
#[derive(Clone, Debug)]
pub struct Take<G, Ix, W> {
    walker: W,
    to_take: usize,
    _graph: PhantomData<G>,
    _index: PhantomData<Ix>,
}


impl<G, Ix, W> Walker<G> for Take<G, Ix, W>
    where Ix: IndexType,
          W: Walker<G, Index=Ix>,
{
    type Index = Ix;
    #[inline]
    fn next(&mut self, graph: &G) -> Option<IndexPair<Ix>> {
        if self.to_take > 0 {
            self.to_take -= 1;
            self.walker.next(graph)
        } else {
            None
        }
    }
}


/// A walker that repeats its internal walker endlessly.
#[derive(Clone, Debug)]
pub struct Cycle<G, Ix, W> {
    walker: W,
    clone: W,
    _graph: PhantomData<G>,
    _index: PhantomData<Ix>,
}


impl<G, Ix, W> Walker<G> for Cycle<G, Ix, W>
    where Ix: IndexType,
          W: Walker<G, Index=Ix> + Clone,
{
    type Index = Ix;
    #[inline]
    fn next(&mut self, graph: &G) -> Option<IndexPair<Ix>> {
        self.clone.next(graph).or_else(|| {
            self.clone = self.walker.clone();
            self.clone.next(graph)
        })
    }
}


/// A walker that calls a function with a reference to each index pair before yielding them.
/// This is often useful for debugging a walker pipeline.
#[derive(Clone, Debug)]
pub struct Inspect<W, F> {
    walker: W,
    f: F,
}


impl<G, Ix, W, F> Walker<G> for Inspect<W, F>
    where Ix: IndexType,
          W: Walker<G, Index=Ix>,
          F: FnMut(&G, EdgeIndex<Ix>, NodeIndex<Ix>),
{
    type Index = Ix;
    #[inline]
    fn next(&mut self, graph: &G) -> Option<IndexPair<Ix>> {
        self.walker.next(graph).map(|(e, n)| {
            (self.f)(graph, e, n);
            (e, n)
        })
    }
}


/// An iterator yielding index pairs produced by its internal walker and graph.
#[derive(Clone, Debug)]
pub struct Iter<'a, G: 'a, Ix, W> {
    graph: &'a G,
    walker: W,
    _index: PhantomData<Ix>,
}

impl<'a, G, Ix, W> Iter<'a, G, Ix, W> {

    /// Convert to an iterator that only yields the edge indices.
    #[inline]
    pub fn edges(self) -> IterEdges<'a, G, Ix, W> {
        let Iter { graph, walker, .. } = self;
        IterEdges {
            graph: graph,
            walker: walker,
            _index: PhantomData,
        }
    }

    /// Convert to an iterator that only yields the node indices.
    #[inline]
    pub fn nodes(self) -> IterNodes<'a, G, Ix, W> {
        let Iter { graph, walker, .. } = self;
        IterNodes {
            graph: graph,
            walker: walker,
            _index: PhantomData,
        }
    }

}

impl<'a, G, Ix, W> Iterator for Iter<'a, G, Ix, W>
    where Ix: IndexType,
          W: Walker<G, Index=Ix>,
{
    type Item = IndexPair<Ix>;
    #[inline]
    fn next(&mut self) -> Option<IndexPair<Ix>> {
        let Iter { ref mut walker, ref graph, .. } = *self;
        walker.next(graph)
    }
}


/// An iterator yielding edge indices produced by its internal walker and graph.
#[derive(Clone, Debug)]
pub struct IterEdges<'a, G: 'a, Ix, W> {
    graph: &'a G,
    walker: W,
    _index: PhantomData<Ix>,
}

impl<'a, G, Ix, W> Iterator for IterEdges<'a, G, Ix, W>
    where Ix: IndexType,
          W: Walker<G, Index=Ix>,
{
    type Item = EdgeIndex<Ix>;
    #[inline]
    fn next(&mut self) -> Option<EdgeIndex<Ix>> {
        let IterEdges { ref mut walker, ref graph, .. } = *self;
        walker.next_edge(graph)
    }
}


/// An iterator yielding node indices produced by its internal walker and graph.
#[derive(Clone, Debug)]
pub struct IterNodes<'a, G: 'a, Ix, W> {
    graph: &'a G,
    walker: W,
    _index: PhantomData<Ix>,
}

impl<'a, G, Ix, W> Iterator for IterNodes<'a, G, Ix, W>
    where Ix: IndexType,
          W: Walker<G, Index=Ix>,
{
    type Item = NodeIndex<Ix>;
    #[inline]
    fn next(&mut self) -> Option<NodeIndex<Ix>> {
        let IterNodes { ref mut walker, ref graph, .. } = *self;
        walker.next_node(graph)
    }
}


/// An iterator yielding weights associated with the index pairs produced by its internal walker
/// and graph.
#[derive(Clone, Debug)]
pub struct IterWeights<'a, G: 'a, Ix, W> {
    graph: &'a G,
    walker: W,
    _index: PhantomData<Ix>,
}

impl<'a, G, Ix, W> IterWeights<'a, G, Ix, W> {

    /// Convert to an iterator yielding only the edge weights.
    pub fn edges(self) -> IterEdgeWeights<'a, G, Ix, W> {
        let IterWeights { graph, walker, .. } = self;
        IterEdgeWeights {
            graph: graph,
            walker: walker,
            _index: PhantomData,
        }
    }

    /// Convert to an iterator yielding only the node weights.
    pub fn nodes(self) -> IterNodeWeights<'a, G, Ix, W> {
        let IterWeights { graph, walker, .. } = self;
        IterNodeWeights {
            graph: graph,
            walker: walker,
            _index: PhantomData,
        }
    }

}

impl<'a, G, Ix, W> Iterator for IterWeights<'a, G, Ix, W>
    where Ix: IndexType,
          W: Walker<G, Index=Ix>,
          G: Index<EdgeIndex<Ix>>,
          G: Index<NodeIndex<Ix>>,
{
    type Item = (&'a <G as Index<EdgeIndex<Ix>>>::Output, &'a <G as Index<NodeIndex<Ix>>>::Output);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let IterWeights { ref mut walker, ref graph, .. } = *self;
        walker.next(graph).map(|(e, n)| (&graph[e], &graph[n]))
    }
}


/// An iterator yielding edge weights associated with the indices produced by its internal walker
/// and graph.
#[derive(Clone, Debug)]
pub struct IterEdgeWeights<'a, G: 'a, Ix, W> {
    graph: &'a G,
    walker: W,
    _index: PhantomData<Ix>,
}

impl<'a, G, Ix, W> Iterator for IterEdgeWeights<'a, G, Ix, W>
    where Ix: IndexType,
          W: Walker<G, Index=Ix>,
          G: Index<EdgeIndex<Ix>>,
{
    type Item = &'a <G as Index<EdgeIndex<Ix>>>::Output;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let IterEdgeWeights { ref mut walker, ref graph, .. } = *self;
        walker.next_edge(graph).map(|e| &graph[e])
    }
}


/// An iterator yielding node weights associated with the indices produced by its internal walker
/// and graph.
#[derive(Clone, Debug)]
pub struct IterNodeWeights<'a, G: 'a, Ix, W> {
    graph: &'a G,
    walker: W,
    _index: PhantomData<Ix>,
}

impl<'a, G, Ix, W> Iterator for IterNodeWeights<'a, G, Ix, W>
    where Ix: IndexType,
          W: Walker<G, Index=Ix>,
          G: Index<NodeIndex<Ix>>,
{
    type Item = &'a <G as Index<NodeIndex<Ix>>>::Output;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let IterNodeWeights { ref mut walker, ref graph, .. } = *self;
        walker.next_node(graph).map(|n| &graph[n])
    }
}
