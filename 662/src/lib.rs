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
    /// Given the root of a binary tree, return the maximum width of the given tree.
    ///
    /// The maximum width of a tree is the maximum width among all levels.
    ///
    /// The width of one level is defined as the length between the end-nodes
    /// (the leftmost and rightmost non-null nodes), where the null nodes between
    /// the end-nodes that would be present in a complete binary tree extending down to that level are also counted into the length calculation.
    ///
    ///It is guaranteed that the answer will in the range of a 32-bit signed integer.
    // Assign each node an integer, s.t. root got `0`, and that if a node got `n`
    // its left child is `2n+1`, whereas right is `2n+2`.
    // If storing the binary tree in an array, this number is exactly the index
    // in the array.
    // Traverse the tree, for each depth, record minimum/maximum.
    // Return maximum difference.
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut index_pairs = Vec::new();
        if let Some(root) = root {
            Self::dfs_traversal(root, 0, 0, &mut index_pairs);
            index_pairs
                .iter()
                .max_by(|pair1, pair2| (pair1.1 - pair1.0).cmp(&(pair2.1 - pair2.0)))
                .map(|(left, right)| (right - left) as i32 + 1)
                .unwrap()
        } else {
            0
        }
    }
    fn dfs_traversal(
        node: Rc<RefCell<TreeNode>>,
        node_index: u32,
        depth: u32,
        index_pairs: &mut Vec<(u32, u32)>,
    ) {
        match index_pairs.get_mut(depth as usize) {
            None => {
                index_pairs.push((node_index, node_index));
            }
            Some((left, right)) => {
                if node_index < *left {
                    *left = node_index
                } else if node_index > *right {
                    *right = node_index
                }
            }
        }
        if let Some(left) = &node.borrow().left {
            Self::dfs_traversal(Rc::clone(left), node_index * 2 + 1, depth + 1, index_pairs);
        }
        if let Some(right) = &node.borrow().right {
            Self::dfs_traversal(Rc::clone(right), node_index * 2 + 2, depth + 1, index_pairs);
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
