pub struct Solution;

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        if n == 0 {
            return 1;
        };
        i32::try_from(Self::integer_replacement_worker(u32::try_from(n).unwrap())).unwrap()
    }
    fn integer_replacement_worker(u: u32) -> u32 {
        use std::num::NonZeroUsize;
        let u = usize::try_from(u).unwrap();
        let mut dp = Vec::<Option<NonZeroUsize>>::with_capacity(u);
        let mut bfs_queue = std::collections::VecDeque::from([(1, 1)]);

        while let Some((n, ans)) = bfs_queue.pop_front() {
            if dp.get(n).is_none() {
                dp.resize(n + 1, None);
            }
            dp.get_mut(n)
                .unwrap()
                .get_or_insert(NonZeroUsize::new(ans).unwrap());

            if n == u {
                break;
            }

            bfs_queue.push_back((n * 2, ans + 1));
            if n % 2 == 0 {
                bfs_queue.push_back((n - 1, ans + 1));
                bfs_queue.push_back((n + 1, ans + 1));
            }
        }

        u32::try_from(
            dp.get(u)
                .unwrap()
                .as_ref()
                .copied()
                .map(NonZeroUsize::get)
                .unwrap()
                .checked_sub(1)
                .unwrap(),
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::integer_replacement(1), 0);
        assert_eq!(Solution::integer_replacement(0), 1);
        assert_eq!(Solution::integer_replacement(2), 1);
        assert_eq!(Solution::integer_replacement(3), 2);
    }
}

macro_rules! lprintln {
    // Match when a format string and additional arguments are provided
    ($fmt:expr, $($arg:tt)*) => {{
        if cfg!(feature = "local_test") {
            println!($fmt, $($arg)*);
        }
    }};

    // Match when only a format string is provided
    ($fmt:expr) => {{
        if cfg!(feature = "local_test") {
            println!($fmt);
        }
    }};
}

