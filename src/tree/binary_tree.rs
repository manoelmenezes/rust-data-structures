use crate::tree::Node;

pub struct BinaryTree<T> {
    root: Option<Node<T>>,
}

impl<T: std::cmp::PartialEq + Copy> BinaryTree<T> {

    pub fn new() -> Self {
        Self {
            root: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root == None
    }

    pub fn set_root(&mut self, node: Node<T>) {
        self.root = Some(node);
    }

    pub fn contains(&self, value: T) -> bool {
        if let Some(node) = &self.root {
           node.contains(value)
        } else {
            false
        }
    }
}

