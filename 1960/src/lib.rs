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
        symmetric_len.iter().enumerate().for_each(|(idx, &l)| {
            left_right[idx..=idx + l]
                .iter_mut()
                .enumerate()
                .for_each(|(l, i)| {
                    i.0 = std::cmp::max(i.0, l * 2 + 1);
                });
            left_right[idx - l..=idx]
                .iter_mut()
                .rev()
                .enumerate()
                .for_each(|(l, i)| {
                    i.1 = std::cmp::max(i.1, l * 2 + 1);
                });
        });
        #[cfg(test)]
        {
            println!("{:?}", s);
            println!("{:?}", symmetric_len);
            println!("{:?}", left_right);
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
            println!("{:?}", left_right);
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
    }
}
