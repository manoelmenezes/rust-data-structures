pub mod binary_tree;
pub mod bst;

#[derive(PartialEq, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaf() {
        let leaf = Node::<i32>::leaf(1);
        assert_eq!(leaf.left, None);
        assert_eq!(leaf.right, None);
        assert_eq!(leaf.value, 1);
    }

    #[test]
    fn with_left() {
        let node = Node::<i32>::with_left(1, Node{value:0, left:None, right: None});
        assert_eq!(node.value, 1);
        assert_eq!(node.right, None);

        let left = node.left.unwrap();
        assert_eq!(left.value, 0);
        assert_eq!(left.left, None);
        assert_eq!(left.right, None);
    }    


    #[test]
    fn with_right() {
        let node = Node::<i32>::with_right(1, Node{value:0, left:None, right: None});
        assert_eq!(node.value, 1);
        assert_eq!(node.left, None);

        let right = node.right.unwrap();
        assert_eq!(right.value, 0);
        assert_eq!(right.left, None);
        assert_eq!(right.right, None);
    }
    
    #[test]
    fn complete() {
        let node = Node::<i32>::complete(
            0,
            Node{value:-1, left:None, right: None},
            Node{value:1, left:None, right: None},
        );
        assert_eq!(node.value, 0);

        let left = node.left.unwrap();
        assert_eq!(left.value, -1);
        assert_eq!(left.left, None);
        assert_eq!(left.right, None);
        
        let right = node.right.unwrap();
        assert_eq!(right.value, 1);
        assert_eq!(right.left, None);
        assert_eq!(right.right, None);
    }

    #[test]
    fn contains() {
        let node = Node::<i32>::complete(
            0,
            Node{value:-1, left:None, right: None},
            Node{value:1, left:None, right: None},
        );
        assert!(node.contains(0));
        assert!(node.contains(-1));
        assert!(node.contains(1));
        assert!(!node.contains(10));
    }    
}

