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
impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut remaining = head;
        let mut ret = None;
        let mut in_between_zeros = None;
        let mut tail_of_ret = &mut ret;
        let mut tail_of_in_between_zeros = &mut in_between_zeros;
        while let Some(mut head) = remaining {
            match head.val == 0 {
                true => {
                    remaining = head.next;
                    tail_of_in_between_zeros = &mut in_between_zeros;
                    Self::consolidate_append(tail_of_ret, tail_of_in_between_zeros.take());
                    if let Some(t) = tail_of_ret {
                        tail_of_ret = &mut t.next;
                    }
                }
                false => {
                    remaining = head.next.take();
                    match tail_of_in_between_zeros {
                        None => *tail_of_in_between_zeros = Some(head),
                        Some(t) => {
                            t.next = Some(head);
                            tail_of_in_between_zeros = &mut t.next;
                        }
                    }
                }
            }
        }
        ret
    }
    fn consolidate_append(tail: &mut Option<Box<ListNode>>, list: Option<Box<ListNode>>) {
        let node_to_append = match list {
            None => None,
            node @ Some(_) => {
                let mut ptr = &node;
                let mut append = 0;
                while let Some(node) = ptr {
                    append += node.val;
                    ptr = &node.next;
                }
                let mut node = node.unwrap();
                node.val = append;
                node.next.take();
                Some(node)
            }
        };
        match tail {
            None => *tail = node_to_append,
            Some(t) => {
                t.next = node_to_append;
            }
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
