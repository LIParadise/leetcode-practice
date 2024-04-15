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
        // 1D DP table
        // At the beginning of each iteration,
        // the table stores answer to subproblem with different target sum,
        // using only part of the given candidates.
        //
        // Specifically,
        // `dp[0]` stores how to get sum 1,
        // `dp[1]` stores how to get sum 2,
        // so on and so forth.
        let mut dp = vec![Vec::<Vec<i32>>::new(); target as usize];
        candidates.iter().for_each(|&candidate| {
            // Q: what new combinations could be achieved using this idx?
            (1..=dp.len()).rev().for_each(|sum| {
                // Why reverse?
                // Suppose with the new candidate, there's multiple way to add up to sum
                // e.g. sum = 8, previously considered candidates are [2],
                // if in-order i.e. no reverse, we'd try first sum 5, yeilding 2+3*1=5
                // then try 8, yeilding 5+3*1=8,
                // and we'd record BOTH 2+3*2=8 AND 5+3*1=(2+3)+3*1=8
                // yeilding [2, 3, 3] twice, one since 2+3*2, one since 5+3*1=(2+3)+3*1
                // Reversing the order eliminates such issue.
                if (candidate as usize) <= sum {
                    // New candidate is only possible if it's not larger than
                    // designated target sum.
                    // Note that problem explicity allows for repeated selection
                    let mut new_solutions = Vec::new();
                    if sum % candidate as usize == 0 {
                        new_solutions.extend(vec![vec![candidate; sum / candidate as usize]]);
                    }
                    let mut repeat = 1;
                    while sum > (candidate as usize * repeat) {
                        let mut recorded_solutions =
                            dp[sum - 1 - repeat * candidate as usize].clone();
                        recorded_solutions.iter_mut().for_each(|vec| {
                            for _ in 0..repeat {
                                vec.push(candidate);
                            }
                        });
                        new_solutions.extend(recorded_solutions);
                        repeat += 1;
                    }
                    dp[sum - 1].extend(new_solutions);
                }
            });
        });
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
