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

pub use std::cell::RefCell;
pub use std::rc::Rc;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut targets = vec![target_sum as i64];
        root.as_ref()
            .and_then(|root| root.try_borrow().ok())
            .map(|root| Self::path_sum_dfs_worker(&root, &mut targets))
            .and_then(|u| u.try_into().ok())
            .unwrap_or(0)
    }
    fn path_sum_dfs_worker(node: &TreeNode, targets: &mut Vec<i64>) -> usize {
        mod view {
            #[derive(Debug)]
            pub struct View<'a>(&'a mut Vec<i64>);
            impl<'a> View<'a> {
                pub fn new(arr: &'a mut Vec<i64>, i: i32) -> (usize, Self) {
                    arr[1..].iter_mut().for_each(|x| *x += i as i64);
                    arr.push(i as i64);
                    let ret = match &arr[..] {
                        [orig_target, sums_of_nodes @ ..] => sums_of_nodes
                            .iter()
                            .filter(|sum| sum == &orig_target)
                            .count(),
                        [] => 0,
                    };
                    (ret, Self(arr))
                }
            }
            impl<'a> Drop for View<'a> {
                fn drop(&mut self) {
                    if let Some(i) = self.0.pop() {
                        self.0[1..].iter_mut().for_each(|sum| *sum -= i);
                    }
                }
            }
            impl<'a> std::ops::Deref for View<'a> {
                type Target = Vec<i64>;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
            impl<'a> std::ops::DerefMut for View<'a> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
        }
        let (mut ret, mut view) = dbg!(view::View::new(targets, node.val));
        if let Some(l) = node.left.as_ref().and_then(|l| l.try_borrow().ok()) {
            ret += Self::path_sum_dfs_worker(&l, &mut view);
        }
        if let Some(r) = node.right.as_ref().and_then(|r| r.try_borrow().ok()) {
            ret += Self::path_sum_dfs_worker(&r, &mut view);
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
    }
}
