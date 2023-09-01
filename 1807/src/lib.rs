pub struct Solution;

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let mut knowledge = knowledge;
        knowledge.sort_unstable_by(|k_v_0, k_v_1| k_v_0.iter().cmp(k_v_1.iter()));
        let mut brackets_indices = s
            .chars()
            .enumerate()
            .filter(|(_, c)| &'(' == c)
            .zip(s.chars().enumerate().filter(|(_, c)| &')' == c));
        let mut ret = String::with_capacity(s.len());
        s.chars().fold(false, |is_in_brackets, c| {
            if !is_in_brackets {
                if '(' == c {
                    let (start, end) = brackets_indices.next().unwrap();
                    let key = &s[start.0 + 1..end.0];
                    ret.push_str(
                        knowledge
                            .binary_search_by(|kv| kv.get(0).map(|s| s.as_str()).unwrap().cmp(key))
                            .map_or("?", |kv_idx| knowledge.get(kv_idx).unwrap().get(1).unwrap()),
                    );
                    true
                } else if ')' == c {
                    panic!("Unmatched right parenthesis ')'")
                } else {
                    ret.push(c);
                    false
                }
            } else {
                ')' != c
            }
        });
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        let s = "(name)is(age)yearsold".to_owned();
        let knowledge = vec![
            vec!["name".to_owned(), "bob".to_owned()],
            vec!["age".to_owned(), "two".to_owned()],
        ];
        assert_eq!(
            Solution::evaluate(s, knowledge),
            "bobistwoyearsold".to_owned()
        );
        let s = "hi(name)".to_owned();
        let knowledge = vec![vec!["a".to_owned(), "b".to_owned()]];
        assert_eq!(Solution::evaluate(s, knowledge), "hi?".to_owned());
        let s = "(a)(a)(a)aaa".to_owned();
        let knowledge = vec![vec!["a".to_owned(), "yes".to_owned()]];
        assert_eq!(Solution::evaluate(s, knowledge), "yesyesyesaaa".to_owned());
    }
}
