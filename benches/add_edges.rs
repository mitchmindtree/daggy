
#![feature(test)]

extern crate test;
extern crate daggy;

use test::Bencher;

use daggy::Dag;
use daggy::NodeIndex;
struct Weight;


#[bench]
fn add_edge(bench: &mut Bencher) {
    let max_node = 10;
    let edges = &[(1, 4), (3, 4), (2, 5), (3, 5), (2, 8), (1, 9), (1, 8),
                  (1, 3), (2, 7), (1, 7), (0, 6), (1, 2), (0, 7), (1, 6),
                  (2, 4), (0, 1), (0, 9), (2, 9), (2, 6), (0, 4), (2, 3),
                  (0, 2), (0, 3), (0, 5), (0, 8), (1, 5)];
    let mut dag = Dag::<Weight, u32, u32>::with_capacity(max_node, edges.len());
    for _ in 0..max_node {
        dag.add_node(Weight);
    }

    bench.iter(|| {
        dag.clear_edges();
        edges.iter().all(|&(a, b)|
            dag.add_edge(NodeIndex::new(a), NodeIndex::new(b), 0).is_ok())
    });
}

#[bench]
fn add_edges(bench: &mut Bencher) {
    let max_node = 10;
    let edges = &[(1, 4), (3, 4), (2, 5), (3, 5), (2, 8), (1, 9), (1, 8),
                  (1, 3), (2, 7), (1, 7), (0, 6), (1, 2), (0, 7), (1, 6),
                  (2, 4), (0, 1), (0, 9), (2, 9), (2, 6), (0, 4), (2, 3),
                  (0, 2), (0, 3), (0, 5), (0, 8), (1, 5)];
    let mut dag = Dag::<Weight, u32, u32>::with_capacity(max_node, edges.len());
    for _ in 0..max_node {
        dag.add_node(Weight);
    }

    bench.iter(|| {
        dag.clear_edges();
        dag.add_edges(edges.iter().map(|&(a, b)| (NodeIndex::new(a), NodeIndex::new(b), 0)))
    });
}
