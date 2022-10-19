pub struct Solution;

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
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(root) = &root {
            Self::dfs_find_sum(Rc::clone(root), target_sum)
        } else {
            return false;
        }
    }
    fn dfs_find_sum(node: Rc<RefCell<TreeNode>>, tgt: i32) -> bool {
        let node = node.borrow();
        match (&node.left, &node.right) {
            (None, None) => node.val == tgt,
            (Some(left), None) => Self::dfs_find_sum(Rc::clone(left), tgt - node.val),
            (None, Some(right)) => Self::dfs_find_sum(Rc::clone(right), tgt - node.val),
            (Some(left), Some(right)) => {
                if Self::dfs_find_sum(Rc::clone(left), tgt - node.val) {
                    true
                } else if Self::dfs_find_sum(Rc::clone(right), tgt - node.val) {
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        todo!()
    }
}
