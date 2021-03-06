use std::fmt::Display;

pub struct BNode<T: Ord> {
    val: T,
    left: ChildNode<T>,
    right: ChildNode<T>,
}
type ChildNode<T: Ord> = Option<Box<BNode<T>>>;

impl<T: Ord> BNode<T> {
    /// Create a new `BNode<T>`
    pub fn new(val: T, left: ChildNode<T>, right: ChildNode<T>) -> Self {
        Self { val, left, right }
    }

    /// Create a new `BNode<T>` containing given value and insert
    /// the node to the left if the value is smaller than current node's value.
    /// insert to the right if the value is bigger than current node's value
    pub fn insert(&mut self, val: T) {
        match val.cmp(&self.val) {
            std::cmp::Ordering::Less => match self.left.as_mut() {
                Some(l) => l.insert(val),
                None => self.left = Some(Box::new(BNode::new(val, None, None))),
            },
            std::cmp::Ordering::Greater => match self.right.as_mut() {
                Some(r) => r.insert(val),
                None => self.right = Some(Box::new(BNode::new(val, None, None))),
            },
            std::cmp::Ordering::Equal => {
                return;
            }
        }
    }

    /// Look for a value by traversing child nodes and
    /// return `true` if given value exists or `false` if not exists
    pub fn find(&self, val: &T) -> bool {
        match val.cmp(&self.val) {
            std::cmp::Ordering::Less => match self.left.as_ref() {
                Some(l) => l.find(val),
                None => false,
            },
            std::cmp::Ordering::Greater => match self.right.as_ref() {
                Some(r) => r.find(val),
                None => false,
            },
            std::cmp::Ordering::Equal => true,
        }
    }
}

impl<T: Ord> BNode<T> {
    /// Return immutable reference to value inside `BNode<T>`
    pub fn val(&self) -> &T {
        &self.val
    }

    /// Return immutable reference to left node if exists, or Return  `None`
    pub fn left(&self) -> Option<&BNode<T>> {
        self.left.as_ref().map(|node| node.as_ref())
    }

    /// Return immutable reference to right node if exists, or Return  `None`
    pub fn right(&self) -> Option<&BNode<T>> {
        self.right.as_ref().map(|node| node.as_ref())
    }

    ///// Return `true` if leaf node and return `false` is not leaf node
    //pub fn is_leaf(&self) -> bool {
    //    self.left.is_none() && self.right.is_none()
    //}

    ///// Return `true` if Node is full and return `false` if not full
    //pub fn is_full(&self) -> bool {
    //    self.left.is_some() && self.right.is_some()
    //}
}

/// traverse left -> self -> right
pub fn traverse_inorder<T: Ord>(node: &BNode<T>) -> Vec<&T> {
    let mut vals = Vec::new();

    if node.left.is_some() {
        vals.extend(traverse_inorder(node.left.as_ref().unwrap()));
    };
    vals.push(node.val());
    if node.right.is_some() {
        vals.extend(traverse_inorder(node.right.as_ref().unwrap()));
    }

    vals
}

/// traverse  self -> left -> right
pub fn traverse_preorder<T: Ord>(node: &BNode<T>) -> Vec<&T> {
    let mut vals = Vec::new();

    vals.push(node.val());
    if node.left.is_some() {
        vals.extend(traverse_preorder(node.left.as_ref().unwrap()));
    };
    if node.right.is_some() {
        vals.extend(traverse_preorder(node.right.as_ref().unwrap()));
    }

    vals
}

/// traverse  left -> right -> self
pub fn traverse_postorder<T: Ord>(node: &BNode<T>) -> Vec<&T> {
    let mut vals = Vec::new();

    if node.left.is_some() {
        vals.extend(traverse_postorder(node.left.as_ref().unwrap()));
    };
    if node.right.is_some() {
        vals.extend(traverse_postorder(node.right.as_ref().unwrap()));
    }
    vals.push(node.val());

    vals
}

struct BinarySearchTree<T: Ord> {
    root: Option<BNode<T>>,
}

impl<T: Ord> BinarySearchTree<T> {
    /// Return root `BNode<T>` if exists, or return `None`
    pub fn root(&self) -> Option<&BNode<T>> {
        self.root.as_ref()
    }
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Create a new `BNode<T>` containing given value and insert
    /// the node to the left if the value is smaller than current node's value.
    /// insert to the right if the value is bigger than current node's value
    pub fn insert(&mut self, val: T) {
        match self.root.as_mut() {
            Some(r) => r.insert(val),
            None => self.root = Some(BNode::new(val, None, None)),
        }
    }

    /// Look for a value by traversing child nodes and
    /// return `true` if given value exists or `false` if not exists
    pub fn find(&mut self, val: &T) -> bool {
        match self.root.as_mut() {
            Some(r) => r.find(val),
            None => false,
        }
    }
}

impl<T: Ord> BinarySearchTree<T> {
    /// traverse left -> self -> right
    pub fn traverse_inorder(&self) -> Vec<&T> {
        match self.root.as_ref() {
            Some(r) => traverse_inorder(r),
            None => Vec::new(),
        }
    }

    /// traverse self -> left -> right
    pub fn traverse_preorder(&self) -> Vec<&T> {
        match self.root.as_ref() {
            Some(r) => traverse_preorder(r),
            None => Vec::new(),
        }
    }

    /// traverse left -> right -> right
    pub fn traverse_postorder(&self) -> Vec<&T> {
        match self.root.as_ref() {
            Some(r) => traverse_postorder(r),
            None => Vec::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::BinarySearchTree;

    #[test]
    fn test_insert() {
        let mut bst = BinarySearchTree::new();
        bst.insert(10); // root
        bst.insert(5); // root - left
        bst.insert(1); // root - left - left
        bst.insert(7); // root - left - right
        bst.insert(20); // root - right
        bst.insert(15); // root - right - left
        bst.insert(30); // root - right - right

        assert_eq!(*bst.root().unwrap().val(), 10);
        assert_eq!(*bst.root().unwrap().left().unwrap().val(), 5);
        assert_eq!(
            *bst.root().unwrap().left().unwrap().left().unwrap().val(),
            1
        );
        assert_eq!(
            *bst.root().unwrap().left().unwrap().right().unwrap().val(),
            7
        );
        assert_eq!(*bst.root().unwrap().right().unwrap().val(), 20);
        assert_eq!(
            *bst.root().unwrap().right().unwrap().left().unwrap().val(),
            15
        );
        assert_eq!(
            *bst.root().unwrap().right().unwrap().right().unwrap().val(),
            30
        );
    }

    #[test]
    fn test_find() {
        let mut bst = BinarySearchTree::new();
        bst.insert(10); // root
        bst.insert(5); // root - left
        bst.insert(1); // root - left - left
        bst.insert(7); // root - left - right
        bst.insert(20); // root - right
        bst.insert(15); // root - right - left
        bst.insert(30); // root - right - right

        assert_eq!(bst.find(&10), true);
        assert_eq!(bst.find(&5), true);
        assert_eq!(bst.find(&1), true);
        assert_eq!(bst.find(&7), true);
        assert_eq!(bst.find(&20), true);
        assert_eq!(bst.find(&15), true);
        assert_eq!(bst.find(&30), true);

        assert_eq!(bst.find(&2), false);
    }

    #[test]
    fn test_traverse_inorder() {
        let mut bst = BinarySearchTree::new();
        bst.insert(10); // root
        bst.insert(5); // root - left
        bst.insert(1); // root - left - left
        bst.insert(7); // root - left - right
        bst.insert(20); // root - right
        bst.insert(15); // root - right - left
        bst.insert(30); // root - right - right

        assert_eq!(bst.traverse_inorder(), vec![&1, &5, &7, &10, &15, &20, &30]);
    }

    #[test]
    fn test_traverse_preorder() {
        let mut bst = BinarySearchTree::new();
        bst.insert(10); // root
        bst.insert(5); // root - left
        bst.insert(1); // root - left - left
        bst.insert(7); // root - left - right
        bst.insert(20); // root - right
        bst.insert(15); // root - right - left
        bst.insert(30); // root - right - right

        assert_eq!(
            bst.traverse_preorder(),
            vec![&10, &5, &1, &7, &20, &15, &30]
        );
    }

    #[test]
    fn test_traverse_postorder() {
        let mut bst = BinarySearchTree::new();
        bst.insert(10); // root
        bst.insert(5); // root - left
        bst.insert(1); // root - left - left
        bst.insert(7); // root - left - right
        bst.insert(20); // root - right
        bst.insert(15); // root - right - left
        bst.insert(30); // root - right - right

        assert_eq!(
            bst.traverse_postorder(),
            vec![&1, &7, &5, &15, &30, &20, &10]
        );
    }
}
