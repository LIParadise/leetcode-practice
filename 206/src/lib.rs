pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(unused)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl ListNode {
    fn split_head_tail(mut self: Box<Self>) -> (Box<Self>, Option<Box<Self>>) {
        let tail = self.next.take();
        (self, tail)
    }
}

impl FromIterator<Box<ListNode>> for Option<Box<ListNode>> {
    fn from_iter<T: IntoIterator<Item = Box<ListNode>>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let mut ret = iter.next();
        let mut tail;
        if ret.is_some() {
            tail = &mut ret.as_mut().unwrap().next;
            for list_node in iter {
                *tail = Some(list_node);
                tail = &mut tail.as_mut().unwrap().next;
            }
        }
        ret
    }
}

pub struct ListIter {
    list: Option<Box<ListNode>>,
}
impl Iterator for ListIter {
    type Item = Box<ListNode>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(list) = self.list.take() {
            let (ret, tail) = list.split_head_tail();
            self.list = tail;
            Some(ret)
        } else {
            None
        }
    }
}
impl IntoIterator for Box<ListNode> {
    type Item = Self;
    type IntoIter = ListIter;
    fn into_iter(self) -> Self::IntoIter {
        ListIter { list: Some(self) }
    }
}

impl Solution {
    pub fn reverse_list(orig_list: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match orig_list {
            None => None,
            Some(orig_list) => {
                let (mut ret, mut orig_tail) = orig_list.split_head_tail();
                while let Some(tail) = orig_tail {
                    let (mut node, remained) = tail.split_head_tail();
                    node.next = Some(ret);
                    ret = node;
                    orig_tail = remained;
                }
                Some(ret)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ListNode, Solution};
    #[test]
    fn test_soln() {
        let data = [1, 1, 4, 5, 1, 4];
        test_soln_helper(&data);
        test_iter_traits_helper(&data);

        let data = [] as [i32; 0];
        test_soln_helper(&data);
        test_iter_traits_helper(&data);

        let data = b"yjsp_114514_1919810";
        test_soln_helper(data);
        test_iter_traits_helper(data);

        let data = b"x";
        test_soln_helper(data);
        test_iter_traits_helper(data);
    }
    fn test_soln_helper<T>(data: &[T])
    where
        T: Into<i32> + Copy,
    {
        let reversed_data = data.iter().cloned().rev().collect::<Vec<_>>();
        let orig_list = Option::<Box<ListNode>>::from_iter(data.iter().cloned().map(|i| {
            Box::new(ListNode {
                next: None,
                val: i.into(),
            })
        }));
        let reversed_list = Solution::reverse_list(orig_list);
        if let Some(reversed_list) = reversed_list {
            assert!(
                reversed_data
                    .into_iter()
                    .zip(reversed_list)
                    .inspect(|(ans, mine)| { assert!(<T as Into<i32>>::into(*ans) == mine.val) })
                    .count()
                    == data.len()
            )
        } else {
            assert!(data.is_empty())
        }
    }
    fn test_iter_traits_helper<T>(data: &[T])
    where
        T: Into<i32> + Copy,
    {
        fn make_list<T>(data: &[T]) -> Option<Box<ListNode>>
        where
            T: Into<i32> + Copy,
        {
            Option::<Box<ListNode>>::from_iter(data.iter().map(|i| {
                Box::new(ListNode {
                    next: None,
                    val: <T as Into<i32>>::into(*i),
                })
            }))
        }
        let v = Vec::from_iter(make_list(data).into_iter().flatten());
        let l = make_list(data);
        if let Some(l) = l {
            assert_eq!(
                data.iter()
                    .zip(v)
                    .zip(l)
                    .inspect(|((&d, v), l)| {
                        assert_eq!(d.into(), v.val);
                        assert_eq!(v.val, l.val);
                    })
                    .count(),
                data.len()
            );
        } else {
            assert!(data.is_empty());
            assert!(v.is_empty());
        }
    }
}
