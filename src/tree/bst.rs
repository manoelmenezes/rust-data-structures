use crate::tree::Node;

pub struct BST<T> {
    root: Option<Node<T>>,
}

impl<T: std::cmp::PartialOrd + std::cmp::PartialEq + Copy> BST<T> {
    pub fn new() -> Self {
        Self {
            root: None,
        }
    }

    pub fn as_vec(&self) -> Vec<T> {
        let mut result = vec![];
        if let Some(node) = &self.root {
            Self::add_to_vec(node, &mut result);
        }
        return result;
    }

    fn add_to_vec(node: &Node<T>, v: &mut Vec<T>) {
        if let Some(l) = &node.left {
            Self::add_to_vec(&*l, v);
        }
        v.push(node.value);
        if let Some(r) = &node.right {
            Self::add_to_vec(&*r, v);
        }
    }

    pub fn insert(&mut self, value: T) {
        if let Some(node) = &mut self.root {
            Self::insert_node(node, value);
        } else {
            self.root = Some(Node::leaf(value));
        }
    }

    fn insert_node(node: &mut Node<T>, value: T) {
        if node.value == value {
            return;
        }
        if node.value < value {
            if let Some(r) = &mut node.right {
                Self::insert_node(&mut *r, value);
            } else {
                node.right = Some(Box::new(Node::leaf(value)));
            }
        } else {
            if let Some(l) = &mut node.left {
                Self::insert_node(&mut *l, value);
            } else {
                node.left = Some(Box::new(Node::leaf(value)));
            }
        }
    }
}

