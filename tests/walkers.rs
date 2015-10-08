
extern crate daggy;

use daggy::Dag;

struct Weight;


#[test]
fn walk_children() {

    let mut dag = Dag::<Weight, Weight, u32>::new();
    let parent = dag.add_node(Weight);
    let (a_e, a_n) = dag.add_child(parent, Weight, Weight);
    let (b_e, b_n) = dag.add_child(parent, Weight, Weight);
    let (c_e, c_n) = dag.add_child(parent, Weight, Weight);

    let mut child_walker = dag.walk_children(parent);
    assert_eq!(Some((c_e, c_n)), child_walker.next_child(&dag));
    assert_eq!(Some((b_e, b_n)), child_walker.next_child(&dag));
    assert_eq!(Some((a_e, a_n)), child_walker.next_child(&dag));
    assert_eq!(None, child_walker.next(&dag));

    let (d_e, d_n) = dag.add_child(b_n, Weight, Weight);
    let (e_e, e_n) = dag.add_child(b_n, Weight, Weight);
    let (f_e, f_n) = dag.add_child(b_n, Weight, Weight);

    child_walker = dag.walk_children(b_n);
    assert_eq!(Some((f_e, f_n)), child_walker.next_child(&dag));
    assert_eq!(Some((e_e, e_n)), child_walker.next_child(&dag));
    assert_eq!(Some((d_e, d_n)), child_walker.next_child(&dag));
    assert_eq!(None, child_walker.next_child(&dag));
}


#[test]
fn walk_parents() {

    let mut dag = Dag::<Weight, Weight, u32>::new();
    let child = dag.add_node(Weight);
    let (a_e, a_n) = dag.add_parent(child, Weight, Weight);
    let (b_e, b_n) = dag.add_parent(child, Weight, Weight);
    let (c_e, c_n) = dag.add_parent(child, Weight, Weight);
    let (d_e, d_n) = dag.add_parent(child, Weight, Weight);

    let mut parent_walker = dag.walk_parents(child);
    assert_eq!(Some((d_e, d_n)), parent_walker.next_parent(&dag));
    assert_eq!(Some((c_e, c_n)), parent_walker.next_parent(&dag));
    assert_eq!(Some((b_e, b_n)), parent_walker.next_parent(&dag));
    assert_eq!(Some((a_e, a_n)), parent_walker.next_parent(&dag));
    assert_eq!(None, parent_walker.next_parent(&dag));
}


