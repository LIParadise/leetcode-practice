pub struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        Self::four_sum_worker(&nums, target)
    }
    fn four_sum_worker(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        let mut a = 0;
        while a + 3 < nums.len() {
            let mut d = nums.len() - 1;
            while a + 2 < d {
                // two pointers
                let mut b = a + 1;
                let mut c = d - 1;
                let target = target as i64 - nums[a] as i64 - nums[d] as i64;
                while b < c {
                    match (nums[b] as i64 + nums[c] as i64).cmp(&target) {
                        std::cmp::Ordering::Less => b += 1,
                        std::cmp::Ordering::Greater => c -= 1,
                        std::cmp::Ordering::Equal => {
                            let b_count = (b..=c).take_while(|i| nums[*i] == nums[b]).count();
                            let c_count = (b..=c).rev().take_while(|i| nums[*i] == nums[c]).count();
                            ret.push(vec![nums[a], nums[b], nums[c], nums[d]]);
                            b += b_count;
                            c -= c_count;
                        }
                    }
                }
                d -= (a + 1..=d)
                    .rev()
                    .take_while(|i| nums[d] == nums[*i])
                    .count();
            }
            a += (a..nums.len()).take_while(|i| nums[a] == nums[*i]).count();
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::Solution;
    #[test]
    fn test_soln() {
        let mine = Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0);
        let soln = HashSet::<Vec<_>>::from_iter([
            vec![-2, -1, 1, 2],
            vec![-2, 0, 0, 2],
            vec![-1, 0, 0, 1],
        ]);
        assert_eq!(mine.len(), soln.len());
        assert!(
            soln.symmetric_difference(&HashSet::from_iter(mine))
                .next()
                .is_none()
        );

        let mine = Solution::four_sum(vec![2, 2, 2, 2, 2, 2], 8);
        let soln = HashSet::<Vec<_>>::from_iter([vec![2, 2, 2, 2]]);
        assert_eq!(mine.len(), soln.len());
        assert!(
            soln.symmetric_difference(&HashSet::from_iter(mine))
                .next()
                .is_none()
        );
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

