#[derive(Debug)]
pub(crate) struct Tree<T> {
    root: Option<Box<Node<T>>>,
}
#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

// per l into Box<Node> serve il From
impl<T> From<Node<T>> for Option<Box<Node<T>>> {
    fn from(node: Node<T>) -> Self {
        Some(Box::new(node))
    }
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Tree { root: None }
    }
    pub fn insert(&mut self, value: T)
    where
        T: PartialOrd,
    {
        match &mut self.root {
            None => self.root = Node::new(value).into(),
            Some(node) => Tree::insert_recursively(node, value),
        }
    }
    
    
    fn insert_recursively(node: &mut Box<Node<T>>, value: T)
    where
        T: PartialOrd,
    {
        if value <= node.value {
            match &mut node.left {
                None => node.left = Node::new(value).into(),
                Some(left_node) => Tree::insert_recursively(left_node, value),
            }
            return;
        }

        match &mut node.right {
            None => node.right = Node::new(value).into(),
            Some(right_node) => Tree::insert_recursively(right_node, value),
        }
    }
}
