//! You are given a 0-indexed array of integers nums of length n.
//! You are initially positioned at nums[0].
//! Each element nums[i] represents the maximum length of a forward
//! jump from index i. In other words, if you are at nums[i],
//! you can jump to any nums[i + j] where:
//! 0 <= j <= nums[i] and
//! i + j < n
//! Return the minimum number of jumps to reach nums[n - 1].
//! The test cases are generated such that you can reach nums[n - 1].
pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        // Idea:
        // Make an array s.t. its [n] records how far one can go
        // using [0..=n] of input.
        //
        // Algorithm:
        // With such an array at hand, start from [0], and just
        // go to what it reads (i.e. the farest one can jump to from
        // there), till we realize that the end point is in reach, then
        // count how many times one had jumped.
        //
        // Proof:
        // By definition, such an array is non-decreasing.
        // Suppose optimal jump chooses these indices:
        // [i_0 = 0, i_1, i_2, ..., i_n = len-1]
        // Suppose our choises of indices are
        // [j_0 = 0, j_1, j_2, ..., j_m = len-1]
        // Notice that by our algorithm and construction of the array,
        // j_k >= i_k for all k.
        // This is shown by induction and the fact the array being
        // non-decreasing.
        // Hence m <= n i.e. our algorithm must be "quicker" to be s.t.
        // the end is within a grasp, again utilizing the fact said
        // array being non-decreasing.
        //
        // And turns our that constructing such an array is just a linear
        // scan.

        let mut reachable_indices = nums;
        (1..reachable_indices.len()).for_each(|idx| {
            let (known, to_mod) = reachable_indices.split_at_mut(idx);
            if let Some(to_mod) = to_mod.get_mut(0) {
                *to_mod = std::cmp::max(known.last().unwrap().clone(), idx as i32 + *to_mod);
            }
        });
        dbg!(&reachable_indices);
        let (mut idx, mut ret) = (0, 0);
        while let Some(can_jump_to) = reachable_indices.get(idx) {
            // LC promises that input is s.t. always able to jump to end.
            // Hence no sanity check.
            if idx >= reachable_indices.len() - 1 {
                break;
            }
            ret += 1;
            idx = *can_jump_to as usize;
        }
        ret as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
        assert_eq!(Solution::jump(vec![0]), 0);
    }
}
