pub struct Solution;

// Definition for singly-linked list.
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

impl From<&[i32]> for ListNode {
    fn from(arr: &[i32]) -> Self {
        if arr.len() == 0 {
            panic!("Cowardly refuse to contruct empty list since the type can't convey the concept of being empty.");
        } else {
            let mut ret = Self::new(arr[0]);
            let mut ptr = &mut ret;
            for num in arr.iter().skip(1) {
                ptr.next = Some(Box::new(Self::new(*num)));
                ptr = ptr.next.as_mut().unwrap();
            }
            ret
        }
    }
}

/// Given the head of a singly linked list, return true if it is a palindrome
/// or false otherwise.
///
/// Bonus: O(1) space and O(n) time?
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let len = Self::len(&head);
        if len <= 1 {
            return true;
        }
        let (pt1, pt2) = if len % 2 == 1 {
            let (pt1, pt2) = Self::cut(head.unwrap(), len / 2);
            let (_, pt2) = Self::cut(pt2.unwrap(), 1);
            (pt1, Some(Self::rev(pt2.unwrap())))
        } else {
            let (pt1, pt2) = Self::cut(head.unwrap(), len / 2);
            (pt1, Some(Self::rev(pt2.unwrap())))
        };
        pt1.eq(&pt2)
    }

    /// In-place reverse a singly linked list
    fn rev(mut head: Box<ListNode>) -> Box<ListNode> {
        struct TmpNodeOwner {
            data: Option<Box<ListNode>>,
        }
        let remaining = head.as_mut().next.take();
        let mut reversed = TmpNodeOwner { data: Some(head) };
        let mut remaining = TmpNodeOwner { data: remaining };
        // Suppose list is 1 -> 2 -> 3 -> 4
        // First, to work with ownership, create TmpNodeOwner
        // s.t. the list is decompsed as "reversed:  % -> 1"
        //                               "remaining: % -> 2 -> 3 -> 4"
        while let Some(mut r) = remaining.data {
            // After pattern match we have "reversed:  % -> 1"
            //                             "remaining: %"
            //                             "r:         2 -> 3 -> 4"
            // Note `remaining` is partially moved temporarily!
            remaining.data = r.next.replace(reversed.data.take().unwrap());
            // RHS makes "reversed:  %"
            //           "r:         2 -> 1"
            //           "remaining: %"
            // and "3 -> 4" being result of expression thx to `Option::replace`
            //
            // Assignment to LHS makes "reversed:  %"
            //                         "r:         2 -> 1"
            //                         "remaining: % -> 3 -> 4"
            // Now `remaining` is no longer partially moved and valid
            // to be used again.
            reversed.data.replace(r);
            // Finally we bring them together, yeilding
            // "reversed:  % -> 2 -> 1"
            // "remaining: % -> 3 -> 4"
        }
        reversed.data.unwrap()
    }
    
    /// Get length of singly linked list
    fn len(mut head: &Option<Box<ListNode>>) -> usize {
        let mut ret = 0;
        while let Some(node) = &head {
            ret += 1;
            head = &node.next;
        }
        ret
    }

    /// Cut the list at given index, returning the resulting 2 lists
    ///
    /// Too lazy to implement boundary check so wraps around in a weird way,
    /// so be sure to verify index is LESS then len!
    ///
    /// Examples:
    /// Suppose list is (a -> b -> c)
    /// 1. cut 0
    ///    (None, Some(a -> b -> c))
    /// 2. cut 1
    ///    (Some(a), Some(b -> c))
    fn cut(head: Box<ListNode>, idx: usize) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut first_half = Some(head);
        let mut end_of_first_half = &mut None;
        for _ in 0..idx {
            match end_of_first_half {
                None => end_of_first_half = &mut first_half,
                Some(cur) => end_of_first_half = &mut cur.next,
            }
        }
        if let None = end_of_first_half {
            (None, first_half)
        } else {
            let second_half = end_of_first_half.as_mut().unwrap().next.take();
            (first_half, second_half)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ListNode, Solution};
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::is_palindrome(Some(Box::new(ListNode::from(vec![1, 2, 2, 1].as_ref())))),
            true
        );
        assert_eq!(
            Solution::is_palindrome(Some(Box::new(ListNode::from(vec![1, 2].as_ref())))),
            false
        );
        assert_eq!(
            Solution::is_palindrome(Some(Box::new(ListNode::from(vec![1, 0, 1].as_ref())))),
            true
        );
    }
}
