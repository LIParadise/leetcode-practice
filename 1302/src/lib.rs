pub struct Solution;
use lc_98_validate_binary_search_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut pair_deepest_depth_sum = (0_usize, 0_i32);
        if let Some(root) = root {
            Self::dfs_deepest_leaves_sum_worker(root, 0, &mut pair_deepest_depth_sum);
            pair_deepest_depth_sum.1
        } else {
            0
        }
    }
    fn dfs_deepest_leaves_sum_worker(
        root: Rc<RefCell<TreeNode>>,
        depth: usize,
        pair_deepest_depth_sum: &mut (usize, i32),
    ) {
        if depth == 3 {
        }
        if depth == pair_deepest_depth_sum.0 {
            pair_deepest_depth_sum.1 += root.borrow().val;
        } else if depth > pair_deepest_depth_sum.0 {
            pair_deepest_depth_sum.0 = depth;
            pair_deepest_depth_sum.1 = root.borrow().val;
        }
        if let Some(left) = &root.borrow().left {
            Self::dfs_deepest_leaves_sum_worker(Rc::clone(left), depth + 1, pair_deepest_depth_sum);
        }
        if let Some(right) = &root.borrow().right {
            Self::dfs_deepest_leaves_sum_worker(
                Rc::clone(right),
                depth + 1,
                pair_deepest_depth_sum,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use lc_98_validate_binary_search_tree::TreeNode as T;
    use std::cell::RefCell;
    use std::rc::Rc;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::deepest_leaves_sum(Some(Rc::new(RefCell::new(T::from(
                [
                    Some(1),
                    Some(2),
                    Some(3),
                    Some(4),
                    Some(5),
                    None,
                    Some(6),
                    Some(7),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(8)
                ]
                .as_slice()
            ))))),
            15
        );
    }
}
