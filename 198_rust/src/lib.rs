pub struct Solution;

impl Solution {
    /// You are a professional robber planning to rob houses along a street.
    /// Each house has a certain amount of money stashed, the only constraint
    /// stopping you from robbing each of them is that adjacent houses have
    /// security systems connected and it will automatically contact the police
    /// if two adjacent houses were broken into on the same night.
    ///
    /// Given an integer array nums representing the amount of money of each
    /// house, return the maximum amount of money you can rob tonight without
    /// alerting the police.
    ///
    /// Input are guaranteed to be non-negative.
    ///
    /// Example:
    /// ```
    /// use lc_198_house_robber::Solution as S;
    /// assert_eq!(S::rob(Vec::from(
    ///     [1, 2, 3, 1])), 4);
    /// //   ^     ^
    /// // 1 + 3 = 4
    /// assert_eq!(S::rob(Vec::from(
    ///     [2, 7, 9, 3, 1])), 12);
    /// //   ^     ^     ^
    /// // 2 + 9 + 1 = 12
    /// assert_eq!(S::rob(Vec::from(
    ///     [3, 1, 2, 4])), 7);
    /// //   ^        ^
    /// // 3 + 4 = 7
    /// ```
    ///
    /// Notice by non-negativity, it's guaranteed to be non-decreasing
    /// if one were to consider larger and larger subarray.
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            nums.iter().max().map_or(0, |&max| max)
        } else {
            let mut best_second_last = nums[0];
            let mut best_last = std::cmp::max(nums[0], nums[1]);
            let mut safe = nums[1] <= nums[0];
            nums.iter()
                .skip(1)
                .zip(nums.iter().skip(2))
                .for_each(|(prev, cur)| {
                    let tmp = best_second_last;
                    best_second_last = best_last;
                    match (safe, cur > prev) {
                        (true, _) => {
                            best_last += cur;
                            safe = false;
                        }
                        (false, true) => {
                            best_last = tmp + cur;
                        }
                        (false, false) => {
                            if tmp + cur > best_last {
                                best_last = tmp + cur;
                            } else {
                                safe = true;
                            }
                        }
                    }
                });
            best_last
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::rob(Vec::from([2, 3, 2])), 4);
    }
}
