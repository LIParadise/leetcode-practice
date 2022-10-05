pub struct Solution;

impl Solution {
    /// Given an array nums of distinct integers, return all the possible
    /// permutations.
    /// You can return the answer in any order.
    ///
    /// 1 <= nums.length <= 6
    /// -10 <= nums[i] <= 10
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::BTreeSet as Set;
        /*
         * 1 => a
         * 2 => ab, ba
         * 3 => consider where to put 'c';
         *      3 locations for "ab"
         *      3 locations for "ba"
         *      They are all different, and they together are all.
         * 4 => consider where to put 'd', 4*6 = 24
         * So on and so forth.
         */
        let mut ret: Set<Vec<i32>> = Set::new();
        let mut mem = Vec::new();
        for n in nums {
            if ret.is_empty() {
                mem.push(vec![n]);
            } else {
                mem.clear();
                for old in &ret {
                    for idx in 0..=old.len() {
                        let mut old_copy = old.clone();
                        old_copy.insert(idx, n.clone());
                        mem.push(old_copy);
                    }
                }
            }
            ret.clear();
            mem.iter().for_each(|arr| {
                ret.insert(arr.clone());
            });
        }
        ret.into_iter().collect()
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

    fn test_soln_worker<T: Eq + Hash + Debug>(a: Vec<T>, b: Vec<T>) {
        assert!(a.len() == b.len());
        let a_set = a.into_iter().collect::<HashSet<T>>();
        let b_set = b.into_iter().collect::<HashSet<T>>();
        assert!(a_set.iter().all(|a| b_set.contains(a) == true));
    }
}
