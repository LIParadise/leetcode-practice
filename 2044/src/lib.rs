pub struct Solution;

impl Solution {
    // Brute force
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let target = nums.iter().fold(0, |target, n| target | n);
        let mut ret = 0;
        (0..2_u32.pow(nums.len() as u32)).for_each(|mut bitmask| {
            if target
                == nums
                    .iter()
                    .filter(|_| {
                        let in_subset = bitmask & 1 == 1;
                        bitmask >>= 1;
                        in_subset
                    })
                    .fold(0, |result, n| result | n)
            {
                ret += 1;
            }
        });
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::count_max_or_subsets(vec![3, 1]), 2);
        assert_eq!(Solution::count_max_or_subsets(vec![2, 2, 2]), 7);
        assert_eq!(Solution::count_max_or_subsets(vec![3, 2, 1, 5]), 6);
    }
}
