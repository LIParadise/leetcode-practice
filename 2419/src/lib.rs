pub struct Solution;

impl Solution {
    // Note that bit-wise AND operation always results in smaller number
    // Hence the problem is just finding the max member
    // and check how long a subarray consisting only of that member could be.
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let max = nums
            .iter()
            .max()
            .expect("Leetcode specifies non-empty array, but empty. Abort.");
        nums.iter()
            .fold((0, 0), |(max_len, cur_len), n| {
                if n == max {
                    let new_cur_len = cur_len + 1;
                    (std::cmp::max(max_len, new_cur_len), new_cur_len)
                } else {
                    (max_len, 0)
                }
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::longest_subarray(vec![1, 2, 3, 3, 2, 2]), 2);
        assert_eq!(Solution::longest_subarray(vec![1, 2, 3, 4]), 1);
    }
}
