pub struct Solution;
impl From<&[Option<i32>]> for TreeNode {
    fn from(arr: &[Option<i32>]) -> Self {
        if arr.len() == 0 {
            panic!()
        } else {
            fn build_node(idx: usize, arr: &[Option<i32>]) -> Option<TreeNode> {
                match arr.get(idx) {
                    None | Some(None) => None,
                    Some(&Some(val)) => Some(TreeNode {
                        val,
                        left: build_node(idx * 2 + 1, arr).map(|node| Rc::new(RefCell::new(node))),
                        right: build_node(idx * 2 + 2, arr).map(|node| Rc::new(RefCell::new(node))),
                    }),
                }
            }
            build_node(0, arr).unwrap()
        }
    }
}

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

// Janky way to set marker for DFS traversal
// Leetcode specifies input [-10^9, 10^9]
static ABS_RANGE: i32 = 1_000_000_000;
static SHIFT: i32 = ABS_RANGE + 1;
fn shift(i: &mut i32) {
    if i.is_positive() {
        *i += SHIFT;
    } else {
        *i -= SHIFT;
    }
}
fn unshift(i: &mut i32) {
    if i.is_positive() {
        *i -= SHIFT;
    } else {
        *i += SHIFT;
    }
}
fn not_shifted(i: &i32) -> bool {
    (-ABS_RANGE..=ABS_RANGE).contains(i)
}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        static ERR_NOT_FOUND: &'static str = "Leetcode asserts input in tree but not found";
        if let (Some(root), Some(p), Some(q)) = (root, p, q) {
            let p_trace = Self::search_with_trace(Rc::clone(&root), p).expect(ERR_NOT_FOUND);
            dbg!("here");
            let q_trace = Self::search_with_trace(Rc::clone(&root), q).expect(ERR_NOT_FOUND);
            p_trace
                .into_iter()
                .zip(q_trace.into_iter())
                .take_while(|(pt, qt)| pt == qt)
                .last()
                .map(|(ret, _)| ret)
        } else {
            None
        }
    }
    fn search_with_trace(
        root: Rc<RefCell<TreeNode>>,
        tgt: Rc<RefCell<TreeNode>>,
    ) -> Option<Vec<Rc<RefCell<TreeNode>>>> {
        // DFS mark is done via shifting input range
        //
        // This works based on the assumption that input, if exists, must be some
        // `rc::Rc` from within the tree.
        // In other words, it FAILS miserably if search target ain't from within the tree.
        //
        // Yes, a totally janky solution, but at the end of the day, a tree that doesn't store
        // extra field to support traversal is itself a janky idea.
        let mut trace_stack = Vec::new();
        shift(&mut root.borrow_mut().val);
        trace_stack.push(Rc::clone(&root));
        while let Some(cur_node) = trace_stack.last() {
            let cur_node = Rc::clone(cur_node);
            dbg!(cur_node.borrow().val);
            if cur_node == tgt {
                Self::unmark_tree(Rc::clone(&root));
                return Some(trace_stack);
            } else {
                if let Some(left) = cur_node.borrow().left.as_ref() {
                    if not_shifted(&left.borrow().val) {
                        shift(&mut left.borrow_mut().val);
                        trace_stack.push(Rc::clone(left));
                        continue;
                    }
                }
                if let Some(right) = cur_node.borrow().right.as_ref() {
                    if not_shifted(&right.borrow().val) {
                        shift(&mut right.borrow_mut().val);
                        trace_stack.push(Rc::clone(right));
                        continue;
                    }
                }
                trace_stack.pop();
            }
        }
        Self::unmark_tree(Rc::clone(&root));
        None
    }
    fn unmark_tree(root: Rc<RefCell<TreeNode>>) {
        let mut dfs_stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        dfs_stack.push(Rc::clone(&root));
        while let Some(cur_node) = dfs_stack.last() {
            let cur_node = Rc::clone(&cur_node);
            if let Some(left) = cur_node.borrow_mut().left.as_ref() {
                if !not_shifted(&left.borrow_mut().val) {
                    dfs_stack.push(Rc::clone(left));
                    continue;
                }
            }
            if let Some(right) = cur_node.borrow_mut().right.as_ref() {
                if !not_shifted(&right.borrow_mut().val) {
                    dfs_stack.push(Rc::clone(right));
                    continue;
                }
            }
            // TODO
            // ???
            // no `;` or no more statements here causes borrow checker to complain
            unshift(&mut cur_node.borrow_mut().val);
            dfs_stack.pop();
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
        let tree = Rc::new(RefCell::new(Into::<TreeNode>::into(
            vec![
                Some(3),
                Some(5),
                Some(1),
                Some(6),
                Some(2),
                Some(0),
                Some(8),
                None,
                None,
                Some(7),
                Some(4),
            ]
            .as_ref(),
        )));
        let root = Some(Rc::clone(&tree));
        let p = Some(Rc::clone(tree.borrow().left.as_ref().unwrap()));
        let q = Some(Rc::clone(tree.borrow().right.as_ref().unwrap()));
        assert_eq!(
            Solution::lowest_common_ancestor(root, p, q),
            Some(Rc::clone(&tree))
        );

        let tree = Rc::new(RefCell::new(Into::<TreeNode>::into(
            vec![
                Some(3),
                Some(5),
                Some(1),
                Some(6),
                Some(2),
                Some(0),
                Some(8),
                None,
                None,
                Some(7),
                Some(4),
            ]
            .as_ref(),
        )));
        let root = Some(Rc::clone(&tree));
        let p = Some(Rc::clone(tree.borrow().left.as_ref().unwrap()));
        let q = Some(Rc::clone(
            tree.borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap(),
        ));
        assert_eq!(
            Solution::lowest_common_ancestor(root, p, q),
            Some(Rc::clone(tree.borrow().left.as_ref().unwrap()))
        );

        let tree = Rc::new(RefCell::new(Into::<TreeNode>::into(
            vec![Some(1), Some(2)].as_ref(),
        )));
        let root = Some(Rc::clone(&tree));
        let p = Some(Rc::clone(&tree));
        let q = Some(Rc::clone(tree.borrow().left.as_ref().unwrap()));
        assert_eq!(
            Solution::lowest_common_ancestor(root, p, q),
            Some(Rc::clone(&tree))
        );

        let tree = Rc::new(RefCell::new(Into::<TreeNode>::into(
            vec![
                Some(9),
                Some(-1),
                Some(-4),
                Some(10),
                Some(3),
                None,
                None,
                None,
                Some(5),
            ]
            .as_ref(),
        )));
        let root = Some(Rc::clone(&tree));
        let p = Some(Rc::clone(
            tree.borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap(),
        ));
        let q = Some(Rc::clone(
            tree.borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap(),
        ));
        assert_eq!(
            Solution::lowest_common_ancestor(root, p, q),
            Some(Rc::clone(tree.borrow().left.as_ref().unwrap()))
        );
    }
}
