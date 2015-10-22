
extern crate daggy;

use daggy::{Dag, Walker};

struct Weight;


#[test]
fn children() {

    let mut dag = Dag::<Weight, Weight, u32>::new();
    let parent = dag.add_node(Weight);
    let (_, a) = dag.add_child(parent, Weight, Weight);
    let (_, b) = dag.add_child(parent, Weight, Weight);
    let (_, c) = dag.add_child(parent, Weight, Weight);

    {
        let mut children = dag.children(parent).iter(&dag).nodes();
        assert_eq!(Some(c), children.next());
        assert_eq!(Some(b), children.next());
        assert_eq!(Some(a), children.next());
        assert_eq!(None, children.next());
    }

    let (d, _) = dag.add_child(b, Weight, Weight);
    let (e, _) = dag.add_child(b, Weight, Weight);
    let (f, _) = dag.add_child(b, Weight, Weight);
    {
        let mut children = dag.children(b).iter(&dag).edges();
        assert_eq!(Some(f), children.next());
        assert_eq!(Some(e), children.next());
        assert_eq!(Some(d), children.next());
        assert_eq!(None, children.next());
    }
}

#[test]
fn parents() {

    let mut dag = Dag::<Weight, Weight, u32>::new();
    let child = dag.add_node(Weight);
    let (a_e, a_n) = dag.add_parent(child, Weight, Weight);
    let (b_e, b_n) = dag.add_parent(child, Weight, Weight);
    let (c_e, c_n) = dag.add_parent(child, Weight, Weight);
    let (d_e, d_n) = dag.add_parent(child, Weight, Weight);

    {
        let mut parents = dag.parents(child).iter(&dag);
        assert_eq!(Some((d_e, d_n)), parents.next());
        assert_eq!(Some((c_e, c_n)), parents.next());
        assert_eq!(Some((b_e, b_n)), parents.next());
        assert_eq!(Some((a_e, a_n)), parents.next());
        assert_eq!(None, parents.next());
    }

}

#[test]
fn weights() {

    let mut dag = Dag::<&str, i32, u32>::new();
    let parent = dag.add_node("0");
    dag.add_child(parent, 1, "1");
    dag.add_child(parent, 2, "2");
    dag.add_child(parent, 3, "3");

    {
        let mut children = dag.children(parent).iter_weights(&dag);
        assert_eq!(Some((&3, &"3")), children.next());
        assert_eq!(Some((&2, &"2")), children.next());
        assert_eq!(Some((&1, &"1")), children.next());
        assert_eq!(None, children.next());
    }

    {
        let mut child_edges = dag.children(parent).iter_weights(&dag).edges();
        assert_eq!(Some(&3), child_edges.next());
        assert_eq!(Some(&2), child_edges.next());
        assert_eq!(Some(&1), child_edges.next());
        assert_eq!(None, child_edges.next());
    }

    {
        let mut child_nodes = dag.children(parent).iter_weights(&dag).nodes();
        assert_eq!(Some(&"3"), child_nodes.next());
        assert_eq!(Some(&"2"), child_nodes.next());
        assert_eq!(Some(&"1"), child_nodes.next());
        assert_eq!(None, child_nodes.next());
    }

}


