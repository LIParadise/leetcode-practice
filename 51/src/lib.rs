pub struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        if n > u8::MAX as usize {
            panic!("Really? N-Queen problem where N larger than {}?", u8::MAX);
        } else if n == 1 {
            return vec![vec!["Q".to_string()]];
        } else if n == 2 || n == 3 {
            return Vec::new();
        }

        /// Notice a valid N-Queen must be s.t. the cols of Queens of each row
        /// must form a permutation of [0..N]
        ///
        /// To help construct permutations of [0..N], we build a "draw slot".
        ///
        /// What?
        ///
        /// Consider N balls for N numbered ppl, each take a ball in order
        /// The remaining balls for the k-th ppl is just an array of len N-k
        /// I.e. the k-th ppl only cares what in [0..(N-k)] he chooses
        /// Hence if we walk through [0..N]x[0..(N-1)]x[0..(N-2)]X...X[0..1]
        /// we have all permutations of N balls.
        ///
        /// Concretely, e.g. if N is 3, we store [A, B] and implicitly C
        /// [A, B, C] is initially [0, 0, 0], followed by
        /// [0, 1, 0] -> [1, 0, 0] -> [1, 1, 0] -> [2, 0, 0] -> [2, 1, 0]
        struct DrawFromSlot {
            vec: Vec<usize>,
            can_next: bool,
        }
        impl DrawFromSlot {
            fn new(n: usize) -> Self {
                let mut vec = Vec::with_capacity(n - 1);
                (0..(n - 1)).for_each(|_| vec.push(0));
                Self {
                    vec,
                    can_next: true,
                }
            }
        }
        impl Iterator for DrawFromSlot {
            type Item = Vec<usize>;
            fn next(&mut self) -> Option<Self::Item> {
                if self.can_next {
                    let ret = self.vec.clone();
                    let mut carry = false;
                    *self.vec.last_mut().unwrap() += 1;
                    for (idx, val) in self.vec.iter_mut().rev().enumerate() {
                        if carry == true {
                            *val = *val + 1;
                        }
                        if *val > idx + 1 {
                            *val = 0;
                            carry = true;
                        } else {
                            carry = false;
                            break;
                        }
                    }
                    self.can_next = !carry;
                    Some(ret)
                } else {
                    None
                }
            }
        }

        let mut candidate = Vec::with_capacity(n);
        let mut indexed_balls = Vec::with_capacity(n);
        let mut ret = Vec::new();
        for s in DrawFromSlot::new(n) {
            indexed_balls.clear();
            candidate.clear();
            (0..n as u8).for_each(|i| indexed_balls.push(i));
            for col in s {
                candidate.push(indexed_balls.remove(col));
            }
            candidate.push(indexed_balls.remove(0));
            if Self::check(&candidate) {
                ret.push(Self::make_board(&candidate));
            }
        }
        ret
    }

    /// Input: col idx of queen of each row
    /// E.g. [1, 3, 0, 2] is a soln to 4-Queen
    /// Since _ Q _ _
    ///       _ _ _ Q
    ///       Q _ _ _
    ///       _ _ Q _ is a solution.
    #[inline]
    fn check(vec: &[u8]) -> bool {
        let ret = vec.iter().enumerate().all(|(row, col)| {
            // no same col
            vec.iter().enumerate().all(|(r, c)| r == row || c != col)
                && vec.iter().enumerate().all(|(r, c)| {
                    if r == row {
                        true
                    } else {
                        let on_backslash =
                            (r as u8).wrapping_sub(row as u8) == c.wrapping_sub(*col as u8);
                        let on_slash =
                            (row as u8).wrapping_sub(r as u8) == c.wrapping_sub(*col as u8);
                        !on_slash && !on_backslash
                    }
                })
        });
        return ret;
    }
    fn make_board(vec: &[u8]) -> Vec<String> {
        vec.iter()
            .map(|&col| {
                let mut row = Vec::with_capacity(vec.len());
                while row.len() < vec.len() {
                    row.push('.' as u8);
                }
                row[col as usize] = 'Q' as u8;
                String::from_utf8(row).unwrap()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use std::collections::BTreeSet as BTS;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::solve_n_queens(4).into_iter().collect::<BTS<_>>(),
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."]
            ]
            .into_iter()
            .map(|vec_of_str| vec_of_str.into_iter().map(|s| s.to_string()).collect())
            .collect::<BTS<_>>()
        );
        assert_eq!(
            Solution::solve_n_queens(1).into_iter().collect::<BTS<_>>(),
            vec![vec!["Q"]]
                .into_iter()
                .map(|vec_of_str| vec_of_str.into_iter().map(|s| s.to_string()).collect())
                .collect::<BTS<_>>()
        );
    }
    #[test]
    fn test_check() {
        assert_eq!(Solution::check(&vec![1, 3, 2, 0]), false);
    }
}
