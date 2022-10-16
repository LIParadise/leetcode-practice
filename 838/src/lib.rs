pub struct Solution;

enum State {
    R(usize),
    Null(usize),
}

use State::*;

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut state = Null(0);
        let dominoes: Vec<char> = dominoes.chars().collect();
        let mut ret = dominoes.clone();
        dominoes
            .iter()
            .enumerate()
            .for_each(|(idx, ch)| match state {
                Null(_) if ch == &'.' => {}
                Null(_) if ch == &'R' => state = R(idx),
                Null(last_idx) if ch == &'L' => {
                    for i in last_idx..idx {
                        ret[i] = 'L';
                        state = Null(idx);
                    }
                }
                R(_) if ch == &'.' => {}
                R(last_idx) if ch == &'R' => {
                    state = R(idx);
                    for i in last_idx + 1..idx {
                        ret[i] = 'R';
                    }
                }
                R(last_idx) if ch == &'L' => {
                    let idx_diff = idx - last_idx;
                    for i in (last_idx + 1..).take((idx_diff - 1) / 2) {
                        ret[i] = 'R';
                    }
                    for i in (idx - (idx_diff - 1) / 2)..idx {
                        ret[i] = 'L';
                    }
                    state = Null(idx + 1)
                }
                _ => panic!("Invalid character"),
            });
        if let R(idx) = state {
            ret.get_mut(idx..)
                .unwrap()
                .iter_mut()
                .for_each(|c| *c = 'R');
        }
        ret.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::push_dominoes("RR.L".to_string()),
            "RR.L".to_string()
        );
        assert_eq!(
            Solution::push_dominoes(".L.R...LR..L..".to_string()),
            "LL.RR.LLRRLL..".to_string()
        );
    }
}
