pub struct Solution;

/// You are a professional robber planning to rob houses along a street. Each
/// house has a certain amount of money stashed, the only constraint stopping
/// you from robbing each of them is that adjacent houses have security systems
/// connected and it will automatically contact the police if two adjacent
/// houses were broken into on the same night.
///
/// Given an integer array nums representing the amount of money of each house,
/// return the maximum amount of money you can rob tonight without alerting
/// the police.
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        // 1 <= nums.length <= 100
        // 0 <= nums[i] <= 400

        let mut take = *nums.first().expect("Expected non-zero length");
        if take < 0 {
            take = 0;
        }
        let mut ignr = 0;
        for i in nums.iter().skip(1) {
            /*
             * take => max revenue if last index were a rob
             * ignr => max revenue if last index were ignored, i.e. NOT robbed
             */
            let new_take = std::cmp::max(i + ignr, take);
            ignr = take;
            take = new_take;
        }
        /*
         * Notice take >= ignr throughout the loop
         */
        take
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }
}
