#![cfg(feature = "stable_dag")]
extern crate daggy;

use daggy::stable_dag::StableDag;

#[test]
pub fn remove_nodes() {
    let mut dag = StableDag::<u32, u32, u32>::new();
    let root = dag.add_node(0);
    let a = dag.add_node(1);
    let b = dag.add_node(2);
    let c = dag.add_node(3);

    dag.remove_node(b);

    assert_eq!(Some(&0), dag.node_weight(root));
    assert_eq!(Some(&1), dag.node_weight(a));
    assert_eq!(None, dag.node_weight(b));
    assert_eq!(Some(&3), dag.node_weight(c));
}

#[test]
fn remove_edges() {
    let mut dag = StableDag::<u32, u32, u32>::new();
    let root = dag.add_node(0);
    let a = dag.add_node(1);
    let b = dag.add_node(2);
    let c = dag.add_node(3);

    let e_a = dag.add_edge(root, a, 0).unwrap();
    let e_b = dag.add_edge(root, b, 1).unwrap();
    let e_c = dag.add_edge(root, c, 2).unwrap();

    dag.remove_edge(e_b);

    assert_eq!(Some(&0), dag.edge_weight(e_a));
    assert_eq!(None, dag.edge_weight(e_b));
    assert_eq!(Some(&2), dag.edge_weight(e_c));
}
