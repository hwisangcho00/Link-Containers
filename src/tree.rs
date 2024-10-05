#[allow(unused_imports)]
use std::{cmp::Ord, mem};

#[derive(Clone, Debug)]
pub enum TreeNode<T: Ord> {
    Leaf,
    Node(T, Box<TreeNode<T>>, Box<TreeNode<T>>),
}

// Provided functions
impl<T: Ord> TreeNode<T> {
    pub fn height(&self) -> usize {
        match self {
            TreeNode::Leaf => 0,
            TreeNode::Node(_, left, right) => 1 + std::cmp::max(left.height(), right.height()),
        }
    }

    /// Verifies that the tree is a binary search tree
    fn is_bst(&self) -> bool {
        fn is_bst_helper<T: Ord>(tree: &TreeNode<T>, min: Option<&T>, max: Option<&T>) -> bool {
            match tree {
                TreeNode::Leaf => true,
                TreeNode::Node(value, left, right) => {
                    match min {
                        Some(min) => {
                            if value <= min {
                                return false;
                            }
                        }
                        _ => {}
                    }
                    match max {
                        Some(max) => {
                            if value >= max {
                                return false;
                            }
                        }
                        _ => {}
                    }
                    is_bst_helper(left, min, Some(value)) && is_bst_helper(right, Some(value), max)
                }
            }
        }
        is_bst_helper(self, None, None)
    }

    /// Verifies that the tree is balanced
    pub fn is_balanced(&self) -> bool {
        match self {
            TreeNode::Leaf => true,
            TreeNode::Node(_, left, right) => {
                let left_height = left.height();
                let right_height = right.height();
                let diff = (left_height as i32 - right_height as i32).abs();
                diff <= 1 && left.is_balanced() && right.is_balanced()
            }
        }
    }

    /// Verifies that the tree is a valid balanced binary search tree
    pub fn validate(&self) -> bool {
        self.is_bst() && self.is_balanced()
    }
}

// Required functions
impl<T: Ord> TreeNode<T> {
    /// Creates a new `TreeNode<T>` with value `value` and children `left` and `right`
    pub fn node(value: T, left: TreeNode<T>, right: TreeNode<T>) -> TreeNode<T> {
        TreeNode::Node(value, Box::new(left), Box::new(right))
    }

    /// Creates a new `TreeNode<T>` with no children
    pub fn new() -> TreeNode<T> {
        TreeNode::Leaf
    }

    /// Inserts a new node with value `value` into the tree. If the value already exists in the tree,
    /// the function does nothing.
    ///
    /// After insertion, the tree is rebalanced if necessary
    pub fn insert(&mut self, value: T) {
        match self {
            TreeNode::Leaf => *self = Self::node(value, TreeNode::Leaf, TreeNode::Leaf),
            TreeNode::Node(v, lt, rt) => {
                if &value < v {
                    lt.insert(value);
                } else if &value > v {
                    rt.insert(value);
                }
            }
        }
    }

    /// Computes the balance factor of the tree (the difference between the height of the left and right subtrees)
    fn balance_factor(&self) -> i32 {
        todo!()
    }

    /// Performs a left rotation on the tree
    pub fn left_rotate(&mut self) {
        todo!()
    }
    /// Performs a right rotation on the tree
    pub fn right_rotate(&mut self) {
        todo!()
    }

    /// Rebalances the tree using either a single or double rotation, as specified in the AVL tree
    /// rebalancing algorithm.
    fn rebalance(&mut self) {
        todo!()
    }
}

// Implement `Default` for `TreeNode<T>`
impl<T: Ord> Default for TreeNode<T> {
    fn default() -> Self {
        TreeNode::Leaf
    }
}

// Implement `PartialEq` for `TreeNode<T>`
impl<T: Ord> PartialEq for TreeNode<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Leaf, Self::Leaf) => true,
            (Self::Node(v1, lt1, rt1), Self::Node(v2, lt2, rt2)) => {
                v1 == v2 && lt1 == lt2 && rt1 == rt2
            }
            _ => false,
        }
    }
}

// Implement `Eq` for `TreeNode<T>`
impl<T: Ord> Eq for TreeNode<T> {}

// Implement `From<Vec<T>>` for `TreeNode<T>`
impl<T: Ord> From<Vec<T>> for TreeNode<T> {
    fn from(value: Vec<T>) -> Self {
        let mut res = TreeNode::Leaf;

        for val in value {
            res.insert(val);
        }

        res

    }
}

// Implement `From<TreeNode<T>>` for `Vec<T>`
impl<T: Ord> From<TreeNode<T>> for Vec<T> {
    fn from(value: TreeNode<T>) -> Self {
        let mut res = Vec::new();
    
        fn inorder_traversal<T: Ord>(tree: TreeNode<T>, res: &mut Vec<T>) {
            match tree {
                TreeNode::Leaf => (),
                TreeNode::Node(v, lt, rt) => {
                    inorder_traversal(*lt, res);
                    res.push(v);
                    inorder_traversal(*rt, res);
                }
            }
        }

        inorder_traversal(value, &mut res);
        res
    }
}
