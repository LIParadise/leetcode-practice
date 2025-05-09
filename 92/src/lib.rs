pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct ListNodeIter {
    node: Option<Box<ListNode>>,
}
impl Iterator for ListNodeIter {
    type Item = Box<ListNode>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.node.as_mut().and_then(|node| node.next.take()) {
            self.node.replace(next)
        } else {
            self.node.take()
        }
    }
}
impl FromIterator<Box<ListNode>> for Option<Box<ListNode>> {
    fn from_iter<T: IntoIterator<Item = Box<ListNode>>>(iter: T) -> Self {
        let mut ret = None;
        let mut tail = &mut ret;
        for node in iter {
            *tail = Some(node);
            tail = &mut tail.as_mut().unwrap().next;
        }
        ret
    }
}

impl Solution {
    /// Given 1-indexed linked list indices, reverse the sub list.
    /// Both ends included, e.g. if two and four then that's three nodes.
    // Note the cursed 1-indexed convention...
    //
    // Implicitly we'd like a constant space solution,
    // for if linear space is allowed, just use arrays.
    // So recursive solutions are probably out of the way.
    //
    // Idea:
    // It's relatively straightforward reversing whole linked list:
    // Pop each node, and for each one prepend it to the new list.
    //
    // Thus all we have to do is severing the sub lists that we want to keep,
    // apply the aforementioned idea to the middle sub list,
    // stitch them together, then return.
    pub fn reverse_between(
        mut node: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        if !(right >= left && left >= 1) {
            panic!("index out of bound");
        }
        if node.is_none() {
            return None;
        }

        if left == 1 {
            // 1-indexed is so cursed...
            let mut before_tail = &mut node;
            for _ in 1..right {
                before_tail = &mut before_tail.as_mut().unwrap().next;
            }
            let old_tail = before_tail.as_mut().and_then(|n| n.next.take());
            let mut reversed = Self::reverse_list(node);
            before_tail = &mut reversed;
            while before_tail.is_some() {
                before_tail = &mut before_tail.as_mut().unwrap().next;
            }
            *before_tail = old_tail;
            reversed
        } else {
            let left_of_left_node = {
                let mut ret = &mut node;
                for _ in 2..left {
                    ret = &mut ret.as_mut().unwrap().next;
                }
                ret
            };
            let mut left_node = left_of_left_node.as_mut().and_then(|n| n.next.take());
            let mut right_node = &mut left_node;
            for _ in left..right {
                right_node = &mut right_node.as_mut().unwrap().next;
            }
            let right_of_right_node = right_node.as_mut().and_then(|n| n.next.take());
            let mut reversed = Self::reverse_list(left_node);
            {
                let mut last = &mut reversed;
                while last.is_some() {
                    last = &mut last.as_mut().unwrap().next;
                }
                *last = right_of_right_node;
            }
            if let Some(left_of_left_node) = left_of_left_node {
                left_of_left_node.next = reversed;
            }
            node
        }
    }
    fn reverse_list(node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        ListNodeIter { node }.fold(None, |ret, mut node| {
            node.next = ret;
            Some(node)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{ListNode, ListNodeIter, Solution};
    #[test]
    fn test_soln() {
        let expected = Option::<Box<ListNode>>::from_iter(vec![]);
        let ans = Solution::reverse_between(Option::<_>::from_iter(vec![]), 1, 1);
        assert_eq!(
            Vec::from_iter(ListNodeIter { node: ans }),
            Vec::from_iter(ListNodeIter { node: expected })
        );

        let expected =
            Option::<Box<ListNode>>::from_iter([1].into_iter().map(ListNode::new).map(Box::new));
        let ans = Solution::reverse_between(
            Option::<_>::from_iter([1].into_iter().map(ListNode::new).map(Box::new)),
            1,
            1,
        );
        assert_eq!(
            Vec::from_iter(ListNodeIter { node: ans }),
            Vec::from_iter(ListNodeIter { node: expected })
        );

        let expected =
            Option::<Box<ListNode>>::from_iter([1, 2].into_iter().map(ListNode::new).map(Box::new));
        let ans = Solution::reverse_between(
            Option::<_>::from_iter([1, 2].into_iter().map(ListNode::new).map(Box::new)),
            1,
            1,
        );
        assert_eq!(
            Vec::from_iter(ListNodeIter { node: ans }),
            Vec::from_iter(ListNodeIter { node: expected })
        );

        let expected =
            Option::<Box<ListNode>>::from_iter([1, 2].into_iter().map(ListNode::new).map(Box::new));
        let ans = Solution::reverse_between(
            Option::<_>::from_iter([1, 2].into_iter().map(ListNode::new).map(Box::new)),
            2,
            2,
        );
        assert_eq!(
            Vec::from_iter(ListNodeIter { node: ans }),
            Vec::from_iter(ListNodeIter { node: expected })
        );

        let expected =
            Option::<Box<ListNode>>::from_iter([1, 2].into_iter().map(ListNode::new).map(Box::new));
        let ans = Solution::reverse_between(
            Option::<_>::from_iter([2, 1].into_iter().map(ListNode::new).map(Box::new)),
            1,
            2,
        );
        assert_eq!(
            Vec::from_iter(ListNodeIter { node: ans }),
            Vec::from_iter(ListNodeIter { node: expected })
        );

        let expected = Option::<Box<ListNode>>::from_iter(
            [1, 2, 3].into_iter().map(ListNode::new).map(Box::new),
        );
        let ans = Solution::reverse_between(
            Option::<_>::from_iter([3, 2, 1].into_iter().map(ListNode::new).map(Box::new)),
            1,
            3,
        );
        assert_eq!(
            Vec::from_iter(ListNodeIter { node: ans }),
            Vec::from_iter(ListNodeIter { node: expected })
        );

        let expected = Option::<Box<ListNode>>::from_iter(
            [1, 3, 2].into_iter().map(ListNode::new).map(Box::new),
        );
        let ans = Solution::reverse_between(
            Option::<_>::from_iter([1, 2, 3].into_iter().map(ListNode::new).map(Box::new)),
            2,
            3,
        );
        assert_eq!(
            Vec::from_iter(ListNodeIter { node: ans }),
            Vec::from_iter(ListNodeIter { node: expected })
        );
    }
}
