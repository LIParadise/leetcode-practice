/*
 * Start of implied definition for singly-linked list.
 * Should omit this part when LC submit
 */
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
/*
 * End of implied definition for singly-linked list.
 * Should omit this part when LC submit
 */

// Utilities for create list from containers or vice versa
mod list_helpers {
    use super::ListNode;
    impl ListNode {
        pub fn from<T: IntoIterator<Item = i32>>(iter: T) -> Option<Box<Self>> {
            let iter = iter.into_iter();
            Self::from_iter(iter)
        }
        fn from_iter<T: Iterator<Item = i32>>(mut iter: T) -> Option<Box<Self>> {
            match iter.next() {
                None => None,
                Some(i) => Some(Box::new(Self {
                    val: i,
                    next: Self::from_iter(iter),
                })),
            }
        }
    }

    pub struct ListNodeIntoIterator {
        node: Option<Box<ListNode>>,
    }

    impl Iterator for ListNodeIntoIterator {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            match &mut self.node {
                None => None,
                Some(node) => {
                    let ret = node.val;
                    self.node = node.next.take();
                    Some(ret)
                }
            }
        }
    }

    impl IntoIterator for ListNode {
        type Item = i32;
        type IntoIter = ListNodeIntoIterator;
        fn into_iter(self) -> Self::IntoIter {
            let node = Some(Box::new(self));
            Self::IntoIter { node }
        }
    }
}

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let len = Self::len(&head);
        let mut dummy = ListNode { val: 0, next: head };
        let mut parent_ref = &mut dummy;
        for _ in 0..len / 2 {
            /*
             * Ownership makes list manipulation behave like playing solitaire
             * For each deck we'd have to prepare a *placeholder*
             * Here placeholder are just variables.
             * Remember to put them back into one list!
             *
             * Since len is known we just unwrap.
             */
            // parent_ref -> cur -> nxt -> etc
            let mut cur = parent_ref.next.take().unwrap();
            let mut nxt = cur.next.take().unwrap();
            let etc = nxt.next.take();
            cur.next = etc;
            nxt.next = Some(cur);
            parent_ref.next = Some(nxt);
            parent_ref = parent_ref.next.as_mut().unwrap().next.as_mut().unwrap();
        }
        dummy.next
    }
    fn len(mut head: &Option<Box<ListNode>>) -> usize {
        let mut ret = 0;
        loop {
            if head.is_some() {
                head = &head.as_ref().unwrap().next;
                ret += 1;
            } else {
                break;
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_soln() {
        let output = Solution::swap_pairs(ListNode::from(vec![1, 2, 3, 4]))
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();
        let ans = vec![2, 1, 4, 3];
        assert_eq!(ans, output);
        let output = Solution::swap_pairs(ListNode::from(vec![1]))
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();
        let ans = vec![1];
        assert_eq!(ans, output);
        let output = Solution::swap_pairs(ListNode::from(vec![]));
        let ans = None;
        assert_eq!(ans, output);
    }
}
