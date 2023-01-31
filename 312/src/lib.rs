pub struct Solution;
use triang_array::TriangArr;

impl Solution {
    /// You are given n balloons, indexed from 0 to n - 1.
    /// Each balloon is painted with a number on it represented by an array nums.
    /// You are asked to burst all the balloons.
    /// If you burst the ith balloon, you will get
    /// nums[i - 1] * nums[i] * nums[i + 1] coins.
    /// If i - 1 or i + 1 goes out of bounds of the array, then treat it as if
    /// there is a balloon with a 1 painted on it.
    /// Return the maximum coins you can collect by bursting the balloons wisely.
    // Somehow this problem is akin to matrix chain multiplication.
    // TODO add clarification
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let eff_mat_chain_len = nums.len() + 1;
        let nums = {
            let mut nums = nums;
            let mut ret = Vec::with_capacity(nums.len() + 2);
            ret.push(1);
            ret.append(&mut nums);
            ret.push(1);
            ret
        };
        let mut dp: TriangArr<i32> = TriangArr::new(eff_mat_chain_len);
        (2..=eff_mat_chain_len).for_each(|mat_subset_len| {
            // Total length is `eff_mat_chain_len`
            // We can make many contiguous subsets with `mat_subset_len`
            // E.g. supp. 5 matrices ABCDE, subset length 3, then
            // ABC, BCD, CDE
            // We got 3 valid choices.
            // `mat_subset_idx` records such idx.
            (0..eff_mat_chain_len + 1 - mat_subset_len).for_each(|mat_subset_idx| {
                let mut max = i32::MIN;
                (1..mat_subset_len).for_each(|mat_subsubset_len| {
                    max = std::cmp::max(
                        max,
                        // right, middle, left
                        dp.get(
                            mat_subset_len - mat_subsubset_len - 1,
                            mat_subset_idx + mat_subsubset_len,
                        )
                        .unwrap()
                            + nums[mat_subset_idx]
                                * nums[mat_subset_idx + mat_subsubset_len]
                                * nums[mat_subset_idx + mat_subset_len]
                            + dp.get(mat_subsubset_len - 1, mat_subset_idx).unwrap(),
                    );
                });
                *dp.get_mut(mat_subset_len - 1, mat_subset_idx).unwrap() = max;
            })
        });
        dbg!(&dp);
        *dp.get(eff_mat_chain_len - 1, 0).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution as S;
    #[test]
    fn test_soln() {
        assert_eq!(S::max_coins(Vec::from([3, 1, 5, 8])), 167);
        assert_eq!(S::max_coins(Vec::from([1, 5])), 10);
    }
}
