// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

// assumes there is at least one node
pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    diameter_rec(&root).1
}

pub fn diameter_rec(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    match root {
        Some(root) => {
            let root = root.borrow();
            let (lheight, ldiam) = diameter_rec(&root.left);
            let (rheight, rdiam) = diameter_rec(&root.right);
            let subtrees_max_diam = max(ldiam, rdiam);
            let max_diam_through_root = 2 + lheight + rheight;
            (
                1 + max(lheight, rheight),
                max(subtrees_max_diam, max_diam_through_root),
            )
        }
        None => (-1, -1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_node() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(diameter_of_binary_tree(root), 0);
    }

    #[test]
    fn test_diameter_three() {
        let mut root = TreeNode::new(1);
        let mut node2 = TreeNode::new(2);
        let mut node3 = TreeNode::new(3);
        let mut node4 = TreeNode::new(4);
        let node5 = TreeNode::new(5);

        node4.right = Some(Rc::new(RefCell::new(node5)));
        node3.right = Some(Rc::new(RefCell::new(node4)));
        node2.left = Some(Rc::new(RefCell::new(node3)));
        root.left = Some(Rc::new(RefCell::new(node2)));

        let root = Some(Rc::new(RefCell::new(root)));
        assert_eq!(diameter_of_binary_tree(root), 4);
    }

    #[test]
    fn test_three_level_tree() {
        let mut root = TreeNode::new(1);
        let mut node2 = TreeNode::new(2);
        let mut node3 = TreeNode::new(3);
        let node4 = TreeNode::new(4);
        let node5 = TreeNode::new(5);

        node2.left = Some(Rc::new(RefCell::new(node4)));
        node3.left = Some(Rc::new(RefCell::new(node5)));
        root.left = Some(Rc::new(RefCell::new(node2)));
        root.right = Some(Rc::new(RefCell::new(node3)));

        let root = Some(Rc::new(RefCell::new(root)));
        assert_eq!(diameter_of_binary_tree(root), 4);
    }

    #[test]
    fn test_single_child() {
        let mut root = TreeNode::new(1);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let root = Some(Rc::new(RefCell::new(root)));
        assert_eq!(diameter_of_binary_tree(root), 1);
    }
}
