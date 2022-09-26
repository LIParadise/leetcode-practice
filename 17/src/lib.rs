struct Solution;

const DIGIT_CHAR_MAP: &[&str] = &[
    "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
];

impl Solution {
    /// LC spec:
    /// digits[i] is a digit in the range ['2', '9'].
    /// 0 <= digits.length <= 4
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return Vec::new();
        }
        const BASE: u32 = 10;
        digits
            .chars()
            .map(|d| d.to_digit(BASE).unwrap() as usize)
            .fold(vec![String::new()], |ret, idx| {
                let mut tmp: Vec<String> = vec![];
                for r in ret.iter() {
                    for c in DIGIT_CHAR_MAP[idx].chars() {
                        let mut s = r.clone();
                        s.push(c);
                        tmp.push(s);
                    }
                }
                tmp
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_letter_combinations() {
        assert_eq!(
            ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
            Solution::letter_combinations("23".to_string()).as_slice()
        );
        let empty_arr_of_str: &[&str] = &[];
        assert_eq!(
            empty_arr_of_str,
            Solution::letter_combinations("".to_string()).as_slice()
        );
        assert_eq!(
            ["a", "b", "c"],
            Solution::letter_combinations("2".to_string()).as_slice()
        );
    }
}
