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
use std::rc::Rc;

pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut current_level = vec![root.unwrap()];
    let mut level = 1;
    let mut max_sum: i32 = current_level.iter().map(|node| node.borrow().val).sum();
    let mut max_level = level;
    while !current_level.is_empty() {
        level += 1;
        let next_level: Vec<Rc<RefCell<TreeNode>>> = current_level
            .iter()
            .flat_map(|n| {
                [n.borrow().left.clone(), n.borrow().right.clone()]
                    .into_iter()
                    .flatten()
            })
            .collect();
        let next_sum: i32 = next_level.iter().map(|node| node.borrow().val).sum();
        if !next_level.is_empty() && next_sum > max_sum {
            max_sum = next_sum;
            max_level = level;
        }
        current_level = next_level;
    }
    max_level
}
