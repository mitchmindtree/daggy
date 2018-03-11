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
    assert_eq!(Some((c_e, c_n)), child_walker.walk_next(&dag));
    assert_eq!(Some(b_e), child_walker.walk_next(&dag).map(|(e, _)| e));
    assert_eq!(Some(a_n), child_walker.walk_next(&dag).map(|(_, n)| n));
    assert_eq!(None, child_walker.walk_next(&dag));

    let (d_e, d_n) = dag.add_child(b_n, Weight, Weight);
    let (e_e, e_n) = dag.add_child(b_n, Weight, Weight);
    let (f_e, f_n) = dag.add_child(b_n, Weight, Weight);

    child_walker = dag.children(b_n);
    assert_eq!(Some((f_e, f_n)), child_walker.walk_next(&dag));
    assert_eq!(Some((e_e, e_n)), child_walker.walk_next(&dag));
    assert_eq!(Some((d_e, d_n)), child_walker.walk_next(&dag));
    assert_eq!(None, child_walker.walk_next(&dag));
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
    assert_eq!(Some((d_e, d_n)), parent_walker.walk_next(&dag));
    assert_eq!(Some((c_e, c_n)), parent_walker.walk_next(&dag));
    assert_eq!(Some((b_e, b_n)), parent_walker.walk_next(&dag));
    assert_eq!(Some((a_e, a_n)), parent_walker.walk_next(&dag));
    assert_eq!(None, parent_walker.walk_next(&dag));
}

#[test]
fn count() {
    let mut dag = Dag::<Weight, Weight>::new();
    let parent = dag.add_node(Weight);
    dag.add_child(parent, Weight, Weight);
    dag.add_child(parent, Weight, Weight);
    dag.add_child(parent, Weight, Weight);
    assert_eq!(3, dag.children(parent).iter(&dag).count());
}

#[test]
fn last() {
    let mut dag = Dag::<Weight, Weight>::new();
    let parent = dag.add_node(Weight);
    let (last_e, last_n) = dag.add_child(parent, Weight, Weight);
    dag.add_child(parent, Weight, Weight);
    dag.add_child(parent, Weight, Weight);
    assert_eq!(
        Some((last_e, last_n)),
        dag.children(parent).iter(&dag).last()
    );
}

#[test]
fn nth() {
    let mut dag = Dag::<Weight, Weight>::new();
    let parent = dag.add_node(Weight);
    let (e_at_2, n_at_2) = dag.add_child(parent, Weight, Weight);
    let (_, _) = dag.add_child(parent, Weight, Weight);
    let (_, _) = dag.add_child(parent, Weight, Weight);
    assert_eq!(None, dag.children(parent).iter(&dag).nth(3));
    assert_eq!(
        Some((e_at_2, n_at_2)),
        dag.children(parent).iter(&dag).nth(2)
    );
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

    let mut chain = daggy::walker::Chain::new(dag.children(c), dag.children(a));
    assert_eq!(Some(f), chain.walk_next(&dag).map(|(_, n)| n));
    assert_eq!(Some(e), chain.walk_next(&dag).map(|(_, n)| n));
    assert_eq!(Some(d), chain.walk_next(&dag).map(|(_, n)| n));
    assert_eq!(Some(c), chain.walk_next(&dag).map(|(_, n)| n));
    assert_eq!(Some(b), chain.walk_next(&dag).map(|(_, n)| n));
    assert_eq!(None, chain.walk_next(&dag).map(|(_, n)| n));
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

    let children = dag.children(parent);
    let mut even_children = daggy::walker::Filter::new(children, |g, &(_, n)| g[n] % 2 == 0);
    assert_eq!(
        4,
        dag[even_children.walk_next(&dag).map(|(_, n)| n).unwrap()]
    );
    assert_eq!(
        2,
        dag[even_children.walk_next(&dag).map(|(_, n)| n).unwrap()]
    );
    assert_eq!(
        0,
        dag[even_children.walk_next(&dag).map(|(_, n)| n).unwrap()]
    );
    assert!(even_children.walk_next(&dag).is_none());
}

#[test]
fn peekable() {
    let mut dag = Dag::<Weight, Weight>::new();
    let parent = dag.add_node(Weight);
    let (_, a) = dag.add_child(parent, Weight, Weight);
    let (_, b) = dag.add_child(parent, Weight, Weight);
    let (_, c) = dag.add_child(parent, Weight, Weight);

    let children = dag.children(parent);
    let mut children = daggy::walker::Peekable::new(children);
    assert_eq!(Some(c), children.peek(&dag).map(|&(_, n)| n));
    assert_eq!(Some(c), children.walk_next(&dag).map(|(_, n)| n));
    assert_eq!(Some(b), children.walk_next(&dag).map(|(_, n)| n));
    assert_eq!(Some(a), children.peek(&dag).map(|&(_, n)| n));
    assert_eq!(Some(a), children.peek(&dag).map(|&(_, n)| n));
    assert_eq!(Some(a), children.walk_next(&dag).map(|(_, n)| n));
    assert!(children.peek(&dag).is_none());
    assert!(children.walk_next(&dag).is_none());
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

    let children = dag.children(parent);
    let mut children_under_3 = daggy::walker::SkipWhile::new(children, |g, &(_, n)| g[n] >= 3);
    assert_eq!(
        2,
        dag[children_under_3.walk_next(&dag).map(|(_, n)| n).unwrap()]
    );
    assert_eq!(
        1,
        dag[children_under_3.walk_next(&dag).map(|(_, n)| n).unwrap()]
    );
    assert_eq!(
        0,
        dag[children_under_3.walk_next(&dag).map(|(_, n)| n).unwrap()]
    );
    assert!(children_under_3.walk_next(&dag).is_none());
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

    let children = dag.children(parent);
    let mut children_over_2 = daggy::walker::TakeWhile::new(children, |g, &(_, n)| g[n] > 2);
    assert_eq!(
        5,
        dag[children_over_2.walk_next(&dag).map(|(_, n)| n).unwrap()]
    );
    assert_eq!(
        4,
        dag[children_over_2.walk_next(&dag).map(|(_, n)| n).unwrap()]
    );
    assert_eq!(
        3,
        dag[children_over_2.walk_next(&dag).map(|(_, n)| n).unwrap()]
    );
    assert!(children_over_2.walk_next(&dag).is_none());
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

    let children = dag.children(parent);
    let mut children = daggy::walker::Skip::new(children, 3);
    assert_eq!(2, dag[children.walk_next(&dag).map(|(_, n)| n).unwrap()]);
    assert_eq!(1, dag[children.walk_next(&dag).map(|(_, n)| n).unwrap()]);
    assert_eq!(0, dag[children.walk_next(&dag).map(|(_, n)| n).unwrap()]);
    assert!(children.walk_next(&dag).is_none());
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

    let children = dag.children(parent);
    let mut children = daggy::walker::Take::new(children, 3);
    assert_eq!(5, dag[children.walk_next(&dag).map(|(_, n)| n).unwrap()]);
    assert_eq!(4, dag[children.walk_next(&dag).map(|(_, n)| n).unwrap()]);
    assert_eq!(3, dag[children.walk_next(&dag).map(|(_, n)| n).unwrap()]);
    assert!(children.walk_next(&dag).is_none());
}

#[test]
fn all() {
    let mut dag = Dag::<i32, ()>::new();
    let parent = dag.add_node(0);
    dag.add_child(parent, (), 0);
    dag.add_child(parent, (), 2);
    dag.add_child(parent, (), 4);

    let mut children = dag.children(parent);
    assert!(children.iter(&dag).all(|(_, n)| dag[n] % 2 == 0));

    dag.add_child(parent, (), 7);
    children = dag.children(parent);
    assert!(!children.iter(&dag).all(|(_, n)| dag[n] % 2 == 0));
}

#[test]
fn any() {
    let mut dag = Dag::<i32, ()>::new();
    let parent = dag.add_node(0);
    dag.add_child(parent, (), 1);
    dag.add_child(parent, (), 3);
    dag.add_child(parent, (), 5);

    assert!(!dag.children(parent)
        .iter(&dag)
        .any(|(_, n)| dag[n] % 2 == 0));

    dag.add_child(parent, (), 6);

    assert!(
        dag.children(parent)
            .iter(&dag)
            .any(|(_, n)| dag[n] % 2 == 0)
    );
}

#[test]
fn find() {
    let mut dag = Dag::<i32, ()>::new();
    let parent = dag.add_node(0);
    dag.add_child(parent, (), 1);
    dag.add_child(parent, (), 3);
    dag.add_child(parent, (), 5);

    assert_eq!(
        None,
        dag.children(parent)
            .iter(&dag)
            .find(|&(_, n)| dag[n] % 2 == 0)
    );

    let (e, n) = dag.add_child(parent, (), 4);

    assert_eq!(
        Some((e, n)),
        dag.children(parent)
            .iter(&dag)
            .find(|&(_, n)| dag[n] % 2 == 0)
    );
}

#[test]
fn fold() {
    let mut dag = Dag::<i32, i32>::new();
    let parent = dag.add_node(0);
    dag.add_child(parent, 1, 1);
    dag.add_child(parent, 2, 2);
    dag.add_child(parent, 3, 3);

    assert_eq!(
        12,
        dag.children(parent)
            .iter(&dag)
            .fold(0, |acc, (e, n)| acc + dag[e] + dag[n])
    );
}

#[test]
fn recursive_walk() {
    let mut dag = Dag::<i32, i32>::new();
    let grand_parent = dag.add_node(0);
    let (_, parent) = dag.add_child(grand_parent, 0, 0);
    let (_, child) = dag.add_child(parent, 0, 0);

    let mut parent_recursion = dag.recursive_walk(child, |g, n| {
        g.parents(n).iter(g).find(|&(e, n)| g[e] == 0 && g[n] == 0)
    });
    assert_eq!(
        Some(parent),
        parent_recursion.walk_next(&dag).map(|(_, n)| n)
    );
    assert_eq!(
        Some(grand_parent),
        parent_recursion.walk_next(&dag).map(|(_, n)| n)
    );
    assert_eq!(None, parent_recursion.walk_next(&dag));
}
