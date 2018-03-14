# daggy [![Build Status](https://travis-ci.org/mitchmindtree/daggy.svg?branch=master)](https://travis-ci.org/mitchmindtree/daggy) [![Crates.io](https://img.shields.io/crates/v/daggy.svg)](https://crates.io/crates/daggy) [![Crates.io](https://img.shields.io/crates/l/daggy.svg)](https://github.com/mitchmindtree/daggy/blob/master/LICENSE-MIT) [![docs.rs](https://docs.rs/daggy/badge.svg)](https://docs.rs/daggy/)


A [directed acyclic graph](https://en.wikipedia.org/wiki/Directed_acyclic_graph) data structure for Rust.

It is Implemented on top of [petgraph](https://github.com/bluss/petulant-avenger-graphlibrary)'s [Graph](https://docs.rs/petgraph/0.4.11/petgraph/graph/struct.Graph.html) data structure and attempts to follow similar conventions where suitable.


Usage
-----

Please see the [tests directory](https://github.com/mitchmindtree/daggy/tree/master/tests) for some basic usage examples.

Use daggy in your project by adding it to your Cargo.toml dependencies like so:

```toml
[dependencies]
daggy = "*"
```

and then adding

```rust
extern crate daggy;
```

to your lib.rs.


License
-------

Dual-licensed to be compatible with the petgraph and Rust projects.

Licensed under the Apache License, Version 2.0 http://www.apache.org/licenses/LICENSE-2.0 or the MIT license http://opensource.org/licenses/MIT, at your option. This file may not be copied, modified, or distributed except according to those terms.

