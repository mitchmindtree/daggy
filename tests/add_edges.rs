extern crate daggy;

use daggy::{Dag, WouldCycle};
use daggy::NodeIndex;
use std::iter::once;

struct Weight;

#[test]
fn add_edges_ok() {
    let mut dag = Dag::<Weight, u32, u32>::new();
    let root = dag.add_node(Weight);
    let a = dag.add_node(Weight);
    let b = dag.add_node(Weight);
    let c = dag.add_node(Weight);

    let edges = once((root, a, 0))
        .chain(once((root, b, 1)))
        .chain(once((root, c, 2)));
    let mut new_edges = dag.add_edges(edges).unwrap();

    assert_eq!(new_edges.next(), dag.find_edge(root, a));
    assert_eq!(new_edges.next(), dag.find_edge(root, b));
    assert_eq!(new_edges.next(), dag.find_edge(root, c));
}

#[test]
fn add_edges_err() {
    let mut dag = Dag::<Weight, u32, u32>::new();
    let root = dag.add_node(Weight);
    let a = dag.add_node(Weight);
    let b = dag.add_node(Weight);
    let c = dag.add_node(Weight);

    let add_edges_result = dag.add_edges(
        once((root, a, 0))
            .chain(once((root, b, 1)))
            .chain(once((root, c, 2)))
            .chain(once((c, root, 3))),
    );

    match add_edges_result {
        Err(WouldCycle(returned_weights)) => assert_eq!(returned_weights, vec![3, 2, 1, 0]),
        Ok(_) => panic!("Should have been an error"),
    }
}

#[test]
fn add_edges_more() {
    let mut dag = Dag::<Weight, u32, u32>::new();
    let root = dag.add_node(Weight);
    let a = dag.add_node(Weight);
    let b = dag.add_node(Weight);
    let c = dag.add_node(Weight);

    assert!(dag.add_edge(root, a, 0).is_ok());
    assert!(dag.add_edge(root, b, 0).is_ok());
    assert!(dag.add_edge(root, c, 0).is_ok());
    assert!(dag.add_edge(c, root, 0).is_err());
}

#[test]
fn add_edges_more2() {
    let max_node = 10;
    let edges = &[
        (1, 4),
        (3, 4),
        (2, 5),
        (3, 5),
        (2, 8),
        (1, 9),
        (1, 8),
        (1, 3),
        (2, 7),
        (1, 7),
        (0, 6),
        (1, 2),
        (0, 7),
        (1, 6),
        (2, 4),
        (0, 1),
        (0, 9),
        (2, 9),
        (2, 6),
        (0, 4),
        (2, 3),
        (0, 2),
        (0, 3),
        (0, 5),
        (0, 8),
        (1, 5),
    ];
    let mut dag = Dag::<Weight, u32, u32>::with_capacity(max_node, edges.len());
    for _ in 0..max_node {
        dag.add_node(Weight);
    }
    for &(a, b) in edges {
        assert!(
            dag.add_edge(NodeIndex::new(a), NodeIndex::new(b), 0)
                .is_ok()
        );
    }
}
