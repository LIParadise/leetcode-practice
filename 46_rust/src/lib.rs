pub struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut cache = vec![];
        let mut idx_in_cache = vec![false; nums.len()];
        let mut ret = vec![];
        Self::permute_worker(&nums, &mut cache, &mut idx_in_cache, &mut ret);
        ret
    }
    fn permute_worker(
        nums: &[i32],
        cache: &mut Vec<i32>,
        idx_in_cache: &mut [bool],
        ret: &mut Vec<Vec<i32>>,
    ) {
        if cache.len() == nums.len() {
            ret.push(cache.clone());
            return;
        } else {
            nums.iter().enumerate().for_each(|(idx, num)| {
                if !idx_in_cache[idx] {
                    idx_in_cache[idx] = true;
                    cache.push(*num);
                    Self::permute_worker(nums, cache, idx_in_cache, ret);
                    idx_in_cache[idx] = false;
                    cache.pop();
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use std::collections::HashSet;
    #[test]
    fn test_soln() {
        let mine = Solution::permute(vec![1]);
        let ans = HashSet::<Vec<_>>::from_iter([vec![1]]);
        assert_eq!(mine.len(), ans.len());
        assert!(
            ans.symmetric_difference(&HashSet::from_iter(mine))
                .next()
                .is_none()
        );

        let mine = Solution::permute(vec![42, 69]);
        let ans = HashSet::<Vec<_>>::from_iter([vec![42, 69], vec![69, 42]]);
        assert_eq!(mine.len(), ans.len());
        assert!(
            ans.symmetric_difference(&HashSet::from_iter(mine))
                .next()
                .is_none()
        );

        let mine = Solution::permute(vec![42, 69, 114514]);
        let ans = HashSet::<Vec<_>>::from_iter([
            vec![42, 69, 114514],
            vec![42, 114514, 69],
            vec![69, 42, 114514],
            vec![69, 114514, 42],
            vec![114514, 42, 69],
            vec![114514, 69, 42],
        ]);
        assert_eq!(mine.len(), ans.len());
        assert!(
            ans.symmetric_difference(&HashSet::from_iter(mine))
                .next()
                .is_none()
        );
    }
}
