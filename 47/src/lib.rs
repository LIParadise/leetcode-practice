pub struct Solution;

impl Solution {
    /// Given an array nums of distinct integers, return all the possible
    /// permutations.
    /// You can return the answer in any order.
    ///
    /// 1 <= nums.length <= 6
    /// -10 <= nums[i] <= 10
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut ret = Vec::new();
        let mut permutation = Vec::with_capacity(nums.len());
        let mut used = vec![false; nums.len()];
        Self::permute_unique_worker(&nums, &mut ret, &mut permutation, &mut used);
        ret
    }

    // See also
    // [issac3@LC](https://leetcode.com/problems/permutations/solutions/18239/a-general-approach-to-backtracking-questions-in-java-subsets-permutations-combination-sum-palindrome-partioning)
    //
    // Basic idea, naive algorithm:
    // At recursive call depth d (depth of first call is deemed as 1),
    // consider for each element if it were to appear at index (d-1) in some permutation,
    // then dive deeper.
    // Note that [LC Q.46](https://leetcode.com/problems/permutations/description) is just
    // this algorithm without further constraints.
    //
    // The gist is eliminating duplicate permutations since input may contain identical elements
    // e.g. [a, b, b] gives only 3 unique permutations while 3! = 6
    //
    // Assuming the array is sorted.
    // Suppose the elements in input array are additionally tagged with index,
    // s.t. equal elements could now be differentiated.
    // E.g. [a_0, a_1, b_2, b_3] for [a, a, b, b]
    // Under what conditions should we ignore the permutations we found in the naive algorithm?
    //
    // For duplicate permutations, we choose (choice function) the one in which elements' index are
    // in increasing order!
    //
    // In the algorithm, it considers index by index which element is to be put at the index.
    // For the particular permutation dictated by our choice function,
    // consider the element e_i in recursion depth d, it must be s.t. those equal to e_i
    // i.e. e_j where j < i have indices already determined in previous shallower recursion
    // depth i.e. recursion depth c where c < d.
    //
    // In layman's terms, the permutation must be like this:
    // [..., e_0, ..., e_1, ..., e_2, ..., e_j, ..., e_i]
    //                                                ^ index (d-1)
    //
    // This is where the boolean array comes into picture.
    // The bool values act as flags for if element is in the permutation we're considering.
    // It's then immediate to see that if we're in recursion depth d and considering e_i,
    // it's the permutation dictated by the choice function iff all e_j where j < i are used.
    // In particular, if it's a duplicate permutation that were to be removed, there must be some
    // e_i where it's not chosen and we're already considering e_{i+1}, i.e. e_i is false while
    // we're considering e_{i+1}. In that case, ignore it.
    //
    // Remark:
    // One cannot just simply abort when e_i false and e_{i+1} true. The reason being that the
    // order of usage may be fixed in deeper recursion levels, that is there may still be valid
    // permutations yet to be discovered; it's just we shall consider them in deeper (i.e. later)
    // recursion levels.
    fn permute_unique_worker(
        nums: &[i32],
        permutations: &mut Vec<Vec<i32>>,
        permutation: &mut Vec<i32>,
        used: &mut [bool],
    ) {
        if permutation.len() == nums.len() {
            permutations.push(permutation.clone());
        } else {
            (0..nums.len()).for_each(|idx| {
                if !(used[idx] || idx > 0 && nums[idx - 1] == nums[idx] && !used[idx - 1]) {
                    permutation.push(nums[idx]);
                    used[idx] = true;
                    Self::permute_unique_worker(nums, permutations, permutation, used);
                    permutation.pop();
                    used[idx] = false;
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::fmt::Debug;

    use crate::Solution;
    #[test]
    fn test_soln() {
        let output = Solution::permute_unique(vec![1, 2, 3]);
        let ans = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        test_soln_worker(ans, output);
        let output = Solution::permute_unique(vec![4, 2]);
        let ans = vec![vec![4, 2], vec![2, 4]];
        test_soln_worker(ans, output);
        let output = Solution::permute_unique(vec![1, 1, 2]);
        let ans = vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]];
        test_soln_worker(ans, output);
    }

    fn test_soln_worker<T: Eq + Ord + Debug>(a: Vec<T>, b: Vec<T>) {
        assert!(a.len() == b.len());
        let a_set = a.into_iter().collect::<BTreeSet<T>>();
        let b_set = b.into_iter().collect::<BTreeSet<T>>();
        assert!(a_set.symmetric_difference(&b_set).count() == 0);
    }
}
