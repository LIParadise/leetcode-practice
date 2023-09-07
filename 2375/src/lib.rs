//! You are given a 0-indexed string pattern of length `n` consisting of the characters 'I' meaning increasing and 'D' meaning decreasing.
//! A 0-indexed string num of length n + 1 is created using the following conditions:
//!     num consists of the digits '1' to '9', where each digit is used at most once.
//!     If `pattern[i] == 'I'`, then `num[i] < num[i + 1]`.
//!     If `pattern[i] == 'D'`, then `num[i] > num[i + 1]`.
//! Return the lexicographically smallest possible string num that meets the conditions.
pub struct Solution;

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let mut ret: Vec<u8> = Vec::with_capacity(9);
        ret.push(1);
        pattern.chars().skip_while(|c| &'D' == c).enumerate().fold(
            0,
            |last_i_idx, (idx, c)| match c {
                'I' => {
                    ret.push((ret.len() + 1) as u8);
                    idx
                }
                'D' => {
                    ret.push((ret.len() + 1) as u8);
                    let (_, to_rotate) = ret.split_at_mut(last_i_idx + 1);
                    to_rotate.rotate_right(1);
                    last_i_idx
                }
                _ => panic!("Unexpected pattern"),
            },
        );
        let initial_d = pattern.chars().take_while(|c| &'D' == c).count() as u8;
        dbg!(&ret);
        ret.iter_mut().for_each(|r| {
            if *r > 1 {
                *r = *r + initial_d
            }
        });
        (0..initial_d).for_each(|i| {
            ret.insert(0, i + 2);
        });
        ret.iter_mut().for_each(|c| *c = *c + '0' as u8);
        String::from_utf8(ret).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        let mine = Solution::smallest_number(String::from("IIIDIDDD"));
        let ans = String::from("123549876");
        dbg!(mine.len());
        dbg!(ans.len());
        assert_eq!(mine, ans);
        let mine = Solution::smallest_number(String::from("DDD"));
        let ans = String::from("4321");
        dbg!(mine.len());
        dbg!(ans.len());
        assert_eq!(mine, ans);
        let mine = Solution::smallest_number(String::from("DDIDDDD"));
        let ans = String::from("32187654");
        dbg!(mine.len());
        dbg!(ans.len());
        assert_eq!(mine, ans);
        let mine = Solution::smallest_number(String::from("DIDIDI"));
        let ans = String::from("2143657");
        dbg!(mine.len());
        dbg!(ans.len());
        assert_eq!(mine, ans);
        let mine = Solution::smallest_number(String::from("DDIDDIDD"));
        let ans = String::from("321654987");
        dbg!(mine.len());
        dbg!(ans.len());
        assert_eq!(mine, ans);
        let mine = Solution::smallest_number(String::from("DDIDIIDI"));
        let ans = String::from("321546879");
        dbg!(mine.len());
        dbg!(ans.len());
        assert_eq!(mine, ans);
    }
}
