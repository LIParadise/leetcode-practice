//! You are given a 0-indexed binary string s and two integers minJump
//! and maxJump.
//! In the beginning, you are standing at index 0, which is equal to '0'.
//! You can move from index i to index j if the following conditions are fulfilled:
//!     i + minJump <= j <= min(i + maxJump, s.length - 1), and
//!     s[j] == '0'.
//! Return true if you can reach index s.length - 1 in s, or false otherwise.
pub struct Solution;

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        // Consider index i.
        //
        // From which indices we could get to i?
        // [(i - max) .. (i - min + 1)].
        // A sliding window appears, i.e. we can modify little data to
        // get useful information!
        //
        // In particular, suppose we know # of ways to get to i,
        // i.e. how many indices in [(i - max) .. (i - min + 1)] is reachable
        // from the origin,
        // what would it be for (i + 1)?
        // Suppose (i - max) is reachable from the origin, we subtract 1.
        // Suppose (i - min + 1) is reachable from the origin, we add 1.
        // This result is exactly what # it is for (i + 1).
        //
        // The beauti of this solution from Lee215 is that it combines DP with
        // sliding window.
        let mut num_of_ways_to_here = 0;
        let mut dp_arr = vec![false; s.len()];
        let min = min_jump as usize;
        let max = max_jump as usize;
        if let Some(ptr) = dp_arr.get_mut(0) {
            *ptr = true;
        };
        (1..s.len()).for_each(|idx| {
            if s == vec!['0', '0'] {
                dbg!(&dp_arr);
            }
            if idx > max && dp_arr[idx - max - 1] {
                num_of_ways_to_here -= 1;
            }
            if idx >= min && dp_arr[idx - min] {
                num_of_ways_to_here += 1;
            }
            dp_arr[idx] = num_of_ways_to_here > 0 && s[idx] == '0';
        });
        dp_arr.last().expect("Expecting non-empty input").clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert!(Solution::can_reach("011010".to_owned(), 2, 3));
        assert!(!Solution::can_reach("01101110".to_owned(), 2, 3));
        assert!(Solution::can_reach("0".to_owned(), 2, 3));
        assert!(!Solution::can_reach("01".to_owned(), 2, 3));
        assert!(Solution::can_reach("00".to_owned(), 1, 1));
        assert!(!Solution::can_reach("00111010".to_owned(), 3, 5));
    }
}
