extern crate daggy;

use daggy::Dag;

#[test]
fn transitive_reduce() {
    let mut dag = Dag::<&str, &str>::new();

    // construct example DAG from wikipedia

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

    assert_eq!(dag.edge_count(), 5);

    // test case where the alternate route from the parent is more than one node long

    dag.add_edge(a, e, "a->e").unwrap();

    assert_eq!(dag.edge_count(), 6);

    dag.transitive_reduce(vec![a]);

    assert_eq!(dag.edge_count(), 5);
}
