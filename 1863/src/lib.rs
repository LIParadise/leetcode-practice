use std::ops::BitXor;

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
        let mut ret = 0;
        for subset_bitvec in 1..2_usize.pow(nums.len() as u32) {
            let mut xor = 0;
            let mut mask = subset_bitvec;
            nums.iter().for_each(|num| {
                if mask & 1 == 1 {
                    xor = xor.bitxor(num);
                }
                mask = mask >> 1;
            });
            ret += xor;
        }
        ret
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
