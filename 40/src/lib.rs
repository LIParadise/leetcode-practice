pub struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut ret = Vec::new();
        Self::comb_sum2_worker(&candidates, target, 0, &mut Vec::new(), &mut ret);
        ret
    }

    // credits to
    // [issac3@LC](https://leetcode.com/problems/permutations/solutions/18239/a-general-approach-to-backtracking-questions-in-java-subsets-permutations-combination-sum-palindrome-partioning)
    // An elegant way of backtracking, using index of input.
    //
    // In particular, first, sort s.t. same elements are neighbors
    // Next, ask "to be or not to be" question to each index, and proceed the index while we're dt
    // it s.t. no element is used twice.
    // The trick is spotting where the chain of questions leads to duplicates.
    fn comb_sum2_worker(
        candidates: &[i32],
        tgt: i32,
        start_idx: usize,
        backtrack: &mut Vec<i32>,
        solutions: &mut Vec<Vec<i32>>,
    ) {
        if tgt == 0 {
            solutions.push(backtrack.to_owned());
        } else if tgt > 0 {
            candidates
                .iter()
                .enumerate()
                .skip(start_idx)
                .for_each(|(i, &c)| {
                    if i > start_idx && candidates[i - 1] == c {
                        // ensure that c is "fresh"
                        // in that if the criteria holds, all it would produce must have been
                        // produced in previous loop
                        // i.e. all it produces must be duplicates
                        return;
                    }
                    backtrack.push(c);
                    Self::comb_sum2_worker(candidates, tgt - c, i + 1, backtrack, solutions);
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
        let mine = Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8);
        let expected = vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]];
        println!("{:?}", mine);
        println!("{:?}", expected);
        assert_eq!(mine.len(), expected.len());
        let mine = BTS::from_iter(mine.into_iter().map(|mut v| {
            v.sort_unstable();
            v
        }));
        let expected = BTS::from_iter(expected);
        assert!(mine.symmetric_difference(&expected).count() == 0);

        let mine = Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5);
        let expected = vec![vec![1, 2, 2], vec![5]];
        assert_eq!(mine.len(), expected.len());
        let mine = BTS::from_iter(mine.into_iter().map(|mut v| {
            v.sort_unstable();
            v
        }));
        let expected = BTS::from_iter(expected);
        assert!(mine.symmetric_difference(&expected).count() == 0);
    }
}
