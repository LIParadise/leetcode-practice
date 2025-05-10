pub struct Solution;

impl Solution {
    pub fn restore_ip_addresses(orig: String) -> Vec<String> {
        let remained = orig.as_str();
        Vec::from_iter(
            (1..=3)
                .flat_map(|i| Self::munch_token(remained, i))
                .flat_map(|(one, remained)| {
                    (1..=3).flat_map(move |j| {
                        Self::munch_token(remained, j).map(|(two, remained)| (one, two, remained))
                    })
                })
                .flat_map(|(one, two, remained)| {
                    (1..=3).flat_map(move |k| {
                        Self::munch_token(remained, k).and_then(|(three, fourth)| {
                            if Self::try_parse_ipv4_u8(fourth).is_some() {
                                Some(String::from_iter([one, ".", two, ".", three, ".", fourth]))
                            } else {
                                None
                            }
                        })
                    })
                }),
        )
    }
    fn munch_token(s: &str, u: usize) -> Option<(&str, &str)> {
        s.split_at_checked(u).and_then(|(token, remained)| {
            if Self::try_parse_ipv4_u8(token).is_some() {
                Some((token, remained))
            } else {
                None
            }
        })
    }
    fn try_parse_ipv4_u8(s: &str) -> Option<u8> {
        if let Ok(ret) = s.parse::<u8>() {
            if s.strip_prefix("0").map(str::len).unwrap_or_default() > 0 {
                None
            } else {
                Some(ret)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use std::collections::BTreeSet;
    #[test]
    fn test_soln() {
        let expected = BTreeSet::from_iter(["255.255.11.135", "255.255.111.35"]);
        let my_soln = Solution::restore_ip_addresses(String::from("25525511135"));
        let my_soln = BTreeSet::from_iter(my_soln.iter().map(String::as_str));
        assert!(my_soln.symmetric_difference(&expected).next().is_none());

        let expected = BTreeSet::from_iter(["0.0.0.0"]);
        let my_soln = Solution::restore_ip_addresses(String::from("0000"));
        let my_soln = BTreeSet::from_iter(my_soln.iter().map(String::as_str));
        assert!(my_soln.symmetric_difference(&expected).next().is_none());

        let expected = BTreeSet::from_iter([
            "1.0.10.23",
            "1.0.102.3",
            "10.1.0.23",
            "10.10.2.3",
            "101.0.2.3",
        ]);
        let my_soln = Solution::restore_ip_addresses(String::from("101023"));
        let my_soln = BTreeSet::from_iter(my_soln.iter().map(String::as_str));
        assert!(my_soln.symmetric_difference(&expected).next().is_none());
    }
}
