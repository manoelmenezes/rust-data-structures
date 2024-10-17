#[derive(PartialEq)]
pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: std::cmp::PartialEq + Copy> Node<T> {

    pub fn complete(value: T, left: Node<T>, right: Node<T>) -> Self {
        Self {
            value,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    } 

    pub fn leaf(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    pub fn with_left(value: T, left: Node<T>) -> Self {
        Self {
            value,
            left: Some(Box::new(left)),
            right: None,
        }
    }

    pub fn with_right(value: T, right: Node<T>) -> Self {
        Self {
            value,
            left: None,
            right: Some(Box::new(right)),
        }
    }

    pub fn contains(&self, value: T) -> bool {
        if value == self.value {
            return true;
        }

        if let Some(node) = &self.left {
           if node.contains(value) {
               return true;
           }
        }

        if let Some(node) = &self.right {
            if  node.contains(value) {
                return true;
            }
        }

        false
    }

}

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

