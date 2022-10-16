pub struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl From<&[Option<i32>]> for TreeNode {
    fn from(arr: &[Option<i32>]) -> Self {
        if arr.len() == 0 {
            panic!()
        } else {
            fn build_node(idx: usize, arr: &[Option<i32>]) -> Option<TreeNode> {
                match arr.get(idx) {
                    None => None,
                    Some(&Some(val)) => Some(TreeNode {
                        val,
                        left: build_node(idx * 2 + 1, arr).map(|node| Rc::new(RefCell::new(node))),
                        right: build_node(idx * 2 + 2, arr).map(|node| Rc::new(RefCell::new(node))),
                    }),
                    Some(None) => None,
                }
            }
            build_node(0, arr).unwrap()
        }
    }
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
use std::thread::panicking;

struct Stack<T> {
    arr: Vec<T>,
}
impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { arr: Vec::new() }
    }
    pub fn push(&mut self, i: T) {
        self.arr.push(i)
    }
    pub fn pop(&mut self) -> Option<T> {
        self.arr.pop()
    }
}

/// Strict in the sense using `.lt` and `.gt` rather than `.le` or `.ge`
struct StrictRangeChecker {
    lower_bound: Option<i32>,
    upper_bound: Option<i32>,
}
use StrictRangeChecker as SRC;
impl StrictRangeChecker {
    pub fn new((lower_bound, upper_bound): (Option<i32>, Option<i32>)) -> Self {
        Self {
            lower_bound,
            upper_bound,
        }
    }
    pub fn is_valid(&self, u: i32) -> bool {
        match (self.lower_bound, self.upper_bound) {
            (None, None) => true,
            (None, Some(upper_bound)) => u < upper_bound,
            (Some(lower_bound), None) => u > lower_bound,
            (Some(lower_bound), Some(upper_bound)) => u < upper_bound && u > lower_bound,
        }
    }
    pub fn destruct(self) -> (Option<i32>, Option<i32>) {
        (self.lower_bound, self.upper_bound)
    }
}

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(root) => {
                let mut stack = Stack::new();
                let src = SRC::new((None, None));
                stack.push((root, src));
                let mut ret = true;
                while let Some((top, src)) = stack.pop() {
                    if !src.is_valid(top.borrow().val) {
                        ret = false;
                        break;
                    }
                    let (low, up) = src.destruct();
                    if let Some(left) = &top.borrow().left {
                        stack.push((Rc::clone(left), SRC::new((low, Some(top.borrow().val)))))
                    }
                    if let Some(right) = &top.borrow().right {
                        stack.push((Rc::clone(right), SRC::new((Some(top.borrow().val), up))))
                    }
                }
                ret
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Solution, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;
    #[test]
    fn test_soln() {
        let input = Some(Rc::new(RefCell::new(
            <TreeNode as From<&[Option<i32>]>>::from(
                [2, 1, 3]
                    .into_iter()
                    .map(|i| Some(i))
                    .collect::<Vec<_>>()
                    .as_slice(),
            ),
        )));
        assert!(Solution::is_valid_bst(input));
        let input = Some(Rc::new(RefCell::new(
            <TreeNode as From<&[Option<i32>]>>::from(
                &[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)][..],
            ),
        )));
        assert!(!Solution::is_valid_bst(input));
    }
}
