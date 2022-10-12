use std::cmp::max;
use std::sync::Arc;

pub struct Solution;

/// You are a professional robber planning to rob houses along a street. Each
/// house has a certain amount of money stashed.
/// All houses at this place are arranged in a circle.
/// That means the first house is the neighbor of the last one. Meanwhile,
/// adjacent houses have a security system connected,
/// and it will automatically contact the police if two adjacent houses were
/// broken into on the same night.
///
/// Given an integer array nums representing the amount of money of each
/// house, return the maximum amount of money you can rob tonight without
/// alerting the police.
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() <= 3 {
            nums.iter().max().map_or(0, |max| *max)
        } else {
            // Two cases
            // 1. The 1st is chosen
            // 2. The 1st is NOT chosen
            let nums = std::sync::Arc::new(nums);
            let case_1_thrd = std::thread::spawn({
                let nums = Arc::clone(&nums);
                move || nums[0] + Self::rob_linear(nums.get(2..nums.len() - 1).unwrap())
            });
            let case_2_thrd = std::thread::spawn({
                let nums = Arc::clone(&nums);
                move || Self::rob_linear(nums.get(1..).unwrap())
            });
            let case_1 = case_1_thrd.join().unwrap();
            let case_2 = case_2_thrd.join().unwrap();
            max(case_1, case_2)
        }
    }
    /// Linear version of the problem, i.e. LC 198 House Robber
    /// Given an array, return some of choices, where choices are s.t. if an
    /// index is chosen then the adjacent 1/2 indices are NOT chosen.
    /// Linear in that `.first()` and `.last()` are not adjacent if len >= 3
    fn rob_linear(nums: &[i32]) -> i32 {
        let minus_1 = nums.first();
        if let None = minus_1 {
            0
        } else {
            let mut prev_ans = *minus_1.unwrap();
            if prev_ans < 0 {
                prev_ans = 0;
            }
            let mut prev_prev_ans = 0;
            for i in nums.iter().skip(1) {
                // DP, greedy
                /*
                 * At the beginning of each loop, `prev_ans` stores answer
                 * using up to the previous index of that of `i`
                 * Similarly `prev_prev_ans` stores answer using up to the
                 * previous of previous of that of `i`.
                 */
                let cur = std::cmp::max(prev_prev_ans + i, prev_ans);
                prev_prev_ans = prev_ans;
                prev_ans = cur;
            }
            prev_ans
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }
}
