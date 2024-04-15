pub struct Solution;
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Kind {
    Lower,
    Upper,
    Digit,
    Punct,
    Null,
}
impl Kind {
    fn new(c: char) -> Self {
        match c {
            'a'..='z' => Self::Lower,
            'A'..='Z' => Self::Upper,
            '0'..='9' => Self::Digit,
            '.' | '!' => Self::Punct,
            _ => panic!("Char not in Leetcode specification"),
        }
    }
}
impl From<u8> for Kind {
    fn from(value: u8) -> Self {
        Self::new(value as char)
    }
}
impl From<char> for Kind {
    fn from(value: char) -> Self {
        Self::new(value)
    }
}
impl Into<u8> for Kind {
    fn into(self) -> u8 {
        let ret = match self {
            Kind::Lower => 'a',
            Kind::Upper => 'A',
            Kind::Digit => '0',
            Kind::Punct => '.',
            Kind::Null => 0 as char,
        } as u8;
        ret
    }
}
use std::collections::BTreeSet;

impl Solution {
    /// A strong string satisfies all the following:
    /// 1. Len in (6..=20)
    /// 2. Contains at least 1 uppercase, 1 lowercase, 1 digit
    /// 3. No 3 contiguous characters
    ///
    /// A step is one of the following:
    /// a. insert 1 char
    /// b. delete 1 char
    /// c. replace 1 char
    ///
    /// Determine min steps to make input strong
    pub fn strong_password_checker(password: String) -> i32 {
        // lemma: for cost, assume len valid before & after modification,
        // replace < insert < delete
        //
        // proof sketch:
        // "aaa": replace -> 1 (aab)
        //        insert -> 1 (aaba)
        //        delete -> 1 (aa)
        // "aaaa": replace -> 1 (aaba)
        //         insert -> 1 (aabaa)
        //         delete -> 2
        // "aaaaa": replace -> 1 (aabaa)
        //          insert -> 1 (aabaaba)
        //          delete -> 3
        // "aaaaaa": replace -> 2 (aabaab)
        //           insert -> 2 (aabaabaa)
        //           delete -> 4
        // "aaaaaaa": replace -> 2 (aabaaba)
        //            insert -> 3 (aabaabaaba)
        //            delete -> 4

        let mut pwd = password.as_bytes().to_owned();
        let mut pwd_kinds = BTreeSet::<Kind>::from_iter(pwd.iter().map(|&u| u.into()));
        let required_kinds =
            BTreeSet::<Kind>::from_iter([Kind::Upper, Kind::Lower, Kind::Digit].into_iter());

        let mut steps;
        match pwd.len() {
            l @ 0..=2 => {
                steps = 6 - l;
            }
            l @ 3..=5 => {
                // Ignore the no-contiguous condition.
                // Why?
                // 1. Only 3..=5 so must insert
                // 2.a For contiguous 3 or 4, an insert creates required hiatus,
                //     and we need insert anyway. Non-issue.
                // 2.b For contiguous 5, it's either "00000" or ".....",
                //     i.e. the upper/lower/digit criteria or not,
                //     and they happen to be handled correctly in these resp. branches
                let valid_kind_cnt = pwd_kinds.intersection(&required_kinds).count();
                match valid_kind_cnt {
                    2.. => {
                        // just insert new kind
                        steps = 6 - l;
                    }
                    1 => {
                        // at least 2 new kind required, no less than 2 steps
                        steps = std::cmp::max(2, 6 - l);
                    }
                    0 => {
                        // at least 3 new kind required, so no less than 3
                        // 3..=5 so no more than 3
                        steps = 3;
                    }
                    #[allow(unreachable_patterns)]
                    _ => panic!("This branch is unnecessary but Leetcode Rust compiler is so ancient it's required."),
                }
            }
            6..=50 => {
                // First, suppose length is OK (6..=20)
                // By lemma, we consider only replace, and as long as the contiguous 3
                // criteria is met we're happy.
                // Hence, we erase all the contiguous 3 intervals by replacing chars in them, till
                // there's no contiguous 3 interval.
                //
                // After which, we got two cases:
                // Case 1. Required upper/lower/digit kinds are met. We're done.
                // Case 2. Required upper/lower/digit kinds not met.
                // By pigeon hole principle, there must be enough _irrelevant_ characters s.t. we
                // could do arbitrary replace s.t. kinds are met.
                // Which is we're gonna do, and the cost is trivial.
                //
                //
                //
                // Second, suppose length is (21..=50)
                //
                // Observation 1: deletion is necessary
                //
                // Observation 2: if there's contiguous interval of which len is no less than 3,
                // they are of priority to delete from, since this would imply less replace
                // required in the first step.
                //
                // Observation 3: In particular, if the interval is of length mod 3 equal 0, they
                // are first priorities: 1 deletion here saves 1 replace later.
                // Furthermore, if it's mod 1, they are second priorities,
                // since 2 deletions save 1 replace.
                //
                // Finally, mod 2 is the least effective in terms of cost.
                // Case 1. we're (6..=20) when there's some contiguous 3 intervals left.
                // Don't care. First part takes good care of them.
                // Case 2. we're still (21..=50) but all contiguous intervals are cleared.
                // Again by pigeon hole, there must be enough _irrelevant_ chars s.t. the required
                // deletion is trivial task.
                steps = 0;
                let mut intervals_of_repeat = Self::repeat_3_or_more(pwd.as_ref());
                while pwd.len() > 20 {
                    if intervals_of_repeat.is_empty() {
                        break;
                    } else {
                        steps += 1;
                        let remove_this = intervals_of_repeat.last().unwrap().0;
                        pwd.remove(remove_this);
                        if intervals_of_repeat.last().unwrap().1
                            - intervals_of_repeat.last().unwrap().0
                            + 1
                            == 3
                        {
                            intervals_of_repeat.pop();
                        }
                        intervals_of_repeat.iter_mut().for_each(|(a, b)| {
                            if *a > remove_this {
                                *a -= 1;
                            }
                            if *b > remove_this {
                                *b -= 1;
                            }
                        });
                        Self::sort_repeat_by_mod_3(&mut intervals_of_repeat);
                    }
                }
                if pwd.len() > 20 {
                    steps += pwd.len() - 20;
                }
                while let Some(idx) = Self::repeat_3(pwd.as_ref()) {
                    steps += 1;
                    match required_kinds.difference(&pwd_kinds).next() {
                        Some(&k @ Kind::Lower | &k @ Kind::Upper | &k @ Kind::Digit) => {
                            pwd[idx + 2] = k.into();
                            pwd_kinds.insert(k);
                        }
                        _ => {
                            pwd[idx + 2] = Kind::Null.into();
                        }
                    }
                }
                steps += required_kinds.difference(&pwd_kinds).count();
            }
            51.. => panic!("Leetcode specifies no longer than 50, abort."),
            #[allow(unreachable_patterns)]
            _ => panic!(
                "This branch is unnecessary but Leetcode Rust compiler is so ancient it's required."
            ),
        }
        steps as i32
    }
    #[inline]
    fn repeat_3(pwd: &[u8]) -> Option<usize> {
        pwd.windows(3).position(|w| w[0] == w[1] && w[1] == w[2])
    }
    fn repeat_3_or_more(pwd: &[u8]) -> Vec<(usize, usize)> {
        let mut start = 0;
        let mut ret = Vec::new();
        while let Some(idx) = Self::repeat_3(&pwd[start..]) {
            let left = idx;
            let mut right = idx + 2;
            right += pwd[start..]
                .iter()
                .skip(right)
                .zip(pwd[start..].iter().skip(right + 1))
                .take_while(|(a, b)| a == b)
                .count();
            ret.push((start + left, start + right));
            start += right + 1;
        }
        Self::sort_repeat_by_mod_3(&mut ret);
        ret
    }
    fn sort_repeat_by_mod_3(v: &mut [(usize, usize)]) {
        v.sort_by(|a, b| ((b.1 - b.0 + 1) % 3).cmp(&((a.1 - a.0 + 1) % 3)));
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::strong_password_checker("1337C0d3".to_owned()), 0);
        assert_eq!(Solution::strong_password_checker("aA1".to_owned()), 3);
        assert_eq!(Solution::strong_password_checker("a".to_owned()), 5);
        assert_eq!(
            Solution::strong_password_checker("000000000000000000000000000".to_owned()),
            13
        );
        assert_eq!(
            Solution::strong_password_checker("000000000000000000000000000000".to_owned()),
            16
        );
        assert_eq!(
            Solution::strong_password_checker("bbaaaaaaaaaaaaaaacccccc".to_owned()),
            8
        );
        assert_eq!(
            Solution::strong_password_checker("aaaabbbbccccddeeddeeddeedd".to_owned()),
            8
        );
    }
}
