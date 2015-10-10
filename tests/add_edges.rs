
extern crate daggy;

use daggy::{Dag, WouldCycle};
use std::iter::once;

struct Weight;

#[test]
fn add_edges_ok() {

    let mut dag = Dag::<Weight, u32, u32>::new();
    let root = dag.add_node(Weight);
    let a = dag.add_node(Weight);
    let b = dag.add_node(Weight);
    let c = dag.add_node(Weight);

    let mut new_edges = dag.add_edges(once((root, a, 0))
        .chain(once((root, b, 1)))
        .chain(once((root, c, 2))))
        .unwrap();

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

    let add_edges_result = dag.add_edges(once((root, a, 0))
        .chain(once((root, b, 1)))
        .chain(once((root, c, 2)))
        .chain(once((c, root, 3))));

    match add_edges_result {
        Err(WouldCycle(returned_weights)) => assert_eq!(returned_weights, vec![3, 2, 1, 0]),
        Ok(_) => panic!("Should have been an error"),
    }
}
