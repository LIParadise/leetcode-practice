pub struct Solution;

impl Solution {
    /// Given an integer array nums, find the subarray with the largest sum,
    /// and return its sum.
    ///
    /// Example:
    /// ```
    /// use lc_53_maximum_subarray::Solution as S;
    /// assert_eq!(S::max_sub_array(Vec::from(
    ///     [-2, 1, -3, 4, -1, 2, 1, -5, 4])), 6);
    /// assert_eq!(S::max_sub_array(Vec::from(
    ///     [1])), 1);
    /// assert_eq!(S::max_sub_array(Vec::from(
    ///     [5, 4, -1, 7, 8])), 23);
    /// ```
    // Idea:
    // Auxiliary array on which each index records the maximum possible sum
    // amongst those end on that index.
    //
    // Observation:
    // We don't REALLY need an axuiliary array: IN-PLACE manipulation is
    // POSSIBLE, since we don't care about previous entries in the axuiliary
    // array.
    // See also Kadane's Algorithm.
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // This snippet consumes whole array, erasing much information.
        // In-place manipulation preserves integrity of array.
        /*
            let mut nums = nums;
            (0..nums.len()).for_each(|idx| {
                let (prev, after) = nums.split_at_mut(idx);
                after[0] += match prev.last() {
                    None => 0,
                    Some(&p) if p <= 0 => 0,
                    Some(&p) if p > 0 => p,
                    _ => panic!("Contradiction: NaN integer"),
                };
            });
            nums.iter().max().map_or(0, |&max| max)
        */
        Self::max_sub_array_worker(&nums)
    }
    fn max_sub_array_worker(nums: &Vec<i32>) -> i32 {
        let mut best_sum = nums.first().map_or(0, |&i| i);
        let mut cur_sum = 0;
        nums.iter().for_each(|n| {
            cur_sum += n;
            if cur_sum > best_sum {
                best_sum = cur_sum;
            }
            if cur_sum < 0 {
                cur_sum = 0;
            }
        });
        best_sum
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        todo!()
    }
}
