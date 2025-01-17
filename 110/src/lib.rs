pub struct Solution;
use std::cell::RefCell;
use std::rc::Rc;

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
    fn try_from(arr: &[Option<i32>]) -> Result<Self, Self::Error> {
        (!arr.is_empty())
            .then(|| {
                fn build_node(idx: usize, arr: &[Option<i32>]) -> Option<TreeNode> {
                    arr.get(idx)
                        .into_iter()
                        .flatten()
                        .map(|&val| TreeNode {
                            val,
                            left: build_node(idx * 2 + 1, arr).map(RefCell::new).map(Rc::new),
                            right: build_node(idx * 2 + 2, arr).map(RefCell::new).map(Rc::new),
                        })
                        .next()
                }
                build_node(0, arr).unwrap()
            })
            .ok_or(())
    }
}

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        root.map(Self::get_height_if_balanced)
            .as_ref()
            .map(Result::is_ok)
            .unwrap_or(true)
    }

    fn get_height_if_balanced(node: Rc<RefCell<TreeNode>>) -> Result<usize, ()> {
        let left_height = node
            .borrow()
            .left
            .as_ref()
            .cloned() /* `Rc::clone` */
            .map(Self::get_height_if_balanced)
            /*
             * bail out iff we have imbalanced subtree:
             * here we have `Option<Result<usize, ()>>`
             * and we need to early return upon inner `Err` variant,
             * so transpose it and apply `?`
             */
            .transpose()?;
        let right_height = node
            .borrow()
            .right
            .as_ref()
            .cloned() /* `Rc::clone` */
            .map(Self::get_height_if_balanced)
            /*
             * bail out iff we have imbalanced subtree:
             * here we have `Option<Result<usize, ()>>`
             * and we need to early return upon inner `Err` variant,
             * so transpose it and apply `?`
             */
            .transpose()?;

        // the `Err` variant is not invalid (inbalanced): just NIL node so height undefined
        dbg!(node.borrow().val);
        dbg!(match (left_height, right_height) {
            (None, None) => {
                // both left and right subtree NIL, so height is zero
                Ok(0)
            }
            (Some(0), None) | (None, Some(0)) => {
                // similarly, only one side is defined and other side is of height zero,
                // then we are of height one.
                Ok(1)
            }
            (Some(x), Some(y)) if x.abs_diff(y) <= 1 => Ok(std::cmp::max(x, y) + 1),
            _ => {
                // inbalanced, return `Err` variant for callers to bail out
                Err(())
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Solution, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;
    #[test]
    fn test_soln() {
        let tree = TreeNode::try_from(
            [Some(3), Some(9), Some(20), None, None, Some(15), Some(7)].as_slice(),
        )
        .ok()
        .map(RefCell::new)
        .map(Rc::new);
        assert!(Solution::is_balanced(tree));
    }
}
