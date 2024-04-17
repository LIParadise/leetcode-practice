pub struct Solution;

impl Solution {
    /// Given an integer array nums that may contain duplicates, return all possible
    /// subsets
    /// (the power set).
    /// The solution set must not contain duplicate subsets. Return the solution in any order.
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut ret = Vec::new();
        Self::subsets_with_dup_worker(&nums, &mut ret, &mut Vec::new());
        ret
    }

    // issac3's genius algorithm, combining ideas of dynamic programming, backtrack, DFS, and
    // iterations.
    // https://leetcode.com/problems/permutations/solutions/18239/a-general-approach-to-backtracking-questions-in-java-subsets-permutations-combination-sum-palindrome-partioning
    //
    // For each element, there's the boolean question: either to take or not to take,
    // which is why the DFS tree is binary: recursive call this function to DFS backtrack.
    fn subsets_with_dup_worker(nums: &[i32], powerset: &mut Vec<Vec<i32>>, subset: &mut Vec<i32>) {
        // We're thinking of subsets, hence we could do early-stop here:
        // The subset here is subset of previously considered element, meaning it's subset of the
        // whole input, which in turn means we could early-stop and record it here.
        // This way saves us some calculations, e.g. we don't have to scan whole array only to
        // conclude that the empty set is part of the powerset.
        powerset.push(subset.to_owned());
        if nums.is_empty() {
            return;
        }
        nums.iter().enumerate().for_each(|(i, &n)| {
            // At recursion depth d, choose the element to be in some subset in which the
            // element resides at index (d-1).
            // In principle every element is possible, hence iterate through them.
            if i > 0 && nums[i] == nums[i - 1] {
                // To avert duplicates, amongst the duplicates of some subset, choose the one in
                // which every element appeared earliest in the input.
                // E.g. set be [a, b, b], imagine indexing s.t. it's secretly [a0, b1, b2],
                // consider subset [a, b]:
                // choose [a0, b1] rather than [a0, b2].
                //
                // How?
                // First, SORT the array s.t. equal elements are neighbors.
                // Second, suppose there are multiple instances of same element e,
                // if we skipped e_i, then we shall also skip all e_j where j > i,
                // this way we eliminate the possibility where we choose some subset in which e_i
                // is absent whereas e_j is present.
                //
                // If the condition holds, we're excluding the possibility that the repeating
                // element were to be at the current index (recursion depth d i.e. index (d-1))
                // By aforementioned observation namely if e_i were skipped then all e_j where
                // j > i shall also be skipped, we do as required.
                return;
            }
            subset.push(n);
            Self::subsets_with_dup_worker(&nums[i + 1..], powerset, subset);
            subset.pop();
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        compare(
            Solution::subsets_with_dup(vec![1, 2, 2]),
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
                vec![2],
                vec![2, 2],
            ],
        );
        compare(Solution::subsets_with_dup(vec![0]), vec![vec![], vec![0]]);
    }

    fn compare(my_answer: Vec<Vec<i32>>, ans: Vec<Vec<i32>>) {
        use std::collections::HashSet;
        if my_answer.len() != ans.len() {
            println!("{:?}", my_answer);
            println!("{:?}", ans);
            assert!(false);
        }
        let my_hash = HashSet::<_>::from_iter(my_answer);
        let ans_hash = HashSet::from_iter(ans);
        if ans_hash.symmetric_difference(&my_hash).count() != 0 {
            println!("{:?}", my_hash);
            println!("{:?}", ans_hash);
            assert!(false);
        }
    }
}
