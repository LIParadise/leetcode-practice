use super::Solution;
#[test]
fn test_soln() {
    assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
    assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
    assert_eq!(
        Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()),
        false
    );
    assert_eq!(
        Solution::is_match("mississippi".to_string(), "mis*is*ip*.".to_string()),
        true
    );
    assert_eq!(
        Solution::is_match("aaa".to_string(), "ab*a".to_string()),
        false
    );
    assert_eq!(
        Solution::is_match("aaa".to_string(), "ab*a*c*a".to_string()),
        true
    );
    assert_eq!(Solution::is_match("a".to_string(), "ab*".to_string()), true);
    assert_eq!(
        Solution::is_match("a".to_string(), ".*..a*".to_string()),
        false
    );
    assert_eq!(
        Solution::is_match("ab".to_string(), ".*..".to_string()),
        true
    );
    assert_eq!(Solution::is_match("a".to_string(), ".*.".to_string()), true);
    assert_eq!(
        Solution::is_match(
            "aaaaaaaaaaaaab".to_string(),
            "a*a*a*a*a*a*a*a*a*a*c".to_string()
        ),
        false
    );
    assert_eq!(
        Solution::is_match(
            "caaaaccabcacbaac".to_string(),
            "b*.*..*bba.*bc*a*bc".to_string(),
        ),
        false
    );
}
