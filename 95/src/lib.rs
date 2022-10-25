use std::cell::RefCell;
use std::rc::Rc;

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
    fn depth(&self) -> usize {
        1 + std::cmp::max(
            self.right
                .as_ref()
                .map_or(0, |right| right.as_ref().borrow().depth()),
            self.left
                .as_ref()
                .map_or(0, |left| left.as_ref().borrow().depth()),
        )
    }
    fn dfs_build(idx: usize, arr: &[Option<i32>]) -> Option<Self> {
        let val = (*arr.get(idx)?)?;
        Some(Self {
            val,
            left: Self::wrap_node(Self::dfs_build(idx * 2 + 1, arr)),
            right: Self::wrap_node(Self::dfs_build(idx * 2 + 2, arr)),
        })
    }
    #[inline]
    fn wrap_node(t: Option<TreeNode>) -> Option<Rc<RefCell<TreeNode>>> {
        t.map(|t| Rc::new(RefCell::new(t)))
    }
}

impl From<&[Option<i32>]> for TreeNode {
    fn from(arr: &[Option<i32>]) -> Self {
        TreeNode::dfs_build(0, arr).expect("ARR shall not be empty and root shall not be NIL")
    }
}

struct DummyVec {
    vec: Vec<Option<i32>>,
}

/// Convert a binary tree into linear array,
/// s.t. for index i, its left is at (2*i+1) while right at (2*i+2).
///
/// Empty NIL are represented by None; trailing NIL would not be included.
impl From<Option<Rc<RefCell<TreeNode>>>> for DummyVec {
    fn from(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        // We don't know how imbalanced the tree is; allocate full tree.
        let full_binary_tree_size =
            2_usize.pow(root.as_ref().unwrap().as_ref().borrow().depth() as u32) - 1;
        let mut ret = Vec::with_capacity(full_binary_tree_size);
        for _ in 0..full_binary_tree_size {
            ret.push(None)
        }

        // Fit the tree into the array, using aforementioned indexing,
        // i.e. for index i, left at (2*i+1), right at (2*i+2)
        // Indexing start from 0.
        //
        // Using BFS here for no particular reason.
        // Assume tree, hence acyclic, hence no marker required.
        let mut queue: std::collections::LinkedList<(Option<Rc<RefCell<TreeNode>>>, usize)> =
            std::collections::LinkedList::new();
        queue.push_front((root, 0));
        while !queue.is_empty() {
            let ptr = queue.pop_back().unwrap();
            match ptr {
                (Some(top), idx) => {
                    ret[idx] = Some(top.as_ref().borrow().val);
                    queue.push_front((
                        top.borrow().left.as_ref().map(|left| Rc::clone(left)),
                        idx * 2 + 1,
                    ));
                    queue.push_front((
                        top.borrow().right.as_ref().map(|right| Rc::clone(right)),
                        idx * 2 + 2,
                    ));
                }
                _ => {}
            }
        }
        let trailing_nil_cnt = ret.iter().rev().take_while(|op| op.is_none()).count();
        ret.resize(ret.len() - trailing_nil_cnt, None);

        Self { vec: ret }
    }
}

impl Solution {
    /// Given an integer n, return all the structurally unique BST's,
    /// which has exactly n nodes of unique values from 1 to n.
    /// Return the answer in any order.
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        // Consider BST with size n consisting of [1..=n]
        // 0 => {}
        // 1 => {1}
        // 2 => 1 as root, left {} -> 1, right {1} -> 1, # = 1*1 = 1
        //      2 as root, left {1} -> 1, right {} -> 1, # = 1*1 = 1
        //      1 + 1 = 2
        // 3 => 2*1    + 1*1    + 1*2    = 5
        //      root=3   root=2   root=1
        let mut trees = vec![
            vec![vec![None]],    /* null */
            vec![vec![Some(1)]], /* {1} */
            vec![
                /* {1, null, 2}, {2, 1} */
                vec![Some(1), None, Some(2)],
                vec![Some(2), Some(1)],
            ],
        ];
        let n = n as usize;
        if n < trees.len() {
            return trees[n]
                .iter()
                .map(|vec| TreeNode::wrap_node(Some(vec.as_slice().into())))
                .collect();
        }
        while n >= trees.len() {
            let mut bst_variants = Vec::new();
            (1..=trees.len())
                .zip(trees.iter().zip(trees.iter().rev()))
                .for_each(|(root, (l, r))| {
                    l.iter().for_each(|l| {
                        r.iter().for_each(|r| {
                            let mut root_node = TreeNode::new(root as i32);
                            root_node.left = if l[0] == None {
                                // NIL
                                None
                            } else {
                                TreeNode::wrap_node(Some(l.as_slice().into()))
                            };
                            root_node.right = if r[0] == None {
                                // NIL
                                None
                            } else {
                                TreeNode::wrap_node(Some(
                                    // With tree [1..=n], to build a tree with
                                    // [1..=(n+1)], their contents needs offset
                                    // Turns out it's exactly equal to the
                                    // value of the new root.
                                    r.iter()
                                        .map(|n| n.map(|n| n + root as i32))
                                        .collect::<Vec<_>>()
                                        .as_slice()
                                        .into(),
                                ))
                            };
                            bst_variants.push(TreeNode::wrap_node(Some(root_node)));
                        })
                    })
                });
            if n == trees.len() {
                return bst_variants;
            } else {
                // Smaller trees are stored as (binary tree as linear arr)
                // no particular reason except I'm too lazy to do (tree
                // traversal & offset) when building new tree.
                //
                // (For tree [1..=n] to be subtree [1..=(n+1)], some values
                // needs to be adjusted; check above iterator for_each)
                trees.push(
                    bst_variants
                        .into_iter()
                        .map(|bst| <DummyVec as From<Option<Rc<RefCell<TreeNode>>>>>::from(bst).vec)
                        .collect::<Vec<_>>(),
                )
            }
        }
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::{Solution, TreeNode};
    #[test]
    fn test_soln() {
        let my_trees = Solution::generate_trees(3)
            .into_iter()
            .map(|root| <crate::DummyVec as From<Option<Rc<RefCell<TreeNode>>>>>::from(root).vec)
            .collect::<std::collections::BTreeSet<_>>();
        let ans_trees = vec![
            vec![Some(1), None, Some(3), None, None, Some(2)],
            vec![Some(1), None, Some(2), None, None, None, Some(3)],
            vec![Some(2), Some(1), Some(3)],
            vec![Some(3), Some(2), None, Some(1)],
            vec![Some(3), Some(1), None, None, Some(2)],
        ]
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(my_trees, ans_trees);
    }
    #[test]
    fn test_gen_tree() {
        let orig_vec = vec![Some(1), None, Some(2), None, None, None, Some(3)];
        let tree = TreeNode::wrap_node(Some(orig_vec.clone().as_slice().into()));
        let mut ptr: &Rc<RefCell<TreeNode>> = tree.as_ref().unwrap();
        assert_eq!(1, ptr.borrow().val);
        assert_eq!(None, ptr.borrow().left);
        let right = Rc::clone(&ptr.borrow().right.as_ref().unwrap());
        ptr = &right;
        assert_eq!(2, ptr.borrow().val);
        assert_eq!(None, ptr.borrow().left);
        let right = Rc::clone(&ptr.borrow().right.as_ref().unwrap());
        ptr = &right;
        assert_eq!(3, ptr.borrow().val);
        let tree = <crate::DummyVec as From<Option<Rc<RefCell<TreeNode>>>>>::from(tree).vec;
        assert_eq!(orig_vec, tree);
    }
}
