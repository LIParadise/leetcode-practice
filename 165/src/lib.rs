use std::cmp;
pub struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        match Self::compare_version__(&version1, &version2) {
            cmp::Ordering::Less => -1,
            cmp::Ordering::Equal => 0,
            cmp::Ordering::Greater => 1,
        }
    }
    fn compare_version__(v0: &str, v1: &str) -> cmp::Ordering {
        fn make_sanitized_iter(s: &str) -> impl Iterator<Item = (bool, u32)> + '_ {
            s.split(".")
                .map(|s| (true, s.trim_start_matches('0').parse::<u32>().unwrap_or(0)))
                .chain(std::iter::repeat((false, 0)))
        }
        make_sanitized_iter(v0)
            .zip(make_sanitized_iter(v1))
            .take_while(|((v0_subversion_defined, _), (v1_subversion_defined, _))| {
                *v0_subversion_defined || *v1_subversion_defined
            })
            .find_map(
                |(
                    (v0_subversion_defined, v0_subversion),
                    (v1_subversion_defined, v1_subversion),
                )| {
                    match (
                        v0_subversion_defined,
                        v1_subversion_defined,
                        v0_subversion,
                        v1_subversion,
                    ) {
                        (false, false, _, _) => unreachable!(),
                        (false, true, _, v1_subversion) if v1_subversion > 0 => {
                            Some(cmp::Ordering::Less)
                        }
                        (false, true, _, _) => None,
                        (true, false, v0_subversion, _) if v0_subversion > 0 => {
                            Some(cmp::Ordering::Greater)
                        }
                        (true, false, _, _) => None,
                        (true, true, v0_subversion, v1_subversion) => {
                            match v0_subversion.cmp(&v1_subversion) {
                                cmp::Ordering::Equal => None,
                                ord => Some(ord),
                            }
                        }
                    }
                },
            )
            .unwrap_or(cmp::Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            -1,
            Solution::compare_version(String::from("1.2"), String::from("1.10"))
        );
        assert_eq!(
            0,
            Solution::compare_version(String::from("1.01"), String::from("1.001"))
        );
        assert_eq!(
            0,
            Solution::compare_version(String::from("1.0"), String::from("1.0.0.0"))
        );
    }
}
