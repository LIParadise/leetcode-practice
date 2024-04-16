pub struct Solution;

impl Solution {
    /// Given an integer array nums that may contain duplicates, return all possible
    /// subsets
    /// (the power set).
    /// The solution set must not contain duplicate subsets. Return the solution in any order.
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut ret = Vec::new();
        Self::subsets_with_dup_worker(&nums, &mut ret, &mut Vec::new(), None, 0);
        ret
    }

    // DFS backtrack binary tree
    // For each element, there's a boolean question: either to take or not to take,
    // which is why the tree is binary; recursive call this function to DFS backtrack.
    //
    // To avert duplicates, first sort the array, then upon decision of whether to put element
    // in subset, if the answer is no, tell the immediate deeper recursion to also skip that
    // member.
    //
    // This works since the array is sorted, s.t. equal elements are neighbors, and that it forces
    // all the subsets in the powerset to have the following property:
    // if one were to index each element in the sorted input array, then among all the duplicate
    // subsets, the algorithm picks only the one in which sum of index is smallest.
    //
    // In layman's terms, among all duplicate subsets, the algorithm chooses the one appears
    // "earliest" in the input array.
    fn subsets_with_dup_worker(
        nums: &[i32],
        powerset: &mut Vec<Vec<i32>>,
        subset: &mut Vec<i32>,
        ignore: Option<i32>,
        start_idx: usize,
    ) {
        if start_idx == nums.len() {
            powerset.push(subset.to_owned());
        } else if let Some(&n) = nums.get(start_idx) {
            Self::subsets_with_dup_worker(nums, powerset, subset, Some(n), start_idx + 1);
            if let Some(i) = ignore {
                if i == n {
                    return;
                }
            }
            subset.push(n);
            Self::subsets_with_dup_worker(nums, powerset, subset, None, start_idx + 1);
            subset.pop();
        }
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
