
extern crate daggy;

use daggy::{Dag, Walker};

#[derive(Copy, Clone, Debug)]
struct Weight;


#[test]
fn children() {

    let mut dag = Dag::<Weight, Weight>::new();
    let parent = dag.add_node(Weight);
    let (_, a_n) = dag.add_child(parent, Weight, Weight);
    let (b_e, b_n) = dag.add_child(parent, Weight, Weight);
    let (c_e, c_n) = dag.add_child(parent, Weight, Weight);

    let mut child_walker = dag.children(parent);
    assert_eq!(Some((c_e, c_n)), child_walker.next(&dag));
    assert_eq!(Some(b_e), child_walker.next_edge(&dag));
    assert_eq!(Some(a_n), child_walker.next_node(&dag));
    assert_eq!(None, child_walker.next(&dag));

    let (d_e, d_n) = dag.add_child(b_n, Weight, Weight);
    let (e_e, e_n) = dag.add_child(b_n, Weight, Weight);
    let (f_e, f_n) = dag.add_child(b_n, Weight, Weight);

    child_walker = dag.children(b_n);
    assert_eq!(Some((f_e, f_n)), child_walker.next(&dag));
    assert_eq!(Some((e_e, e_n)), child_walker.next(&dag));
    assert_eq!(Some((d_e, d_n)), child_walker.next(&dag));
    assert_eq!(None, child_walker.next(&dag));

}


#[test]
fn parents() {

    let mut dag = Dag::<Weight, Weight>::new();
    let child = dag.add_node(Weight);
    let (a_e, a_n) = dag.add_parent(child, Weight, Weight);
    let (b_e, b_n) = dag.add_parent(child, Weight, Weight);
    let (c_e, c_n) = dag.add_parent(child, Weight, Weight);
    let (d_e, d_n) = dag.add_parent(child, Weight, Weight);

    let mut parent_walker = dag.parents(child);
    assert_eq!(Some((d_e, d_n)), parent_walker.next(&dag));
    assert_eq!(Some((c_e, c_n)), parent_walker.next(&dag));
    assert_eq!(Some((b_e, b_n)), parent_walker.next(&dag));
    assert_eq!(Some((a_e, a_n)), parent_walker.next(&dag));
    assert_eq!(None, parent_walker.next(&dag));
}


#[test]
fn count() {
    let mut dag = Dag::<Weight, Weight>::new();
    let parent = dag.add_node(Weight);
    dag.add_child(parent, Weight, Weight);
    dag.add_child(parent, Weight, Weight);
    dag.add_child(parent, Weight, Weight);

    assert_eq!(3, dag.children(parent).count(&dag));
}


#[test]
fn last() {
    let mut dag = Dag::<Weight, Weight>::new();
    let parent = dag.add_node(Weight);
    let (last_e, last_n) = dag.add_child(parent, Weight, Weight);
    dag.add_child(parent, Weight, Weight);
    dag.add_child(parent, Weight, Weight);

    assert_eq!(Some((last_e, last_n)), dag.children(parent).last(&dag));
    assert_eq!(Some(last_e), dag.children(parent).last_edge(&dag));
    assert_eq!(Some(last_n), dag.children(parent).last_node(&dag));
}


#[test]
fn nth() {
    let mut dag = Dag::<Weight, Weight>::new();
    let parent = dag.add_node(Weight);
    let (e_at_2, n_at_2) = dag.add_child(parent, Weight, Weight);
    let (e_at_1, _) = dag.add_child(parent, Weight, Weight);
    let (_, n_at_0) = dag.add_child(parent, Weight, Weight);

    assert_eq!(None, dag.children(parent).nth(&dag, 3));
    assert_eq!(Some((e_at_2, n_at_2)), dag.children(parent).nth(&dag, 2));
    assert_eq!(Some(e_at_1), dag.children(parent).nth_edge(&dag, 1));
    assert_eq!(Some(n_at_0), dag.children(parent).nth_node(&dag, 0));
}


#[test]
fn chain() {
    let mut dag = Dag::<Weight, Weight>::new();
    let a = dag.add_node(Weight);
    let (_, b) = dag.add_child(a, Weight, Weight);
    let (_, c) = dag.add_child(a, Weight, Weight);
    let (_, d) = dag.add_child(a, Weight, Weight);
    let (_, e) = dag.add_child(c, Weight, Weight);
    let (_, f) = dag.add_child(c, Weight, Weight);

    let mut chain = dag.children(c).chain(dag.children(a));
    assert_eq!(Some(f), chain.next_node(&dag));
    assert_eq!(Some(e), chain.next_node(&dag));
    assert_eq!(Some(d), chain.next_node(&dag));
    assert_eq!(Some(c), chain.next_node(&dag));
    assert_eq!(Some(b), chain.next_node(&dag));
    assert_eq!(None, chain.next_node(&dag));
}


#[test]
fn filter() {
    let mut dag = Dag::<i32, ()>::new();
    let parent = dag.add_node(0);
    dag.add_child(parent, (), 0);
    dag.add_child(parent, (), 1);
    dag.add_child(parent, (), 2);
    dag.add_child(parent, (), 3);
    dag.add_child(parent, (), 4);
    dag.add_child(parent, (), 5);

    let mut even_children = dag.children(parent).filter(|g, _, n| g[n] % 2 == 0);
    assert_eq!(4, dag[even_children.next_node(&dag).unwrap()]);
    assert_eq!(2, dag[even_children.next_node(&dag).unwrap()]);
    assert_eq!(0, dag[even_children.next_node(&dag).unwrap()]);
    assert!(even_children.next(&dag).is_none());
}


#[test]
fn peekable() {
    let mut dag = Dag::<Weight, Weight>::new();
    let parent = dag.add_node(Weight);
    let (_, a) = dag.add_child(parent, Weight, Weight);
    let (_, b) = dag.add_child(parent, Weight, Weight);
    let (_, c) = dag.add_child(parent, Weight, Weight);

    let mut children = dag.children(parent).peekable();
    assert_eq!(Some(c), children.peek_node(&dag));
    assert_eq!(Some(c), children.next_node(&dag));
    assert_eq!(Some(b), children.next_node(&dag));
    assert_eq!(Some(a), children.peek_node(&dag));
    assert_eq!(Some(a), children.peek_node(&dag));
    assert_eq!(Some(a), children.next_node(&dag));
    assert!(children.peek(&dag).is_none());
    assert!(children.next(&dag).is_none());
}


#[test]
fn skip_while() {
    let mut dag = Dag::<i32, ()>::new();
    let parent = dag.add_node(0);
    dag.add_child(parent, (), 0);
    dag.add_child(parent, (), 1);
    dag.add_child(parent, (), 2);
    dag.add_child(parent, (), 3);
    dag.add_child(parent, (), 4);
    dag.add_child(parent, (), 5);

    let mut children_under_3 = dag.children(parent).skip_while(|g, _, n| g[n] >= 3);
    assert_eq!(2, dag[children_under_3.next_node(&dag).unwrap()]);
    assert_eq!(1, dag[children_under_3.next_node(&dag).unwrap()]);
    assert_eq!(0, dag[children_under_3.next_node(&dag).unwrap()]);
    assert!(children_under_3.next(&dag).is_none());
}


#[test]
fn take_while() {
    let mut dag = Dag::<i32, ()>::new();
    let parent = dag.add_node(0);
    dag.add_child(parent, (), 0);
    dag.add_child(parent, (), 1);
    dag.add_child(parent, (), 2);
    dag.add_child(parent, (), 3);
    dag.add_child(parent, (), 4);
    dag.add_child(parent, (), 5);

    let mut children_over_2 = dag.children(parent).take_while(|g, _, n| g[n] > 2);
    assert_eq!(5, dag[children_over_2.next_node(&dag).unwrap()]);
    assert_eq!(4, dag[children_over_2.next_node(&dag).unwrap()]);
    assert_eq!(3, dag[children_over_2.next_node(&dag).unwrap()]);
    assert!(children_over_2.next(&dag).is_none());
}


#[test]
fn skip() {
    let mut dag = Dag::<i32, ()>::new();
    let parent = dag.add_node(0);
    dag.add_child(parent, (), 0);
    dag.add_child(parent, (), 1);
    dag.add_child(parent, (), 2);
    dag.add_child(parent, (), 3);
    dag.add_child(parent, (), 4);
    dag.add_child(parent, (), 5);

    let mut children = dag.children(parent).skip(3);
    assert_eq!(2, dag[children.next_node(&dag).unwrap()]);
    assert_eq!(1, dag[children.next_node(&dag).unwrap()]);
    assert_eq!(0, dag[children.next_node(&dag).unwrap()]);
    assert!(children.next(&dag).is_none());
}


#[test]
fn take() {
    let mut dag = Dag::<i32, ()>::new();
    let parent = dag.add_node(0);
    dag.add_child(parent, (), 0);
    dag.add_child(parent, (), 1);
    dag.add_child(parent, (), 2);
    dag.add_child(parent, (), 3);
    dag.add_child(parent, (), 4);
    dag.add_child(parent, (), 5);

    let mut children = dag.children(parent).take(3);
    assert_eq!(5, dag[children.next_node(&dag).unwrap()]);
    assert_eq!(4, dag[children.next_node(&dag).unwrap()]);
    assert_eq!(3, dag[children.next_node(&dag).unwrap()]);
    assert!(children.next(&dag).is_none());
}


#[test]
fn all() {
    let mut dag = Dag::<i32, ()>::new();
    let parent = dag.add_node(0);
    dag.add_child(parent, (), 0);
    dag.add_child(parent, (), 2);
    dag.add_child(parent, (), 4);

    let mut children = dag.children(parent);
    assert!(children.all(&dag, |g, _, n| g[n] % 2 == 0));

    dag.add_child(parent, (), 7);
    children = dag.children(parent);
    assert!(!children.all(&dag, |g, _, n| g[n] % 2 == 0));
}


#[test]
fn any() {
    let mut dag = Dag::<i32, ()>::new();
    let parent = dag.add_node(0);
    dag.add_child(parent, (), 1);
    dag.add_child(parent, (), 3);
    dag.add_child(parent, (), 5);

    assert!(!dag.children(parent).any(&dag, |g, _, n| g[n] % 2 == 0));

    dag.add_child(parent, (), 6);

    assert!(dag.children(parent).any(&dag, |g, _, n| g[n] % 2 == 0));
}


#[test]
fn find() {
    let mut dag = Dag::<i32, ()>::new();
    let parent = dag.add_node(0);
    dag.add_child(parent, (), 1);
    dag.add_child(parent, (), 3);
    dag.add_child(parent, (), 5);

    assert_eq!(None, dag.children(parent).find(&dag, |g, _, n| g[n] % 2 == 0));

    let (e, n) = dag.add_child(parent, (), 4);

    assert_eq!(Some((e, n)), dag.children(parent).find(&dag, |g, _, n| g[n] % 2 == 0));
}


#[test]
fn fold() {
    let mut dag = Dag::<i32, i32>::new();
    let parent = dag.add_node(0);
    dag.add_child(parent, 1, 1);
    dag.add_child(parent, 2, 2);
    dag.add_child(parent, 3, 3);

    assert_eq!(12, dag.children(parent).fold(0, &dag, |acc, g, e, n| acc + g[e] + g[n]));
}


#[test]
fn recursive_walk() {
    let mut dag = Dag::<i32, i32>::new();
    let grand_parent = dag.add_node(0);
    let (_, parent) = dag.add_child(grand_parent, 0, 0);
    let (_, child) = dag.add_child(parent, 0, 0);

    let mut parent_recursion = dag.recursive_walk(child, |g, n| {
        g.parents(n).find(g, |g, e, n| g[e] == 0 && g[n] == 0)
    });
    assert_eq!(Some(parent), parent_recursion.next_node(&dag));
    assert_eq!(Some(grand_parent), parent_recursion.next_node(&dag));
    assert_eq!(None, parent_recursion.next(&dag));
}
