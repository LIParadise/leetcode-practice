pub struct Solution;

impl Solution {
    pub fn subarray_sum(mut nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::BTreeMap;
        Self::inplace_into_prefix_sum(&mut nums);
        nums.iter()
            .fold(
                (0, BTreeMap::new()),
                |(ret, mut partial_sum_occurred_times), &partial_sum| {
                    let &cnt_of_subarray_matches = partial_sum_occurred_times
                        .get(&(partial_sum - k))
                        .unwrap_or(&0);
                    partial_sum_occurred_times
                        .entry(partial_sum)
                        .and_modify(|times| *times += 1)
                        .or_insert(1);
                    (
                        cnt_of_subarray_matches + ret + if partial_sum == k { 1 } else { 0 },
                        partial_sum_occurred_times,
                    )
                },
            )
            .0
    }
    fn inplace_into_prefix_sum(nums: &mut [i32]) {
        nums.iter_mut()
            .scan(0, |partial_sum, x| {
                let old_partial_sum = *partial_sum;
                *partial_sum += *x;
                Some((old_partial_sum, x))
            })
            .for_each(|(sum_of_prev, x)| {
                *x += sum_of_prev;
            });
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_inplace_into_prefix_sum() {
        let mut v = vec![1, 3, 5, 7, 9];
        Solution::inplace_into_prefix_sum(&mut v);
        assert_eq!(v.as_slice(), [1, 4, 9, 16, 25].as_slice());
    }

    #[test]
    fn test_soln() {
        assert_eq!(Solution::subarray_sum(vec! {1,1,1}, 2), 2);
        assert_eq!(Solution::subarray_sum(vec! {1,2,3}, 3), 2);
    }
}
