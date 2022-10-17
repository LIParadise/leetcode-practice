pub struct Solution;

impl Solution {
    #[inline]
    /// Doesn't require this line if rustc >= 1.59, where one could just use
    /// destructuring assignment.
    fn my_swap<T: Clone>(i: &mut T, j: &mut T) {
        let k = i.clone();
        *i = j.clone();
        *j = k;
    }
    pub fn next_permutation(nums: &mut Vec<i32>) {
        for idx in (0..nums.len() - 1).rev() {
            let jdx = idx + 1;
            if nums[idx] < nums[jdx] {
                let (former, latter) = nums.split_at_mut(jdx);
                let min_index_larger_than_idx =
                    latter.partition_point(|n| n > former.last().unwrap());
                Self::my_swap(
                    former.last_mut().unwrap(),
                    latter.get_mut(min_index_larger_than_idx - 1).unwrap(),
                );
                latter.sort_unstable();
                return;
            }
        }
        nums.reverse();
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        let mut input = vec![1, 2, 3];
        let output = vec![1, 3, 2];
        println!("Input is {:?}", input.as_slice());
        Solution::next_permutation(&mut input);
        assert_eq!(output, input);

        let mut input = vec![3, 2, 1];
        let output = vec![1, 2, 3];
        println!("Input is {:?}", input.as_slice());
        Solution::next_permutation(&mut input);
        assert_eq!(output, input);

        let mut input = vec![1, 1, 5];
        let output = vec![1, 5, 1];
        println!("Input is {:?}", input.as_slice());
        Solution::next_permutation(&mut input);
        assert_eq!(output, input);

        let mut input = vec![2, 3, 1];
        let output = vec![3, 1, 2];
        println!("Input is {:?}", input.as_slice());
        Solution::next_permutation(&mut input);
        assert_eq!(output, input);

        let mut input = vec![1, 3, 2];
        let output = vec![2, 1, 3];
        println!("Input is {:?}", input.as_slice());
        Solution::next_permutation(&mut input);
        assert_eq!(output, input);
    }
}
