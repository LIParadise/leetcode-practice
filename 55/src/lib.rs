pub struct Solution;

/// You are given an integer array nums. You are initially positioned at the
/// array's first index, and each element in the array represents your maximum
/// jump length at that position.
/// Return true if you can reach the last index, or false otherwise.
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        /*
         * What happens if we have one 0 and need to jump?
         * The immediate last index should have a value *larger* than 1 or else
         * we can't jump over this segment of 0.
         * It's okay if we aren't going to jump from immediate last index, but
         * the jump distance would correspondingly grow.
         * Similarly, if instead of one 0 we have consecutive 0 the length to
         * leap over also increases.
         */
        let mut island_of_zeros = 0;
        for i in nums.into_iter().rev().skip(1) {
            // Notice goal is just `nums.back()`, hence the `.skip(1)`
            match (i, island_of_zeros) {
                (0, 0) => island_of_zeros = 2,
                (0, _) => island_of_zeros += 1,
                (_, 0) => {}
                (_, _) => {
                    if i >= island_of_zeros {
                        island_of_zeros = 0
                    } else {
                        island_of_zeros += 1
                    }
                }
            }
        }
        island_of_zeros == 0
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(true, Solution::can_jump(vec![2, 3, 1, 1, 4]));
        assert_eq!(false, Solution::can_jump(vec![3, 2, 1, 0, 4]));
    }
}
