pub struct Solution;

impl Solution {
    /// Given an array nums of distinct integers, return all the possible
    /// permutations.
    /// You can return the answer in any order.
    ///
    /// 1 <= nums.length <= 6
    /// -10 <= nums[i] <= 10
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let factorial = (1..=nums.len()).product::<usize>();
        let mut ret = vec![Vec::with_capacity(nums.len()); factorial];
        let mut smaller_factorial = factorial;
        // Idea:
        // consider [1, 2, 3]
        // 1        2   1            3 2   1
        // 1        2   1              2 3 1
        // 1   ->   2   1       ->     2   1 3
        // 1            1   2            3 1   2
        // 1            1   2              1 3 2
        // 1            1   2              1   2 3
        (0..nums.len()).for_each(|idx| {
            let mut insert_at = 0;
            smaller_factorial /= idx + 1;
            (0..factorial / smaller_factorial).for_each(|i| {
                (0..smaller_factorial).for_each(|j| {
                    ret[smaller_factorial * i + j].insert(insert_at, nums[idx]);
                });
                insert_at += 1;
                if insert_at > idx {
                    insert_at = 0;
                }
            });
        });
        ret
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
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
        let output = Solution::permute(vec![5, 7, 6, 0]);
        let ans = vec![
            vec![5, 7, 6, 0],
            vec![5, 7, 0, 6],
            vec![5, 6, 7, 0],
            vec![5, 6, 0, 7],
            vec![5, 0, 6, 7],
            vec![5, 0, 7, 6],
            vec![7, 6, 0, 5],
            vec![7, 0, 6, 5],
            vec![6, 7, 0, 5],
            vec![6, 0, 7, 5],
            vec![0, 6, 7, 5],
            vec![0, 7, 6, 5],
            vec![7, 5, 6, 0],
            vec![7, 5, 0, 6],
            vec![6, 5, 7, 0],
            vec![6, 5, 0, 7],
            vec![0, 5, 6, 7],
            vec![0, 5, 7, 6],
            vec![6, 0, 5, 7],
            vec![0, 6, 5, 7],
            vec![7, 0, 5, 6],
            vec![0, 7, 5, 6],
            vec![6, 7, 5, 0],
            vec![7, 6, 5, 0],
        ];
        test_soln_worker(ans, output);
    }

    fn test_soln_worker<T: Eq + Hash + Debug + Ord>(a: Vec<T>, b: Vec<T>) {
        assert!(a.len() == b.len());
        let a_set = a.into_iter().collect::<BTreeSet<T>>();
        let b_set = b.into_iter().collect::<BTreeSet<T>>();
        assert_eq!(a_set.symmetric_difference(&b_set).count(), 0);
    }
}
