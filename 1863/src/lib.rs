use std::ops::BitOr;

pub struct Solution;

/// The XOR total of an array is defined as the bitwise XOR of all its elements,
/// or 0 if the array is empty.
///
/// For example, the XOR total of the array [2,5,6] is 2 XOR 5 XOR 6 = 1.
///
/// Given an array nums, return the sum of all XOR totals for every subset of
/// nums.
///
/// Note: Subsets with the same elements should be counted multiple times.
///
/// An array a is a subset of an array b if a can be obtained from b by deleting
/// some (possibly zero) elements of b.
impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        // Observation: given OR of whole array be X,
        // note that each bit of X appear in exactly half of all (XOR of subset)
        // Proof by induction on number of elements with n-th bit being 1.
        // Supp. there's only one element, trivial.
        // Supp. when no larger than K elements with n-th bit being 1 it's ok,
        // consider when there's (K+1) elements with n-th bit being 1.
        // An easy calculation gives 2^{n-2} + 2^{n-2} = 2^{n-1}.
        let xor_of_all = nums.iter().fold(0, |accm, num| accm.bitor(num));
        dbg!(xor_of_all);
        xor_of_all * 2_i32.pow(nums.len() as u32 - 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::subset_xor_sum(vec![1, 3]), 6);
        assert_eq!(Solution::subset_xor_sum(vec![5, 1, 6]), 28);
        assert_eq!(Solution::subset_xor_sum(vec![3, 4, 5, 6, 7, 8]), 480);
    }
}
