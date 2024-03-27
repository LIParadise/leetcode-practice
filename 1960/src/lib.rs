pub struct Solution;

impl Solution {
    pub fn max_product(s: String) -> i64 {
        let s = s.as_bytes();
        let symmetric_len = Vec::from_iter((0..s.len()).map(|i| {
            let (l, r) = s.split_at(i);
            l.iter()
                .rev()
                .zip(r.iter().skip(1))
                .take_while(|(a, b)| a == b)
                .count()
        }));
        let mut left_right = vec![(0, 0); s.len()];
        symmetric_len
            .iter()
            .scan(None, |thres, &x| {
                Some(match thres {
                    None => true,
                    Some(y) if x > *y => {
                        *thres = Some(x);
                        true
                    }
                    _ => false,
                })
            })
            .zip(symmetric_len.iter().enumerate())
            .for_each(|(updated, (i, &x))| {
                if updated {
                    left_right[i..=i + x]
                        .iter_mut()
                        .enumerate()
                        .rev()
                        .try_for_each(|(l, y)| {
                            if 2 * l + 1 > y.0 {
                                y.0 = 2 * l + 1;
                                Ok(())
                            } else {
                                Err(())
                            }
                        })
                        .ok();
                }
            });
        symmetric_len
            .iter()
            .rev()
            .scan(None, |thres, &x| {
                Some(match thres {
                    None => true,
                    Some(y) if x > *y => {
                        *thres = Some(x);
                        true
                    }
                    _ => false,
                })
            })
            .zip(symmetric_len.iter().enumerate().rev())
            .for_each(|(updated, (i, &x))| {
                if updated {
                    left_right[i - x..=i]
                        .iter_mut()
                        .rev()
                        .enumerate()
                        .rev()
                        .try_for_each(|(l, y)| {
                            if 2 * l + 1 > y.1 {
                                y.1 = 2 * l + 1;
                                Ok(())
                            } else {
                                Err(())
                            }
                        })
                        .ok();
                }
            });
        #[cfg(test)]
        {
            println!("{:?}", s);
            println!("s{:?}", symmetric_len);
            println!("l{:?}", left_right.iter().map(|p| p.0).collect::<Vec<_>>());
            println!("r{:?}", left_right.iter().map(|p| p.1).collect::<Vec<_>>());
        }
        (0..left_right.len()).for_each(|i| {
            let (a, b) = left_right.split_at_mut(i);
            if let Some(l) = a.last() {
                b[0].0 = std::cmp::max(l.0, b[0].0);
            }
        });
        (0..left_right.len()).rev().for_each(|i| {
            let (a, b) = left_right.split_at_mut(i);
            if let Some(l) = a.last_mut() {
                l.1 = std::cmp::max(l.1, b[0].1);
            }
        });
        #[cfg(test)]
        {
            println!("l{:?}", left_right.iter().map(|p| p.0).collect::<Vec<_>>());
            println!("r{:?}", left_right.iter().map(|p| p.1).collect::<Vec<_>>());
        }
        left_right
            .iter()
            .zip(left_right.iter().skip(1))
            .max_by(|(a, b), (c, d)| (a.0 * b.1).cmp(&(c.0 * d.1)))
            .map(|(a, b)| a.0 * b.1)
            .unwrap()
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::max_product("ababbb".to_owned()), 9);
        assert_eq!(Solution::max_product("zaaaxbbby".to_owned()), 9);
        assert_eq!(
            Solution::max_product("ggbswiymmlevedhkbdhntnhdbkhdevelmmyiwsbgg".to_owned()),
            45
        );
        assert_eq!(
            Solution::max_product("wtbptdhbjqsrwkxccxkwrsqjbhdtpbtw".to_owned()),
            1
        );
    }
}
