pub struct Solution;

impl Solution {
    pub fn subarray_sum(mut nums: Vec<i32>, k: i32) -> i32 {
        Self::inplace_into_prefix_sum(&mut nums);
        nums.iter()
            .enumerate()
            .rev()
            .fold(0, |ret, (idx, &partial_sum_longer)| {
                ret + <usize as TryInto<i32>>::try_into(
                    std::iter::once(&0)
                        .chain(nums.iter().take(idx))
                        .filter(|&&partial_sum_shorter| {
                            partial_sum_longer - partial_sum_shorter == k
                        })
                        .count(),
                )
                .unwrap()
            })
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
