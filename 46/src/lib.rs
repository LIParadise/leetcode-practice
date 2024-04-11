pub struct Solution;

impl Solution {
    /// Given an array nums of distinct integers, return all the possible
    /// permutations.
    /// You can return the answer in any order.
    ///
    /// 1 <= nums.length <= 6
    /// -10 <= nums[i] <= 10
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        /*
         * Consider doing permutations for "abcde"
         *
         * 1 => a
         * 2 => ab, ba
         * 3 => consider where to put 'c';
         *      3 locations for "ab"
         *      3 locations for "ba"
         *      They are all different, and they together are all.
         * 4 => consider where to put 'd', 4*6 = 24
         * So on and so forth.
         *
         * I.e. build the permutations by parts, consider characters one by one
         * For every new character, consider all available indices it could be, insert it to that
         * index
         * Do such to all the previously built permutation of substrings.
         */
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let mut workspace = Vec::new();
        for n in nums {
            if ret.is_empty() {
                workspace.push(vec![n]);
            } else {
                workspace.clear();
                for permutation_of_substring in &ret {
                    for idx in 0..=permutation_of_substring.len() {
                        let mut copied = permutation_of_substring.clone();
                        copied.insert(idx, n);
                        workspace.push(copied);
                    }
                }
            }
            ret.clear();
            ret.extend(workspace.iter().cloned());
        }
        ret
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
