use std::rc::Rc;

#[allow(dead_code)]
#[derive(Debug)]
struct Node {
    id: usize,
    downstream: Option<Rc<Node>>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, downstream: Rc<Node>) {
        self.downstream = Some(downstream)
    }

    pub fn get_downstream(&self) -> Option<Rc<Node>> {
        self.downstream.as_ref().cloned()
    }
}

fn main() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let node4 = Node::new(4);
    node3.update_downstream(Rc::new(node4));

    node1.update_downstream(Rc::new(node3));
    node2.update_downstream(node1.get_downstream().unwrap());
    println!("node1: {:?}, node2: {:?}", node1, node2);

    // 无法编译通过
    // 然而，它无法编译通过，编译器会告诉你“node3 cannot borrow as mutable”。这是因为 Rc 是一个只读的引用计数器，你无法拿到 Rc 结构内部数据的可变引用，来修改这个数据。这可怎么办？这里，我们需要使用 RefCell。
    // let node5 = Node::new(5);
    // let node3 = node1.get_downstream().unwrap();
    // node3.update_downstream(Rc::new(node5));

    // println!("node1: {:?}, node2: {:?}", node1, node2);
}
