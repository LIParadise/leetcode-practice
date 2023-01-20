pub struct Solution;

impl Solution {
    /// Given a string s and a character c that occurs in s, return an array
    /// of integers answer where answer.length == s.length and answer[i] is
    /// the distance from index i to the closest occurrence of character c in s.
    ///
    /// The distance between two indices i and j is abs(i - j), where abs is
    /// the absolute value function.
    ///
    /// It is guaranteed that c occurs at least once in s.
    /// s[i] and c are lowercase English letters.
    ///
    /// Examples:
    /// ```
    /// use lc_821_shortest_distance_to_a_character::Solution as soln;
    /// let s = "loveleetcode";
    /// let c = 'e';
    /// assert_eq!(soln::shortest_to_char(s.to_string(), c).as_ref(),
    ///     [3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]);
    /// let s = "aaab";
    /// let c = 'b';
    /// assert_eq!(soln::shortest_to_char(s.to_string(), c).as_ref(),
    ///     [3, 2, 1, 0]);
    /// ```
    // Idea: wave reflection: the target char works like walls.
    // Initialize the array with max possible,
    // work through the string till hit,
    // then from that index relax the previous and following entries.
    // Complexity is linear since every entry got inspected/changed
    // at most twice.
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut ret = {
            let mut tmp = Vec::with_capacity(s.len());
            for _ in 0..s.len() {
                tmp.push(i32::MAX);
            }
            tmp
        };
        s.chars()
            .enumerate()
            .filter(|(_, c_in_s)| c_in_s == &c)
            .for_each(|(idx, _)| {
                ret[idx] = 0;
                ((idx + 1)..ret.len())
                    .try_for_each(|i| {
                        if ret[i] > (i - idx) as i32 {
                            ret[i] = (i - idx) as i32;
                            Ok(())
                        } else {
                            Err(())
                        }
                    })
                    .ok();
                (0..idx)
                    .rev()
                    .try_for_each(|i| {
                        if ret[i] > (idx - i) as i32 {
                            ret[i] = (idx - i) as i32;
                            Ok(())
                        } else {
                            Err(())
                        }
                    })
                    .ok();
            });
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        todo!()
    }
}
