impl Solution {
    /// Dynamic Programming
    ///
    /// Supp. s[i..j] is palindrome, a way to extend it is to check if
    /// s[i-1] == s[j], if so it's a longer palindrome s[i-1..=j].
    ///
    /// For example, consider "abbc", len is 4.
    /// Conceptually we'd have table 5x5 called DP
    /// s.t. DP[i, j] denotes if s[i..j] is palindrome,
    /// where (i == j) then DP[i, j] is true (length 0 palindrome)
    /// and (i == j-1) is also true (length 1, single char palindrome)
    /// e.g. DP[2, 2] == true.
    /// This is for s.t. can consider "growing" the length-0 palindrome
    /// from "" to "bb".
    ///
    /// The table could be filled diagonally, i.e. consider j-i from
    /// `0` to `s.len()`.
    /// Notice for each diagonal "row", the value is solely dependent on
    /// the second last filled "row", hence the table could be implemented
    /// in linear memory.
    /// E.g. to check if DP[1, 5] is true, i.e. if s[1..=4] is a palindrome
    /// with length 4, just ask if DP[2..=3] is true AND s[1] == s[4].
    ///
    /// Since the decision is based on second last row, i.e. effectively
    /// we consider even-length palindromes and odd-length palindromes
    /// separately, the dynamic programming table could be implemented with
    /// just two bool arrays.
    pub fn longest_palindrome(s: String) -> String {
        /*
         * Exclude simple cases s.t. later assume non-trivial len
         */
        if s.len() <= 1 {
            return s;
        }

        // The table, stored in linear memory
        // Flags for if len 2 (`&s[0..2]`, `&s[1..3]`, ...),
        // len 4 (`&s[0..4]`, `&s[1..5]`, ...),
        // len 6, etc are palindrome
        let mut even_len_pal_flags = vec![true; s.len() + 1];
        // Flags for if
        // len 1 (`&s[0..1]`, `&s[1..2]`, ...),
        // len 3 (`&s[0..3]`, `&s[1..4]`, ...),
        // len 5, etc are palindrome
        let mut odd_len_pal_flags = vec![true; s.len()];
        // The storage here is kinda perplexed, in order to prevent unnecessary
        // memory move.
        // See the following for-loop.
        let mut longest_pal_len = 1;
        let mut longest_pal_flag_idx = 0;

        /*
         * Diagonally fill the table, i.e. consider palindrome from length
         * 0 to length n
         */
        for new_pal_len in 2..=s.len() {
            // Perplexed indexing here; see the iterator below.
            let dp_cache_flags = if new_pal_len % 2 == 0 {
                &mut even_len_pal_flags
            } else {
                &mut odd_len_pal_flags
            };
            for ((pal_flag_idx, is_pal), (char_before, char_after)) in dp_cache_flags
                .iter_mut()
                .enumerate()
                .skip(new_pal_len / 2)
                /*
                 * Why `.skip()`? Perplexed indexing here!
                 * Notice methods `.enumerate()` and `.skip()` does NOT commute
                 *
                 * To check if `&s[0..2]` is palindrome, ask if `&s[1..1]`
                 * is palindrome (which happens to be; 0-length ""),
                 * rather than ask if `&s[0..0]` is palindrome.
                 * (After which we check if `s[0] == s[1]`)
                 * That is, to check if `&s[0..2]` is palindrome, we check the
                 * cached result at `even_len_pal_flags[1]`, AND store the
                 * result back to `even_len_pal_flags[1]`, so on and so forth.
                 * In other words, offset to read/write DP cache is __1__.
                 *
                 * To check if `&s[0..4]` is palindrome, ask if `&s[1..3]` is.
                 * Last time we stored the result if `&s[0..2]` is palindrome
                 * at `even_len_pal_flags[1]`, similarly `&s[1..3]`
                 * at `even_len_pal_flags[2]`, hence we should R/W the DP table
                 * with offset __2__ instead of 1 this time.
                 * (After which we check if `s[0] == s[3]`)
                 *
                 * As we consider longer palindromes, the R/W offset should be
                 * correspondingly adjusted.
                 */
                .zip(s.chars().zip(s.chars().skip(new_pal_len - 1)))
            {
                let new_longer_pal = *is_pal && (char_before == char_after);
                if new_longer_pal {
                    longest_pal_len = new_pal_len;
                    longest_pal_flag_idx = pal_flag_idx;
                }
                // Store DP result back in cache
                *is_pal = new_longer_pal;
            }
        }
        if longest_pal_len <= 1 {
            return String::from(&s[0..1]);
        } else {
            return String::from(
                &s[longest_pal_flag_idx - longest_pal_len / 2
                    ..longest_pal_flag_idx + longest_pal_len - longest_pal_len / 2],
            );
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_solution() {
        assert_eq!(
            "bb",
            Solution::longest_palindrome("abbc".to_string()).as_str()
        );
        assert_eq!(
            "bab",
            Solution::longest_palindrome("babad".to_string()).as_str()
        );
        assert_eq!(
            "bb",
            Solution::longest_palindrome("cbbd".to_string()).as_str()
        );
    }
}
