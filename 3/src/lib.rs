pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        Self::length_of_longest_substring_worker(&s)
            .try_into()
            .unwrap()
    }
    fn length_of_longest_substring_worker(s: &str) -> usize {
        // Idea: double pointers
        //
        // Brute force -> O(n^2), for all (i, j) where 0 <= i < j < (string len)
        // the trick is to note that if (i, j) is ok but (i, j+1) not ok,
        // then all (i, j+k) where k >= 2 are not ok, either,
        // thus we consider (i+1, j+1) next, thus overall time may be reduced to linear.
        //
        // Note that input character set is ASCII, so hash may simply be implemented with an array.
        let mut hash = [false; 256];
        let mut i = 0;
        let mut j = 0;
        let s = s.as_bytes();
        let mut ret = 0;

        // loop invariant:
        // 1. Indices: 0 <= i <= j < length
        // 2. Consider `&s[i..=j]` in each loop
        loop {
            while j < s.len() {
                if std::mem::replace(&mut hash[s[j] as usize], true) {
                    break;
                }
                j += 1;
            }

            ret = std::cmp::max(ret, j - i);
            if j == s.len() {
                break ret;
            }

            while i < j {
                hash[s[i] as usize] = false;
                i += 1;
                if s[i - 1] == s[j] {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        );
        assert_eq!(Solution::length_of_longest_substring(String::from("a")), 1);
        assert_eq!(Solution::length_of_longest_substring(String::from("aa")), 1);
        assert_eq!(Solution::length_of_longest_substring(String::from("ab")), 2);
        assert_eq!(
            Solution::length_of_longest_substring(String::from("aaa")),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("aab")),
            2
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abb")),
            2
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        );
    }
}

macro_rules! lprintln {
    // Match when a format string and additional arguments are provided
    ($fmt:expr, $($arg:tt)*) => {{
        if cfg!(feature = "local_test") {
            println!($fmt, $($arg)*);
        }
    }};

    // Match when only a format string is provided
    ($fmt:expr) => {{
        if cfg!(feature = "local_test") {
            println!($fmt);
        }
    }};
}

