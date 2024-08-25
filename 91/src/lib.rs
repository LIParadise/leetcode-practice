pub struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        Self::num_decodings_worker(&s)
    }

    /// Dynamic programming
    ///
    /// given string, build from increasing len
    /// for each len,
    /// consider both the last two chars as token or only the last one
    fn num_decodings_worker(s: &str) -> i32 {
        (1..=s.len())
            .fold((0, 1), |(dp_exclude_2, dp_exclude_1), subproblem_len| {
                let mut ret = 0;

                // last token is two chars
                ret += subproblem_len
                    .checked_sub(2)
                    .and_then(|start| {
                        s.get(start..subproblem_len).map(|token| {
                            if Self::whole_str_to_int(token) {
                                dp_exclude_2
                            } else {
                                0
                            }
                        })
                    })
                    .unwrap_or(0);

                // last token is one char
                ret += s
                    .get(subproblem_len - 1..subproblem_len)
                    .map(|token| {
                        if Self::whole_str_to_int(token) {
                            dp_exclude_1
                        } else {
                            0
                        }
                    })
                    .unwrap_or(0);

                /*
                println!(
                    "{subproblem_len}: {:?} -> {:?}",
                    (dp_exclude_2, dp_exclude_1),
                    (dp_exclude_1, ret)
                );
                */

                (dp_exclude_1, ret)
            })
            .1
    }

    fn whole_str_to_int(s: &str) -> bool {
        match s {
            "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" | "11" | "12" | "13"
            | "14" | "15" | "16" | "17" | "18" | "19" | "20" | "21" | "22" | "23" | "24" | "25"
            | "26" => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        let s = "12".to_owned();
        assert_eq!(Solution::num_decodings(s), 2);
        let s = "226".to_owned();
        assert_eq!(Solution::num_decodings(s), 3);
        let s = "06".to_owned();
        assert_eq!(Solution::num_decodings(s), 0);
        let s = "9".to_owned();
        assert_eq!(Solution::num_decodings(s), 1);
    }
}
