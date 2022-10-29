#[derive(Default, Debug)]
struct Node {
    name: String,
    children: Option<Vec<Node>>,
}

impl Node {
    fn new(name: String) -> Self {
        Node { name, children: None}
    }

    fn with_children(name: String, nodes: Vec<Node>) -> Self {
        Node { name, children: Some(nodes) }
    }
}

#[test]
fn test_node(){
    let top = Node::with_children(
        "top".into(), vec![Node::new(
            "a".into()
        ), Node::with_children(
            "b".into(),
            vec![Node::new(
                "c".into()
            ), Node::new(
                "d".into()
            )]
        )]
    );
    println!("{:?}", top);
}

