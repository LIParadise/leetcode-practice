use std::mem::swap;

/// Given a zero-based permutation nums (0-indexed), build an array ans
/// of the same length where ans[i] = nums[nums[i]] for each
/// 0 <= i < nums.length
/// and return it.
///
/// A zero-based permutation nums is an array of distinct integers
/// from 0 to nums.length - 1 (inclusive).
pub struct Solution;

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        // In-Place algorithm.
        // Hack: how to identify those had already been placed to correct
        // location?
        // Bit-Operation?
        // Since these are `i32` supposed to be indices,
        // either set them to negative or plus them by len!
        //
        // Here I chose to use negative.
        // Remember to offset by 1 since 0.
        let mut nums = nums;
        (0..nums.len()).for_each(|i| {
            if nums[i] >= 0 {
                // not yet moved. move them.
                let (mut current_ptr, mut next_ptr) = (i, nums[i]);
                let backup = nums[i];
                while next_ptr as usize != i {
                    nums[current_ptr] = Self::mark(nums[next_ptr as usize]);
                    current_ptr = next_ptr as usize;
                    next_ptr = nums[current_ptr as usize];
                }
                nums[current_ptr] = Self::mark(backup);
            }
        });
        nums.iter_mut().for_each(|i| *i = Self::mark(*i));
        nums
    }
    #[inline(always)]
    fn mark(i: i32) -> i32 {
        -i - 1
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::build_array(vec![1, 3, 4, 0, 2]),
            vec![3, 0, 2, 1, 4]
        );
        assert_eq!(Solution::build_array(vec![1, 0]), vec![0, 1]);
        assert_eq!(Solution::build_array(vec![0, 1]), vec![0, 1]);
        assert_eq!(Solution::build_array(vec![0]), vec![0]);
    }
}
