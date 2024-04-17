pub struct Solution;

/// Given an array of distinct integers candidates and a target integer target,
/// return a list of all unique combinations of candidates where the chosen
/// numbers sum to target. You may return the combinations in any order.
///
/// The same number may be chosen from candidates an unlimited number of times.
/// Two combinations are unique if the frequency of at least one of the chosen
/// numbers is different.
///
/// Constraints:
/// All elements of candidates are distinct.
/// 1 <= candidates.length <= 30
/// 2 <= candidates[i] <= 40
/// 1 <= target <= 500
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let mut backtrack = Vec::new();
        Self::comb_sum_worker(&mut backtrack, target, 0, &candidates, &mut ret);
        ret
    }
    // [issac3@LC](https://leetcode.com/problems/permutations/solutions/18239/a-general-approach-to-backtracking-questions-in-java-subsets-permutations-combination-sum-palindrome-partioning)
    fn comb_sum_worker(
        backtrack: &mut Vec<i32>,
        tgt: i32,
        start_idx: usize,
        candidates: &[i32],
        solutions: &mut Vec<Vec<i32>>,
    ) {
        if tgt == 0 {
            solutions.push(backtrack.to_owned());
        } else if tgt > 0 {
            // Note that candidates are unique
            //
            // Set search start index s.t. the algorithm won't ever find duplicate ways to sum to
            // target:
            // The algorithm always chooses the one with elements of which original indicies in the
            // candidates array is non-decreasing among all the possible ways to sum to target.
            candidates
                .iter()
                .enumerate()
                .skip(start_idx)
                .for_each(|(i, &c)| {
                    backtrack.push(c);
                    Self::comb_sum_worker(backtrack, tgt - c, i, candidates, solutions);
                    backtrack.pop();
                });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use std::collections::BTreeSet as BTS;
    #[test]
    fn test_soln() {
        let mine = Solution::combination_sum(vec![2, 3, 5], 8);
        let expected = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
        assert_eq!(mine.len(), expected.len());
        let mine = BTS::from_iter(mine.into_iter().map(|mut v| {
            v.sort_unstable();
            v
        }));
        let expected = BTS::from_iter(expected);
        assert!(mine.symmetric_difference(&expected).count() == 0);

        let mine = Solution::combination_sum(vec![2, 3, 6, 7], 7);
        let expected = vec![vec![2, 2, 3], vec![7]];
        assert_eq!(mine.len(), expected.len());
        let mine = BTS::from_iter(mine.into_iter().map(|mut v| {
            v.sort_unstable();
            v
        }));
        let expected = BTS::from_iter(expected);
        assert!(mine.symmetric_difference(&expected).count() == 0);

        let mine = Solution::combination_sum(vec![2], 1);
        let expected = Vec::new();
        assert_eq!(mine.len(), expected.len());
        let mine = BTS::from_iter(mine.into_iter().map(|mut v| {
            v.sort_unstable();
            v
        }));
        let expected = BTS::from_iter(expected);
        assert!(mine.symmetric_difference(&expected).count() == 0);
    }
}
