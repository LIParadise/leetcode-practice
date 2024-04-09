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
        if arr.is_empty() {
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

pub struct Solution;

impl Solution {
    fn add_worker(i: i32, j: i32, c: &mut i32, cur_node: &mut Option<Box<ListNode>>) {
        let sum = *c + i + j;
        *c = match sum {
            0..=9 => 0,
            10..=19 => 1,
            20.. => 2,
            _ => panic!("Individual digit of decimal shall not be negative."),
        };
        let n = Box::new(ListNode {
            val: sum % 10,
            next: None,
        });
        match cur_node {
            None => *cur_node = Some(n),
            Some(c) => {
                c.next = Some(n);
            }
        }
    }
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut ret = None;
        let mut cur_node = &mut ret;
        let mut l1 = &l1;
        let mut l2 = &l2;
        loop {
            match (l1, l2) {
                (Some(a), Some(b)) => {
                    Self::add_worker(a.val, b.val, &mut carry, cur_node);
                    l1 = &a.next;
                    l2 = &b.next;
                }
                (Some(a), None) => {
                    Self::add_worker(a.val, 0, &mut carry, cur_node);
                    l1 = &a.next;
                }
                (None, Some(b)) => {
                    Self::add_worker(b.val, 0, &mut carry, cur_node);
                    l2 = &b.next;
                }
                (None, None) => {
                    if carry > 0 {
                        Self::add_worker(0, 0, &mut carry, cur_node);
                    }
                    break;
                }
            };
            cur_node = &mut cur_node.as_mut().unwrap().next;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(crate::ListNode::from([2, 4, 3].as_ref()))),
                Some(Box::new(crate::ListNode::from([5, 6, 4].as_ref())))
            ),
            Some(Box::new(crate::ListNode::from([7, 0, 8].as_ref())))
        );
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(crate::ListNode::from([0].as_ref()))),
                Some(Box::new(crate::ListNode::from([0].as_ref())))
            ),
            Some(Box::new(crate::ListNode::from([0].as_ref())))
        );
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(crate::ListNode::from(
                    [9, 9, 9, 9, 9, 9, 9].as_ref()
                ))),
                Some(Box::new(crate::ListNode::from([9, 9, 9, 9].as_ref())))
            ),
            Some(Box::new(crate::ListNode::from(
                [8, 9, 9, 9, 0, 0, 0, 1].as_ref()
            )))
        );
    }
}
