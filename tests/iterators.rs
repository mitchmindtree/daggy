
extern crate daggy;

use daggy::Dag;

struct Weight;


#[test]
fn children() {

    let mut dag = Dag::<Weight, Weight, u32>::new();
    let parent = dag.add_node(Weight);
    let (_, a) = dag.add_child(parent, Weight, Weight);
    let (_, b) = dag.add_child(parent, Weight, Weight);
    let (_, c) = dag.add_child(parent, Weight, Weight);

    {
        let mut children = dag.children(parent);
        assert_eq!(Some(c), children.next());
        assert_eq!(Some(b), children.next());
        assert_eq!(Some(a), children.next());
        assert_eq!(None, children.next());
    }

    let (_, d) = dag.add_child(b, Weight, Weight);
    let (_, e) = dag.add_child(b, Weight, Weight);
    let (_, f) = dag.add_child(b, Weight, Weight);
    {
        let mut children = dag.children(b);
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
    let (_, a) = dag.add_parent(child, Weight, Weight);
    let (_, b) = dag.add_parent(child, Weight, Weight);
    let (_, c) = dag.add_parent(child, Weight, Weight);
    let (_, d) = dag.add_parent(child, Weight, Weight);

    {
        let mut parents = dag.parents(child);
        assert_eq!(Some(d), parents.next());
        assert_eq!(Some(c), parents.next());
        assert_eq!(Some(b), parents.next());
        assert_eq!(Some(a), parents.next());
        assert_eq!(None, parents.next());
    }
}



