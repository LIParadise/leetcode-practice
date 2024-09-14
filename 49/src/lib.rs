pub struct Solution;

use std::collections::{BTreeMap, HashMap};
use std::str::FromStr;

impl Solution {
    pub fn group_anagrams(mut strs: Vec<String>) -> Vec<Vec<String>> {
        // hash sharding
        // further sharding could be implemented using e.g. XOR of str,
        // but I'm not implementing that here.
        strs.sort_unstable_by_key(String::len);

        let (_, ret) = strs.iter().map(String::as_str).fold(
            (
                None,
                Vec::<HashMap<BTreeMap<char, usize>, Vec<&str>>>::new(),
            ),
            |(last_str_len, mut list_of_db_of_different_len), s| {
                let mut str_char_to_count = BTreeMap::new();
                s.chars().for_each(|c| {
                    str_char_to_count
                        .entry(c)
                        .and_modify(|x| *x += 1)
                        .or_insert(1);
                });
                if last_str_len.is_none() || last_str_len != Some(s.len()) {
                    list_of_db_of_different_len.push(HashMap::from_iter([(
                        str_char_to_count,
                        Vec::from_iter([s]),
                    )]));
                } else {
                    list_of_db_of_different_len
                        .last_mut()
                        .unwrap()
                        .entry(str_char_to_count)
                        .and_modify(|v| v.push(s))
                        .or_insert(Vec::from_iter([s]));
                }
                (Some(s.len()), list_of_db_of_different_len)
            },
        );
        Vec::from_iter(ret.into_iter().flatten().map(|(_key, strs)| {
            Vec::from_iter(strs.into_iter().map(String::from_str).map(Result::unwrap))
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        let input_strs = ["eat", "tea", "tan", "ate", "nat", "bat"];
        let input_strs = Vec::from_iter(input_strs.into_iter().map(|s| s.to_owned()));
        let mine = Solution::group_anagrams(input_strs);
        let expected = vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]];
        let expected = Vec::from_iter(
            expected
                .into_iter()
                .map(|v| Vec::from_iter(v.into_iter().map(|s| s.to_owned()))),
        );
        println!("{mine:?}");
        println!("{expected:?}");

        let input_strs = [""];
        let input_strs = Vec::from_iter(input_strs.into_iter().map(|s| s.to_owned()));
        let mine = Solution::group_anagrams(input_strs);
        let expected = vec![vec![""]];
        let expected = Vec::from_iter(
            expected
                .into_iter()
                .map(|v| Vec::from_iter(v.into_iter().map(|s| s.to_owned()))),
        );
        println!("{mine:?}");
        println!("{expected:?}");

        let input_strs = ["a"];
        let input_strs = Vec::from_iter(input_strs.into_iter().map(|s| s.to_owned()));
        let mine = Solution::group_anagrams(input_strs);
        let expected = vec![vec!["a"]];
        let expected = Vec::from_iter(
            expected
                .into_iter()
                .map(|v| Vec::from_iter(v.into_iter().map(|s| s.to_owned()))),
        );
        println!("{mine:?}");
        println!("{expected:?}");
    }
}

macro_rules! lprintln {
    // Match when a format string and additional arguments are provided
    ($fmt:expr, $($arg:tt)*) => {{
        if cfg!(feature = "local_test") {
            println!($fmt, $($arg)*);
        }
    }};

    // Match when only a format string is provided
    ($fmt:expr) => {{
        if cfg!(feature = "local_test") {
            println!($fmt);
        }
    }};
}
