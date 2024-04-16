pub struct Solution;

impl Solution {
    /// Give all partitioning of input
    /// s.t. for each partitioning, each of the partitioned substrings itself
    /// is a palindrome.
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s = s.as_bytes();
        let mut ret = Vec::new();
        let mut backtrack = Vec::new();
        Self::partition_worker(s, &mut backtrack, 0, &mut ret);
        ret
    }
    fn partition_worker(
        s: &[u8],
        backtrack: &mut Vec<Vec<u8>>,
        start_idx: usize,
        solutions: &mut Vec<Vec<String>>,
    ) {
        if start_idx == s.len() {
            solutions.push(
                backtrack
                    .iter()
                    .cloned()
                    .map(|u8_arr| String::from_utf8(u8_arr).unwrap())
                    .collect(),
            );
        } else if start_idx < s.len() {
            (start_idx + 1..=s.len()).for_each(|end_idx| {
                let new_palindrome = &s[start_idx..end_idx];
                if Self::is_palindrome(new_palindrome) {
                    backtrack.push(new_palindrome.to_owned());
                    Self::partition_worker(s, backtrack, end_idx, solutions);
                    backtrack.pop();
                }
            })
        }
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
        assert_eq!(my_answer.len(), ans.len());
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
