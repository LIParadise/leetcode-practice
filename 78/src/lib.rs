pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut powerset = Vec::with_capacity(2_usize.pow(nums.len() as u32));
        let mut subset = Vec::with_capacity(nums.len());
        Self::gen_subset_worker(&mut subset, 0, &nums, &mut powerset);
        powerset
    }

    // How to generate powerset, i.e. iterate through all subsets?
    // For each element, there's two cases: to take or not to take
    // Ask this to every element till we ran out of elements, then we've got a particular subset.
    // Record it.
    //
    // In essence this is DFS on a binary tree, in which the binary part stems from aforementioned
    // "to take or not to take" question.
    fn gen_subset_worker(
        subset: &mut Vec<i32>,
        idx: usize,
        set: &[i32],
        powerset: &mut Vec<Vec<i32>>,
    ) {
        if let Some(&i) = set.get(idx) {
            Self::gen_subset_worker(subset, idx + 1, set, powerset);
            subset.push(i);
            Self::gen_subset_worker(subset, idx + 1, set, powerset);
            subset.pop();
        } else {
            powerset.push(subset.to_owned());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use std::collections::BTreeSet as BTS;
    #[test]
    fn test_soln() {
        let mine = BTS::from_iter(Solution::subsets(vec![1, 2, 3]));
        let expected = BTS::from_iter(vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ]);
        assert_eq!(mine.symmetric_difference(&expected).count(), 0);

        let mine = BTS::from_iter(Solution::subsets(vec![0]));
        let expected = BTS::from_iter(vec![vec![], vec![0]]);
        assert_eq!(mine.symmetric_difference(&expected).count(), 0);
    }
}
