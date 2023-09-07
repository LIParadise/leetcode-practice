pub struct Solution;

impl Solution {
    /// Given a string s, return the number of palindromic substrings in it.
    /// A string is a palindrome when it reads the same backward as forward.
    /// A substring is a contiguous sequence of characters within the string.
    pub fn count_substrings(s: String) -> i32 {
        // [h, e, l, l, o]
        //  ^^ ^^ ^^ ^^ ^
        // For all these '^' positions, i.e. all the indices and all the in-between
        // places, consider if there's palindrome by extending string slice to
        // both side.
        // (len is 5, while there's 9 such positions)
        // If different, abort.
        // If same, add one to count, and continue.
        let count = (0..2 * s.len()).fold(0, |count, pseudo_idx| {
            if pseudo_idx % 2 == 0 {
                let idx = pseudo_idx / 2;
                count
                    + s.chars()
                        .skip(idx + 1)
                        .zip(s.chars().rev().skip(s.len() - idx))
                        .take_while(|(c0, c1)| c0 == c1)
                        .count()
                    + 1
            } else {
                let jdx = (pseudo_idx + 1) / 2;
                count
                    + s.chars()
                        .skip(jdx)
                        .zip(s.chars().rev().skip(s.len() - jdx))
                        .take_while(|(c0, c1)| c0 == c1)
                        .count()
            }
        });
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        let s = String::from("Hello");
        assert_eq!(Solution::count_substrings(s), 6);
        let s = String::from("aaa");
        assert_eq!(Solution::count_substrings(s), 6);
        let s = String::from("abc");
        assert_eq!(Solution::count_substrings(s), 3);
    }
}
