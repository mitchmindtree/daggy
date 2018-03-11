//! **Walker** is a trait providing a variety of useful methods for traversing graph types.

use petgraph::visit::{GraphBase, GraphRef, Walker};
use std::marker::PhantomData;

/// Recursively walks a graph using the recursive function `recursive_fn`.
#[derive(Clone, Debug)]
pub struct Recursive<G, F>
where
    G: GraphBase,
{
    next: G::NodeId,
    recursive_fn: F,
    _graph: PhantomData<G>,
}

impl<G, F> Recursive<G, F>
where
    G: GraphBase,
    F: FnMut(&G, G::NodeId) -> Option<(G::EdgeId, G::NodeId)>,
{
    /// Construct a new **Recursive** **Walker** starting from the node at the given index.
    pub fn new(start: G::NodeId, recursive_fn: F) -> Self {
        Recursive {
            next: start,
            recursive_fn: recursive_fn,
            _graph: PhantomData,
        }
    }

    /// Yield the next recursion step.
    pub fn next(&mut self, g: &G) -> Option<(G::EdgeId, G::NodeId)> {
        let Recursive {
            ref mut next,
            ref mut recursive_fn,
            ..
        } = *self;
        recursive_fn(g, *next).map(|(e, n)| {
            *next = n;
            (e, n)
        })
    }
}

impl<'a, G, F> Walker<&'a G> for Recursive<G, F>
where
    G: GraphBase,
    F: FnMut(&G, G::NodeId) -> Option<(G::EdgeId, G::NodeId)>,
{
    type Item = (G::EdgeId, G::NodeId);
    #[inline]
    fn walk_next(&mut self, g: &G) -> Option<Self::Item> {
        self.next(g)
    }
}

/// Walks the entirety of `a` before walking the entirety of `b`.
#[derive(Clone, Debug)]
pub struct Chain<G, A, B> {
    a: Option<A>,
    b: B,
    _graph: PhantomData<G>,
}

impl<G, A, B> Chain<G, A, B> {
    /// Create a new `Chain`.
    pub fn new(a: A, b: B) -> Self {
        Chain {
            a: Some(a),
            b,
            _graph: PhantomData,
        }
    }
}

impl<'a, G, A, B> Walker<&'a G> for Chain<G, A, B>
where
    G: GraphBase,
    A: Walker<&'a G>,
    B: Walker<&'a G, Item = A::Item>,
{
    type Item = A::Item;
    #[inline]
    fn walk_next(&mut self, graph: &'a G) -> Option<Self::Item> {
        let Chain {
            ref mut a,
            ref mut b,
            ..
        } = *self;
        match a.as_mut().and_then(|walker| walker.walk_next(graph)) {
            Some(step) => Some(step),
            None => {
                *a = None;
                b.walk_next(graph)
            }
        }
    }
}

/// A walker that applies some given predicate to each element returned by its walker.
///
/// The only index pairs that will be yielded are those that make the predicate evaluate to true.
#[derive(Clone, Debug)]
pub struct Filter<G, W, P> {
    walker: W,
    predicate: P,
    _graph: PhantomData<G>,
}

impl<G, W, P> Filter<G, W, P> {
    /// Create a new `Filter`.
    pub fn new(walker: W, predicate: P) -> Self
    where
        G: GraphRef,
        W: Walker<G>,
        P: FnMut(G, &W::Item) -> bool,
    {
        Filter {
            walker,
            predicate,
            _graph: PhantomData,
        }
    }
}

impl<G, W, P> Walker<G> for Filter<G, W, P>
where
    G: GraphRef,
    W: Walker<G>,
    P: FnMut(G, &W::Item) -> bool,
{
    type Item = W::Item;
    #[inline]
    fn walk_next(&mut self, graph: G) -> Option<Self::Item> {
        while let Some(item) = self.walker.walk_next(graph) {
            if (self.predicate)(graph, &item) {
                return Some(item);
            }
        }
        None
    }
}

/// A walker that has a `.peek(&graph)` method that returns an optional next neighbor.
#[derive(Clone, Debug)]
pub struct Peekable<G, W>
where
    W: Walker<G>,
{
    walker: W,
    maybe_next: Option<W::Item>,
    _graph: PhantomData<G>,
}

impl<G, W> Peekable<G, W>
where
    G: GraphRef,
    W: Walker<G>,
{
    /// Create a new `Peekable` walker.
    pub fn new(walker: W) -> Self {
        Peekable {
            walker,
            maybe_next: None,
            _graph: PhantomData,
        }
    }

    /// The edge node index pair of the neighbor at the next step in our walk of the given graph.
    #[inline]
    pub fn peek(&mut self, graph: G) -> Option<&W::Item> {
        if self.maybe_next.is_none() {
            self.maybe_next = self.walker.walk_next(graph);
        }
        self.maybe_next.as_ref()
    }
}

impl<G, W> Walker<G> for Peekable<G, W>
where
    G: GraphRef,
    W: Walker<G>,
{
    type Item = W::Item;
    #[inline]
    fn walk_next(&mut self, graph: G) -> Option<Self::Item> {
        self.maybe_next
            .take()
            .or_else(|| self.walker.walk_next(graph))
    }
}

/// A walker that invokes the predicate on elements until it returns false.
///
/// Once the predicate returns false, that element and all further elements are yielded.
#[derive(Clone, Debug)]
pub struct SkipWhile<G, W, P> {
    walker: W,
    maybe_predicate: Option<P>,
    _graph: PhantomData<G>,
}

impl<G, W, P> SkipWhile<G, W, P> {
    /// Create a new `SkipWhile` walker.
    pub fn new(walker: W, predicate: P) -> Self
    where
        G: GraphRef,
        W: Walker<G>,
        P: FnMut(G, &W::Item) -> bool,
    {
        SkipWhile {
            walker,
            maybe_predicate: Some(predicate),
            _graph: PhantomData,
        }
    }
}

impl<G, W, P> Walker<G> for SkipWhile<G, W, P>
where
    G: GraphRef,
    W: Walker<G>,
    P: FnMut(G, &W::Item) -> bool,
{
    type Item = W::Item;
    #[inline]
    fn walk_next(&mut self, graph: G) -> Option<Self::Item> {
        match self.maybe_predicate.take() {
            Some(mut predicate) => {
                while let Some(item) = self.walker.walk_next(graph) {
                    if !predicate(graph, &item) {
                        return Some(item);
                    }
                }
                None
            }
            None => self.walker.walk_next(graph),
        }
    }
}

/// A walker that yields elements so long as the predicate returns true.
///
/// After the predicate returns false for the first time, no further elements will be yielded.
#[derive(Clone, Debug)]
pub struct TakeWhile<G, W, P> {
    maybe_walker: Option<W>,
    predicate: P,
    _graph: PhantomData<G>,
}

impl<G, W, P> TakeWhile<G, W, P> {
    /// Create a new `TakeWhile` walker.
    pub fn new(walker: W, predicate: P) -> Self
    where
        G: GraphRef,
        W: Walker<G>,
        P: FnMut(G, &W::Item) -> bool,
    {
        TakeWhile {
            maybe_walker: Some(walker),
            predicate,
            _graph: PhantomData,
        }
    }
}

impl<G, W, P> Walker<G> for TakeWhile<G, W, P>
where
    G: GraphRef,
    W: Walker<G>,
    P: FnMut(G, &W::Item) -> bool,
{
    type Item = W::Item;
    #[inline]
    fn walk_next(&mut self, graph: G) -> Option<Self::Item> {
        let TakeWhile {
            ref mut maybe_walker,
            ref mut predicate,
            ..
        } = *self;
        let maybe_next_idx_pair = maybe_walker
            .as_mut()
            .and_then(|walker| walker.walk_next(graph));
        if let Some(item) = maybe_next_idx_pair {
            if predicate(graph, &item) {
                return Some(item);
            } else {
                *maybe_walker = None;
            }
        }
        None
    }
}

/// A walker that skips the first n steps of this walk, and then yields all further steps.
#[derive(Clone, Debug)]
pub struct Skip<G, W> {
    walker: W,
    to_skip: usize,
    _graph: PhantomData<G>,
}

impl<G, W> Skip<G, W> {
    /// Create a new `Skip` walker..
    pub fn new(walker: W, to_skip: usize) -> Self {
        Skip {
            walker,
            to_skip,
            _graph: PhantomData,
        }
    }
}

impl<G, W> Walker<G> for Skip<G, W>
where
    G: GraphRef,
    W: Walker<G>,
{
    type Item = W::Item;
    #[inline]
    fn walk_next(&mut self, graph: G) -> Option<Self::Item> {
        if self.to_skip > 0 {
            for _ in 0..self.to_skip {
                self.walker.walk_next(graph);
            }
            self.to_skip = 0;
        }
        self.walker.walk_next(graph)
    }
}

/// A walker that yields the first n steps of this walk.
#[derive(Clone, Debug)]
pub struct Take<G, W> {
    walker: W,
    to_take: usize,
    _graph: PhantomData<G>,
}

impl<G, W> Take<G, W> {
    /// Create a new `Take` walker.
    pub fn new(walker: W, to_take: usize) -> Self {
        Take {
            walker,
            to_take,
            _graph: PhantomData,
        }
    }
}

impl<G, W> Walker<G> for Take<G, W>
where
    G: GraphRef,
    W: Walker<G>,
{
    type Item = W::Item;
    #[inline]
    fn walk_next(&mut self, graph: G) -> Option<Self::Item> {
        if self.to_take > 0 {
            self.to_take -= 1;
            self.walker.walk_next(graph)
        } else {
            None
        }
    }
}

/// A walker that repeats its internal walker endlessly.
#[derive(Clone, Debug)]
pub struct Cycle<G, W> {
    walker: W,
    clone: W,
    _graph: PhantomData<G>,
}

impl<G, W> Cycle<G, W>
where
    W: Clone,
{
    /// Create a new `Cycle` walker.
    pub fn new(walker: W) -> Self {
        let clone = walker.clone();
        Cycle {
            walker,
            clone,
            _graph: PhantomData,
        }
    }
}

impl<G, W> Walker<G> for Cycle<G, W>
where
    G: GraphRef,
    W: Walker<G> + Clone,
{
    type Item = W::Item;
    #[inline]
    fn walk_next(&mut self, graph: G) -> Option<Self::Item> {
        self.clone.walk_next(graph).or_else(|| {
            self.clone = self.walker.clone();
            self.clone.walk_next(graph)
        })
    }
}

/// A walker that calls a function with a reference to each index pair before yielding them.
///
/// This is often useful for debugging a walker pipeline.
#[derive(Clone, Debug)]
pub struct Inspect<W, F> {
    walker: W,
    f: F,
}

impl<W, F> Inspect<W, F> {
    /// Create a new `Inspect` walker.
    pub fn new(walker: W, f: F) -> Self {
        Inspect { walker, f }
    }
}

impl<G, W, F> Walker<G> for Inspect<W, F>
where
    G: GraphRef,
    W: Walker<G>,
    F: FnMut(G, &W::Item),
{
    type Item = W::Item;
    #[inline]
    fn walk_next(&mut self, graph: G) -> Option<W::Item> {
        self.walker.walk_next(graph).map(|item| {
            (self.f)(graph, &item);
            item
        })
    }
}
