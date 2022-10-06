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
        // Since numbers could be re-used, supp. we chose c in candidates,
        // then we'd have to solve (candidates, target-c), a similar problem!
        use std::collections::BTreeSet as Set;
        let target = target as usize;
        let mut dp: Vec<Vec<Vec<i32>>> = Vec::with_capacity(target);
        let candidates = {
            let mut ret = candidates;
            ret.sort_unstable();
            ret
        };
        while dp.len() <= target {
            // Each loop solve for problem with target equal to dp.len()
            // I.e. subproblem with target less than dp.len() are ready
            let mut set: Set<Vec<i32>> = Set::new();
            for c in candidates.iter() {
                if *c as usize > dp.len() {
                    break;
                } else if *c as usize == dp.len() {
                    set.insert(vec![*c]);
                } else {
                    dp[dp.len() - *c as usize].iter().for_each(|vec| {
                        set.insert({
                            let mut ret = vec.clone();
                            ret.insert(ret.partition_point(|i| c > i), *c);
                            ret
                        });
                    });
                }
            }
            dp.push(set.into_iter().collect::<Vec<_>>())
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
