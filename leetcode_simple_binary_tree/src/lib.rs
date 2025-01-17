use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let expected = None;
        let calculated = TreeNode::try_from([].as_slice()).ok();
        assert_eq!(expected, calculated);

        let expected = Some(TreeNode {
            val: 42069,
            left: None,
            right: None,
        });
        let calculated = TreeNode::try_from([Some(42069)].as_slice()).ok();
        assert_eq!(expected, calculated);

        let expected = Some(TreeNode {
            val: 42069,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 114514,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1919810,
                    left: None,
                    right: None,
                }))),
            }))),
            right: None,
        });
        let calculated =
            TreeNode::try_from([Some(42069), Some(114514), None, None, Some(1919810)].as_slice())
                .ok();
        assert_eq!(expected, calculated);
        let calculated = TreeNode::try_from(
            [Some(42069), Some(114514), None, None, Some(1919810), None].as_slice(),
        )
        .ok();
        assert_eq!(expected, calculated);
    }
}
