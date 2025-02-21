# daggy [![Actions Status](https://github.com/mitchmindtree/daggy/workflows/daggy/badge.svg)](https://github.com/mitchmindtree/daggy/actions) [![Crates.io](https://img.shields.io/crates/v/daggy.svg)](https://crates.io/crates/daggy) [![Crates.io](https://img.shields.io/crates/l/daggy.svg)](https://github.com/mitchmindtree/daggy/blob/master/LICENSE-MIT) [![docs.rs](https://docs.rs/daggy/badge.svg)](https://docs.rs/daggy/)


A [directed acyclic graph](https://en.wikipedia.org/wiki/Directed_acyclic_graph) data structure for Rust.

It is implemented on top of [petgraph](https://github.com/petgraph/petgraph)'s [Graph](https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html) data structure and attempts to follow similar conventions where suitable.


## Usage

Please see the [tests directory](https://github.com/mitchmindtree/daggy/tree/master/tests) for some basic usage examples.

Use daggy in your project by adding it to your `Cargo.toml` dependencies:

```toml
[dependencies]
daggy = "0.8.1"

# Enables the `StableDag` type.
daggy = { version = "0.8.1", features = ["stable_dag"] }

# Allows the `Dag` to be serialized and deserialized.
daggy = { version = "0.8.1", features = ["serde-1"] }
```


## License

Dual-licensed to be compatible with the petgraph and Rust projects.

Licensed under the Apache License, Version 2.0 http://www.apache.org/licenses/LICENSE-2.0 or the MIT license http://opensource.org/licenses/MIT, at your option. This file may not be copied, modified, or distributed except according to those terms.
