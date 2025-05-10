pub struct Solution;

impl Solution {
    /// Give all partitioning of input
    /// s.t. for each partitioning, each of the partitioned substrings itself
    /// is a palindrome.
    pub fn partition(s: String) -> Vec<Vec<String>> {
        Self::collect_palindromes(&s)
    }

    fn collect_palindromes(s: &str) -> Vec<Vec<String>> {
        (1..=s.len()).fold(vec![], |mut accm, end| {
            if let Some((palindrome, remained)) = Self::palindrome_token(s, end) {
                if remained.is_empty() {
                    accm.push(vec![String::from(palindrome)])
                } else {
                    accm.extend(
                        Self::collect_palindromes(remained)
                            .into_iter()
                            .map(|mut v| {
                                let mut ret = vec![String::from(palindrome)];
                                ret.append(&mut v);
                                ret
                            }),
                    )
                }
            }
            accm
        })
    }

    fn palindrome_token(s: &str, u: usize) -> Option<(&str, &str)> {
        s.split_at_checked(u).and_then(|(token, remained)| {
            if Self::is_palindrome(token) {
                Some((token, remained))
            } else {
                None
            }
        })
    }

    fn is_palindrome(s: &str) -> bool {
        s.chars()
            .zip(s.chars().rev())
            .take(s.len() / 2)
            .all(|(a, b)| a == b)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_is_palindrome() {
        assert!(Solution::is_palindrome(""));
        assert!(Solution::is_palindrome("x"));
        assert!(Solution::is_palindrome("xx"));
        assert!(!Solution::is_palindrome("ab"));
        assert!(Solution::is_palindrome("xxx"));
        assert!(Solution::is_palindrome("xyx"));
        assert!(!Solution::is_palindrome("abc"));
    }
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
        use std::collections::HashSet;
        assert_eq!(my_answer.len(), ans.len());
        let my_hash: HashSet<Vec<&str>> = HashSet::from_iter(
            my_answer
                .iter()
                .map(|v| v.iter().map(String::as_str).collect::<Vec<_>>()),
        );
        let ans_hash: HashSet<Vec<&str>> = HashSet::from_iter(ans);
        assert!(ans_hash.symmetric_difference(&my_hash).next().is_none());
    }
}
