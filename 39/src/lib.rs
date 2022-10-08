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
        // Dynamic Programming
        //
        // We can consider DP[i][j] where i in 0..candidates.len() and j in
        // 1..=target, s.t. DP[i][j] represents the combinations that sum
        // to j using only candidates with indices no more than i.
        // DP structure is DP[i][j] = DP[i-1][j] + DP[i][j-candidates[i]]
        //
        // Notice that we only reference the rows (i-1) and (i), hence the
        // conceptually 2D table could be stored using 1D.

        if target <= 0 {
            return Vec::new();
        }
        let mut dp: Vec<Vec<Vec<i32>>> = Vec::with_capacity(target as usize);
        // 1D DP table
        // At the beginning of each iteration, the table stores answer to
        // subproblem with different target sum, using only part of
        // the given candidates.
        // Specifically, `dp[0]` stores how to get sum 1,
        // `dp[1]` stores how to get sum 2, so on and so forth.
        for _ in 0..target {
            dp.push(Vec::new())
        }
        for idx in 0..candidates.len() {
            // Q: what new combinations could be achieved using this idx?
            for sum in (1..=dp.len()).rev() {
                if (candidates[idx] as usize) <= sum {
                    // New candidate is only possible if it's not larger than
                    // designated target sum.
                    // Also, this problem explicity allows repeated selection
                    let mut tmp = Vec::new();
                    let mut repeat = 1;
                    while sum > (candidates[idx] as usize * repeat) {
                        let mut tmptmp = dp[sum - 1 - repeat * candidates[idx] as usize].clone();
                        tmptmp.iter_mut().for_each(|vec| {
                            for _ in 0..repeat {
                                vec.push(candidates[idx]);
                            }
                        });
                        tmp.extend(tmptmp);
                        repeat += 1;
                    }
                    if sum % candidates[idx] as usize == 0 {
                        tmp.extend({
                            let mut tmptmp = Vec::with_capacity(sum % candidates[idx] as usize);
                            for _ in 0..sum / candidates[idx] as usize {
                                tmptmp.push(candidates[idx]);
                            }
                            vec![tmptmp]
                        });
                    }
                    dp[sum - 1].extend(tmp);
                }
            }
        }
        dp.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        );
        let empty: Vec<Vec<i32>> = Vec::new();
        assert_eq!(Solution::combination_sum(vec![2], 1), empty);
    }
}
