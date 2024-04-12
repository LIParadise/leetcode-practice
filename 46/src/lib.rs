pub struct Solution;

impl Solution {
    /// Given an array nums of distinct integers, return all the possible
    /// permutations.
    /// You can return the answer in any order.
    ///
    /// 1 <= nums.length <= 6
    /// -10 <= nums[i] <= 10
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut permutation = Vec::with_capacity(nums.len());
        let mut ret = Vec::with_capacity((1..=nums.len()).product());
        Self::permute_worker(&mut permutation, &nums, &mut ret);
        ret
    }
    // Idea: for every element, try put them at every index
    //
    // Consider [a, b, c]
    //
    // Upon first call, recursive call self, s.t.
    // call depth          : 1    2       3
    // permutation content : [a]  [a, b]  [a, b, c]
    // and we push [a, b, c] to the collection
    //
    // Next, call depth 3 pops and ends the for-loop, back to call depth 2
    // call depth, while having [a, b], further pops, resulting in [a]
    // the for-loop of call depth 2 hasn't ended yet, this time considering c,
    // yielding [a, c], then dive to call depth 3, this time giving [a, c, b]. 
    //
    // So on and so forth, at each call depth d,
    // the function chooses one element to be at index (d-1) to make a new permutation,
    // and it does so for each element, s.t. all permutations are visited.
    //
    // Pros: consumes little space (O(n)), probably cache-friendly
    // Cons: abyssmal time complexity (W(n!*n^2)), due to linear search in array
    // Possible mitigation: add hash, double the space while time complexity becomes O(n!)
    fn permute_worker(permutation: &mut Vec<i32>, nums: &[i32], permutations: &mut Vec<Vec<i32>>) {
        if permutation.len() == nums.len() {
            permutations.push(permutation.clone());
        } else {
            nums.iter().for_each(|n| {
                if !permutation.contains(n) {
                    permutation.push(*n);
                    Self::permute_worker(permutation, nums, permutations);
                    permutation.pop();
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::fmt::Debug;
    use std::hash::Hash;

    use crate::Solution;
    #[test]
    fn test_soln() {
        let output = Solution::permute(vec![1, 2, 3]);
        let ans = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        test_soln_worker(ans, output);
        let output = Solution::permute(vec![4, 2]);
        let ans = vec![vec![4, 2], vec![2, 4]];
        test_soln_worker(ans, output);
    }

    fn test_soln_worker<T: Eq + Hash + Debug>(a: Vec<T>, b: Vec<T>) {
        assert!(a.len() == b.len());
        let a_set = a.into_iter().collect::<HashSet<T>>();
        let b_set = b.into_iter().collect::<HashSet<T>>();
        assert!(a_set.iter().all(|a| b_set.contains(a) == true));
    }
}
