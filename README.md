# daggy [![Actions Status](https://github.com/mitchmindtree/daggy/workflows/daggy/badge.svg)](https://github.com/mitchmindtree/daggy/actions) [![Crates.io](https://img.shields.io/crates/v/daggy.svg)](https://crates.io/crates/daggy) [![Crates.io](https://img.shields.io/crates/l/daggy.svg)](https://github.com/mitchmindtree/daggy/blob/master/LICENSE-MIT) [![docs.rs](https://docs.rs/daggy/badge.svg)](https://docs.rs/daggy/)


A [directed acyclic graph](https://en.wikipedia.org/wiki/Directed_acyclic_graph) data structure for Rust.

It is implemented on top of [petgraph](https://github.com/petgraph/petgraph)'s [Graph](https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html) data structure and attempts to follow similar conventions where suitable.


## Usage

Use daggy in your project by adding it to your `Cargo.toml` dependencies:

```toml
[dependencies]
daggy = "0.9.0"

# Enables the `StableDag` type.
daggy = { version = "0.9.0", features = ["stable_dag"] }

# Allows the `Dag` to be serialized and deserialized.
daggy = { version = "0.9.0", features = ["serde-1"] }
```

## Examples

> Please see the [tests directory](https://github.com/mitchmindtree/daggy/tree/master/tests) for some basic usage examples.

Transitive reduction:

```rust
use daggy::Dag;

let mut dag = Dag::<&str, &str>::new();

// Reduce edges:
//
// ```text
// # Before:          | # After:
//                    |
// a -> b ----.       | a -> b ----.
//  |         |       |  |         |
//  |-> c ----|----.  |  '-> c     |
//  |    \    |    |  |       \    |
//  |     \   v    |  |        \   v
//  |------>> d    |  |         '> d
//  |          \   v  |             \
//  '----------->> e  |              '> e
// ```

let a = dag.add_node("a");

let (_, b) = dag.add_child(a, "a->b", "b");
let (_, c) = dag.add_child(a, "a->c", "c");
let (_, d) = dag.add_child(a, "a->d", "d");
let (_, e) = dag.add_child(a, "a->e", "e");

dag.add_edge(b, d, "b->d").unwrap();

dag.add_edge(c, d, "c->d").unwrap();
dag.add_edge(c, e, "c->e").unwrap();

dag.add_edge(d, e, "d->e").unwrap();

assert_eq!(dag.edge_count(), 8);

dag.transitive_reduce(vec![a]);

let mut edges = dag.graph().edge_weights().copied().collect::<Vec<_>>();
edges.sort();
assert_eq!(dag.edge_count(), 5);
assert_eq!(&edges, &["a->b", "a->c", "b->d", "c->d", "d->e"]);
```


## License

Dual-licensed to be compatible with the petgraph and Rust projects.

Licensed under the Apache License, Version 2.0 http://www.apache.org/licenses/LICENSE-2.0 or the MIT license http://opensource.org/licenses/MIT, at your option. This file may not be copied, modified, or distributed except according to those terms.
