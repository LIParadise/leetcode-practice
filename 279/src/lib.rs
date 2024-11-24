use std::sync::OnceLock;

pub struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        /// Given input `n`, i.e. asking the min number of squares that some to `n`,
        /// result is stored at index `n` of the array.
        static KNOWN_SOLUTIONS: OnceLock<Vec<usize>> = OnceLock::new();

        KNOWN_SOLUTIONS.get_or_init(|| vec![0, 1]);
        (match KNOWN_SOLUTIONS.get() {
            Some(known_solutions) if known_solutions.len() > n as usize => {
                known_solutions[n as usize]
            }
            Some(known_solutions) => {
                let mut known_solutions = Vec::clone(known_solutions);
                while known_solutions.len() <= n as usize {
                    let best_sqrt = (1..)
                        .take_while(|sqrt| known_solutions.len() / sqrt >= *sqrt)
                        .min_by(|a, b| {
                            known_solutions[known_solutions.len() - a * a]
                                .cmp(&known_solutions[known_solutions.len() - b * b])
                        })
                        .unwrap();
                    known_solutions
                        .push(known_solutions[known_solutions.len() - best_sqrt * best_sqrt] + 1);
                }
                let ret = *known_solutions.last().unwrap();
                KNOWN_SOLUTIONS.set(known_solutions).ok();
                ret
            }
            None => unreachable!(),
        }) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::num_squares(1), 1);
        assert_eq!(Solution::num_squares(2), 2);
        assert_eq!(Solution::num_squares(3), 3);
        assert_eq!(Solution::num_squares(4), 1);
        assert_eq!(Solution::num_squares(5), 2);
        assert_eq!(Solution::num_squares(12), 3);
        assert_eq!(Solution::num_squares(13), 2);
    }
}
