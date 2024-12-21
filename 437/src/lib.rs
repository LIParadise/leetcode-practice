pub struct Solution;

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

impl TryFrom<&[Option<i32>]> for TreeNode {
    type Error = ();
    fn try_from(values: &[Option<i32>]) -> Result<Self, Self::Error> {
        fn make_node_at(values: &[Option<i32>], idx: usize) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(i) = values.get(idx).and_then(|x| x.as_ref()) {
                Some(Rc::new(RefCell::new(TreeNode {
                    val: *i,
                    left: make_node_at(values, idx * 2 + 1),
                    right: make_node_at(values, idx * 2 + 2),
                })))
            } else {
                None
            }
        }
        if let Some(root) = make_node_at(values, 0).and_then(Rc::into_inner) {
            Ok(root.into_inner())
        } else {
            Err(())
        }
    }
}

use std::collections::BTreeMap;
pub use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut partial_sum_ocurrence_counts = BTreeMap::from_iter([(0, 1)]);
        root.as_ref()
            .and_then(|root| root.try_borrow().ok())
            .map(|root| {
                Self::path_sum_dfs_worker(target_sum, &root, 0, &mut partial_sum_ocurrence_counts)
            })
            .and_then(|u| u.try_into().ok())
            .unwrap_or(0)
    }
    fn path_sum_dfs_worker(
        target: i32,
        node: &TreeNode,
        ancestors_sum: i64,
        partial_sum_ocurrence_counts: &mut BTreeMap<i64, usize>,
    ) -> usize {
        let mut ret = 0;
        let sum_of_node_and_ancestors = node.val as i64 + ancestors_sum;
        ret += partial_sum_ocurrence_counts
            .get(&(sum_of_node_and_ancestors - target as i64))
            .unwrap_or(&0);
        partial_sum_ocurrence_counts
            .entry(sum_of_node_and_ancestors)
            .and_modify(|cnt| *cnt += 1)
            .or_insert(1);

        if let Some(left) = node.left.as_ref().and_then(|left| left.try_borrow().ok()) {
            ret += Self::path_sum_dfs_worker(
                target,
                &left,
                sum_of_node_and_ancestors,
                partial_sum_ocurrence_counts,
            );
        }
        if let Some(right) = node
            .right
            .as_ref()
            .and_then(|right| right.try_borrow().ok())
        {
            ret += Self::path_sum_dfs_worker(
                target,
                &right,
                sum_of_node_and_ancestors,
                partial_sum_ocurrence_counts,
            );
        }
        if let Some(cnt) = partial_sum_ocurrence_counts.get_mut(&sum_of_node_and_ancestors) {
            *cnt -= 1;
        }
        ret
    }
}

#[cfg(test)]
mod tests {

    use crate::*;
    #[test]
    fn test_soln() {
        let tree = crate::TreeNode::try_from(
            [
                Some(10),
                Some(5),
                Some(-3),
                Some(3),
                Some(2),
                None,
                Some(11),
                Some(3),
                Some(-2),
                None,
                Some(1),
            ]
            .as_slice(),
        )
        .map(RefCell::new)
        .map(Rc::new)
        .ok();
        assert_eq!(Solution::path_sum(tree, 8), 3);

        let tree = crate::TreeNode::try_from(
            [
                Some(5),
                Some(4),
                Some(8),
                Some(11),
                None,
                Some(13),
                Some(4),
                Some(7),
                Some(2),
                None,
                None,
                Some(5),
                Some(1),
            ]
            .as_slice(),
        )
        .map(RefCell::new)
        .map(Rc::new)
        .ok();
        assert_eq!(Solution::path_sum(tree, 22), 3);

        let tree = crate::TreeNode::try_from([Some(1)].as_slice())
            .map(RefCell::new)
            .map(Rc::new)
            .ok();
        assert_eq!(Solution::path_sum(tree, 0), 0);
    }
}
