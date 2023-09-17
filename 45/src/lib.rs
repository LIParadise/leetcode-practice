//! You are given a 0-indexed array of integers nums of length n.
//! You are initially positioned at nums[0].
//! Each element nums[i] represents the maximum length of a forward
//! jump from index i. In other words, if you are at nums[i],
//! you can jump to any nums[i + j] where:
//! 0 <= j <= nums[i] and
//! i + j < n
//! Return the minimum number of jumps to reach nums[n - 1].
//! The test cases are generated such that you can reach nums[n - 1].
pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        // Relaxation.
        // Start with whole array, mark all as unreachable
        // Then for each index relax the estimations.
        //
        // Loop Invariant:
        // For every step, after loop,
        // every reachable is at its minimal cost
        // Proof:
        // By construction.
        let mut distances: Vec<Option<usize>> = vec![None; nums.len()];
        (0..nums.len()).for_each(|idx| {
            let (known, to_mod) = distances.split_at_mut(idx + 1);
            let dist = match known.last().unwrap() {
                None => {
                    *known.last_mut().unwrap() = Some(0);
                    0
                }
                Some(i) => i.clone(),
            } + 1;
            to_mod
                .iter_mut()
                .take(nums[idx] as usize)
                .for_each(|t| match t {
                    None => *t = Some(dist),
                    Some(t) if *t > dist => *t = dist,
                    _ => {}
                });
        });
        distances.last().map_or(0, |dist| dist.unwrap() as i32)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }
}
