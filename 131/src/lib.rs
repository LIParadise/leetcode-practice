pub struct Solution;

impl Solution {
    /// Give all partitioning of input
    /// s.t. for each partitioning, each of the partitioned substrings itself
    /// is a palindrome.
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s = s.as_bytes();
        let mut answers = Vec::with_capacity(s.len());
        for subprob_len in 1..=s.len() {
            if subprob_len == 1 {
                answers.push(vec![vec![std::str::from_utf8(&s[0..1])
                    .unwrap()
                    .to_owned()]]);
            } else {
                // Dynamic programming
                //
                // For sub-problem, we have:
                // substr = left + right
                //
                // right => last substr, no further partition required, check if palindrome
                // left => may be further partitioned, check DP table
                //
                // If right is indeed a palindrome,
                // combine them to get some answers for the sub-problem
                // How?
                // DP table specifies different ways to decompose left
                // For each way in DP table, append right to it.
                //
                // Example:
                // substr = "aab"
                // consider (left = "aa") + (right = "b") = substr
                // Via DP, we know left has 2 ways of decomposition:
                // [["a", "a"], ["aa"]]
                // Append "b" to all the possible ways, after which we get
                // [["a", "a", "b"], ["aa", "b"]]
                //
                // Remember to iterate through all possible "right" and collect them.
                let mut subprob_answers = Vec::with_capacity(answers.last().unwrap().len());
                for last_part_len in (1..=subprob_len).rev() {
                    let last_part = &s[subprob_len - last_part_len..subprob_len];
                    if Self::is_palindrome(last_part) {
                        if last_part_len == subprob_len {
                            subprob_answers
                                .push(vec![std::str::from_utf8(last_part).unwrap().to_owned()])
                        } else {
                            let answers_to_prev_parts = &answers[subprob_len - last_part_len - 1];
                            subprob_answers.extend(
                                answers_to_prev_parts
                                    .iter()
                                    .map(|strings| {
                                        let mut ret = strings.clone();
                                        ret.push(
                                            std::str::from_utf8(last_part).unwrap().to_owned(),
                                        );
                                        ret
                                    })
                                    .collect::<Vec<_>>(),
                            );
                        }
                    }
                }
                answers.push(subprob_answers);
            }
        }

        answers.pop().unwrap_or(Vec::new())
    }
    fn is_palindrome(arr: &[u8]) -> bool {
        arr.iter()
            .zip(arr.iter().rev())
            .take(arr.len() / 2)
            .all(|(a, b)| a == b)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        // Examples originate from here:
        // https://leetcode.com/problems/palindrome-partitioning/description/comments/1724406
        let s = "aaab".to_owned();
        let output = vec![
            vec!["a", "a", "a", "b"],
            vec!["a", "aa", "b"],
            vec!["aa", "a", "b"],
            vec!["aaa", "b"],
        ];
        compare(Solution::partition(s), output);

        let s = "abcaa".to_owned();
        let output = vec![vec!["a", "b", "c", "a", "a"], vec!["a", "b", "c", "aa"]];
        compare(Solution::partition(s), output);

        let s = "abbab".to_owned();
        let output = vec![
            vec!["a", "b", "b", "a", "b"],
            vec!["a", "b", "bab"],
            vec!["a", "bb", "a", "b"],
            vec!["abba", "b"],
        ];
        compare(Solution::partition(s), output);

        let s = "abaca".to_owned();
        let output = vec![
            vec!["a", "b", "a", "c", "a"],
            vec!["a", "b", "aca"],
            vec!["aba", "c", "a"],
        ];
        compare(Solution::partition(s), output);

        let s = "aaa".to_owned();
        let output = vec![
            vec!["a", "a", "a"],
            vec!["a", "aa"],
            vec!["aa", "a"],
            vec!["aaa"],
        ];
        compare(Solution::partition(s), output);
    }

    fn compare(my_answer: Vec<Vec<String>>, ans: Vec<Vec<&str>>) {
        use std::collections::HashSet as Hash;
        let mut my_hash = Hash::new();
        my_hash.extend(
            my_answer
                .iter()
                .map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
        );
        let mut ans_hash = Hash::new();
        ans_hash.extend(ans);
        assert!(ans_hash
            .symmetric_difference(&my_hash)
            .collect::<Vec<_>>()
            .is_empty());
    }
}
