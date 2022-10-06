pub struct Solution;

/*
 * Start of implicit definition for singly-linked list.
 */
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
 * End of implicit definition for singly-linked list.
 */

/// Given the head of a linked list, remove the nth node from the end of the
/// list and return its head.
///
///
/// The number of nodes in the list is sz.
/// 1 <= sz <= 30
/// 0 <= Node.val <= 100
/// 1 <= n <= sz
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // 2-pass
        // First pass check length
        // Second pass do the substitution
        let n = n as usize;
        // This above explicity conversion would pad sign after which perform
        // 2's complement
        // https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions
        let len = Self::len(&head);
        if n > len {
            return head;
        }
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut rev_idx = len;
        let mut parent = &mut dummy_head;
        while rev_idx > n {
            rev_idx -= 1;
            parent = &mut parent.as_mut().unwrap().next;
        }
        let mut node_to_remove = parent.as_mut().unwrap().next.take();
        let list_to_preserve = node_to_remove.as_mut().unwrap().next.take();
        parent.as_mut().unwrap().next = list_to_preserve;
        dummy_head.unwrap().next
    }
    fn len(mut option_node: &Option<Box<ListNode>>) -> usize {
        let mut ret = 0;
        while let Some(node) = option_node {
            ret += 1;
            option_node = &node.next;
        }
        ret
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
