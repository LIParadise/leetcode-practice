pub struct Solution;
use std::{cell::RefCell, rc::Rc};

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

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut paths = vec![];
        if let Some(root) = root.as_ref() {
            let mut path = vec![Rc::clone(root)];
            Self::path_sum_worker_dfs(
                &root.try_borrow().unwrap(),
                target_sum,
                &mut path,
                &mut paths,
            );
        }
        paths
    }
    fn path_sum_worker_dfs(
        node: &TreeNode,
        tgt: i32,
        path: &mut Vec<Rc<RefCell<TreeNode>>>,
        paths: &mut Vec<Vec<i32>>,
    ) {
        if let Some(l) = node.left.as_ref() {
            path.push(Rc::clone(l));
            Self::path_sum_worker_dfs(&l.try_borrow().unwrap(), tgt - node.val, path, paths);
        }
        if let Some(r) = node.right.as_ref() {
            path.push(Rc::clone(r));
            Self::path_sum_worker_dfs(&r.try_borrow().unwrap(), tgt - node.val, path, paths);
        }
        if node.left.is_none() && node.right.is_none() && node.val == tgt {
            paths.push(path.iter().map(|n| n.try_borrow().unwrap().val).collect());
        }
        path.pop();
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
