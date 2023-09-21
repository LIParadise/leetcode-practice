/// A sentence is a list of words that are separated by a single space with
/// no leading or trailing spaces. Each word consists of lowercase and
/// uppercase English letters.
///
/// A sentence can be shuffled by appending the 1-indexed word position to
/// each word then rearranging the words in the sentence.
///
/// For example, the sentence "This is a sentence" can be shuffled as
/// "sentence4 a3 is2 This1" or "is2 sentence4 This1 a3".
///
/// Given a shuffled sentence s containing no more than 9 words,
/// reconstruct and return the original sentence.
pub struct Solution;

impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut tmp = s.split_whitespace().collect::<Vec<_>>();
        tmp.sort_unstable_by(|a, b| {
            let a = a.chars().rev().take(1).next().unwrap().to_digit(10);
            let b = b.chars().rev().take(1).next().unwrap().to_digit(10);
            a.cmp(&b)
        });
        if 0 == tmp.len() {
            "".to_owned()
        } else {
            tmp.iter().skip(1).fold(
                {
                    let mut ret = String::with_capacity(s.len());
                    ret.push_str(&tmp[0][..tmp[0].len() - 1]);
                    ret
                },
                |ret, s| ret + " " + &s[..s.len() - 1],
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::sort_sentence("is2 sentence4 This1 a3".to_owned()),
            "This is a sentence".to_owned()
        );
        assert_eq!(
            Solution::sort_sentence("Myself2 Me1 I4 and3".to_owned()),
            "Me Myself and I".to_owned()
        );
    }
}
