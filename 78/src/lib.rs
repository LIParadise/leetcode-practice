pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut powerset = Vec::with_capacity(2_usize.pow(nums.len() as u32));
        Self::gen_subset_worker(&Vec::new(), 0, &nums, &mut powerset);
        powerset
    }
    fn gen_subset_worker(part: &[i32], idx: usize, set: &[i32], powerset: &mut Vec<Vec<i32>>) {
        if let Some(&i) = set.get(idx) {
            Self::gen_subset_worker(&part, idx + 1, set, powerset);
            let mut part = part.to_owned();
            part.push(i);
            Self::gen_subset_worker(&part, idx + 1, set, powerset);
        } else {
            powerset.push(part.to_owned());
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
