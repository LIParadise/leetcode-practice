pub struct Solution;
use lc_98_validate_binary_search_tree as leetcode_tree;
use leetcode_tree::*;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// Given the root of a binary tree, return true if you can partition
    /// the tree into two trees with equal sums of values after removing
    /// exactly one edge on the original tree.
    pub fn check_equal_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let total_sum = Self::replace_value_with_sum_subtree(Rc::clone(&root));
            if total_sum % 2 != 0 {
                false
            } else {
                // Cut one edge, result is two trees.
                //
                // One tree have new root being the old root,
                // the other should have new root being NOT the old root
                // s.t. it together with descendants form a proper non-empty subtree.
                //
                // Hence, when search, skip root.
                if let Some(left) = root.borrow().left.as_ref() {
                    if Self::search(Rc::clone(left), total_sum / 2).is_some() {
                        return true;
                    }
                }
                if let Some(right) = root.borrow().right.as_ref() {
                    Self::search(Rc::clone(right), total_sum / 2).is_some()
                } else {
                    false
                }
            }
        } else {
            false
        }
    }
    fn search(root: Rc<RefCell<TreeNode>>, tgt: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if tgt == root.borrow().val {
            Some(root)
        } else {
            if let Some(left) = root.borrow().left.as_ref() {
                if let Some(left_search_hit) = Self::search(Rc::clone(left), tgt) {
                    return Some(left_search_hit);
                }
            }
            if let Some(right) = root.borrow().right.as_ref() {
                if let Some(right_search_hit) = Self::search(Rc::clone(right), tgt) {
                    return Some(right_search_hit);
                }
            }
            None
        }
    }
    fn replace_value_with_sum_subtree(root: Rc<RefCell<TreeNode>>) -> i32 {
        let ret = root.borrow().val
            + root
                .borrow()
                .left
                .as_ref()
                .map_or(0, |l| Self::replace_value_with_sum_subtree(Rc::clone(l)))
            + root
                .borrow()
                .right
                .as_ref()
                .map_or(0, |r| Self::replace_value_with_sum_subtree(Rc::clone(r)));
        root.borrow_mut().val = ret;
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode_tree;
    use crate::Solution;
    use std::cell::RefCell;
    use std::rc::Rc;
    #[test]
    fn test_soln() {
        let data: &[Option<i32>] = &[
            Some(3),
            Some(2),
            None,
            None,
            Some(4),
            None,
            None,
            None,
            None,
            Some(1),
        ];

        let data = Some(Rc::new(RefCell::new(leetcode_tree::TreeNode::from(data))));
        println!("{:#?}", data);
        let flag = Solution::check_equal_tree(Some(Rc::clone(data.as_ref().unwrap())));
        println!("{:#?}", data);
        assert!(flag);
        let data: &[Option<i32>] = &[
            Some(-9),
            Some(-3),
            Some(2),
            None,
            Some(4),
            Some(4),
            Some(0),
            None,
            None,
            Some(-6),
            None,
            Some(-5),
        ];
        let data = Some(Rc::new(RefCell::new(leetcode_tree::TreeNode::from(data))));
        println!("{:#?}", data);
        let flag = Solution::check_equal_tree(Some(Rc::clone(data.as_ref().unwrap())));
        println!("{:#?}", data);
        assert!(!flag);
    }
}
