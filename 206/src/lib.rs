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
        let iter = iter.into_iter();
        let mut ret = None;
        let mut tail = &mut ret;
        for node in iter {
            tail.replace(node);
            /*
             * `while let` would be more elegant,
             * but the borrow checker doesn't work out:
             * the pattern becomes borrowed across iterator loop.
             */
            /*
             * if node itself is already head of list,
             * make it s.t. result is concatenation of lists
             */
            while tail.is_some() {
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
                while let Some((mut node, remained)) = orig_tail.map(ListNode::split_head_tail) {
                    node.next = Some(ret);

                    {
                        /*
                         * This line is all the magic:
                         *
                         * We know it had just been moved into some other variable;
                         * no `impl Drop` was called, since that variable was `Option::None`.
                         *
                         * The identifier is now invalid to be used,
                         * since it's now sort of uninitialized: it had been moved!!!
                         * BUT, what we're allowed to do is assign it a new value,
                         * e.g. move other value in!
                         *
                         * In some sense it's just like one may say `let identifier;`
                         * and not assign it any value till later in the code:
                         * the identifier is free real estate.
                         */
                        ret = node;

                        /*
                         * this line does similar trick:
                         * the value had been moved right in the beginning of the loop,
                         * i.e. the `Option::map`,
                         * so the identifier is free to accept new value to be moved in
                         */
                        orig_tail = remained;
                    }
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

        let vvv = (0..3)
            .flat_map(|_| data.iter())
            .cloned()
            .map(<T as Into<i32>>::into)
            .collect::<Vec<_>>();
        let lll =
            Option::<Box<ListNode>>::from_iter((0..3).flat_map(|_| make_list(data).into_iter()));
        dbg!(&lll);
        assert_eq!(
            vvv.into_iter()
                .zip(lll.into_iter().flatten())
                .inspect(|(v, l)| { assert_eq!(v, &l.val) })
                .count(),
            data.len() * 3
        );
    }
}
