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

    /// What's next, in terms of dictionary order, permutation of a sequence?
    /// Wraps around.
    // Since dictionary order and *next*, the more left a member is swapped,
    // the *larger* the result would be.
    // E.g. [1, 2, 3]
    // If first were to be changed, making the result [2, ?, ?] or [3, ?, ?]
    // This is *far* from the original string in terms of dictionary order.
    //
    // That is, if possible, focus on least significant bits, i.e. end of
    // the sequence.
    //
    // If a reversed view gives an increasing sequence, then the original
    // sequence is *max*, meaning we should return *min*; just reverse it!
    //
    // If a reversed view is not an increasing sequence, there's a *drop*
    // somewhere.
    // E.g. [1, 3, 4, 6, 5, 2]
    //             ^  ^ --> Here's a drop!
    // To make a next permutation, notice the latter part, by our search
    // criteria, using reversed view is a increasing sequence.
    // I.e. in some sense this subsequence already attains maximum.
    // It's like carry bit in addition!
    // Hence, what we need to do is find a suitable element to do *carry*,
    // i.e. the first element that's greater than 4.
    /// ```
    /// use lc_31_next_permutation::Solution as S;
    /// let mut input = vec![1, 3, 4, 6, 5, 2];
    /// let output = vec![1, 3, 5, 2, 4, 6];
    /// S::next_permutation(&mut input);
    /// assert_eq!(input, output);
    /// ```
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
