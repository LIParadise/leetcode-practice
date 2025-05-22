pub struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        Self::longest_valid_parentheses_worker(&s)
    }
    fn longest_valid_parentheses_worker(s: &str) -> i32 {
        // Idea:
        // Keep a stack of indices where '(' occurred;
        // each ')' pops the stack, s.t. we know exact location and length of this substr.
        // For each substr found, we may try concate it with previous entries to form a longer one.
        s.chars()
            .enumerate()
            .fold(
                (0, Vec::<(usize, usize)>::new(), vec![]),
                |(mut ret, mut known_substrs, mut left_paren_indices_stack), (i, ch)| {
                    match ch {
                        '(' => left_paren_indices_stack.push(i),
                        ')' => {
                            if let Some(j) = left_paren_indices_stack.pop() {
                                // the newly found substr may...
                                // 1. fully encompass the previous substr
                                // 2. touch the neighbor
                                // 3. non-zero distance from neighbor
                                if let Some(last_substr) = known_substrs.last_mut() {
                                    if j < last_substr.0 {
                                        // case 1., the tricky one:
                                        // fully encompass the last substr,
                                        // meaning the new one may actually neighbor an earlier substr
                                        let mut substr = (j, i);
                                        while let Some(second_last) =
                                            known_substrs.iter_mut().rev().nth(1)
                                        {
                                            if second_last.1 + 1 == substr.0 {
                                                // touches earlier neighbor:
                                                // they are distincts parts of a longer one!
                                                substr.0 = second_last.0;
                                                known_substrs.pop();
                                            } else {
                                                break;
                                            }
                                        }
                                        *known_substrs.last_mut().unwrap() = substr;
                                    } else if last_substr.1 + 1 == j {
                                        // case 2.
                                        last_substr.1 = i;
                                    } else if last_substr.1 + 1 < j {
                                        // case 3.
                                        known_substrs.push((j, i));
                                    } else {
                                        unreachable!()
                                    }
                                } else {
                                    known_substrs.push((j, i));
                                }
                                ret = std::cmp::max(
                                    ret,
                                    known_substrs.last().unwrap().1
                                        - known_substrs.last().unwrap().0
                                        + 1,
                                );
                            }
                        }
                        _ => unreachable!(),
                    };
                    (ret, known_substrs, left_paren_indices_stack)
                },
            )
            .0
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::longest_valid_parentheses(String::from("(()")), 2);
        assert_eq!(
            Solution::longest_valid_parentheses(String::from(")()())")),
            4
        );
        assert_eq!(Solution::longest_valid_parentheses(String::from("")), 0);
        assert_eq!(
            Solution::longest_valid_parentheses(String::from("))(((((()((((((()")),
            2
        );
        assert_eq!(
            Solution::longest_valid_parentheses(String::from("()(())")),
            6
        );
        assert_eq!(
            Solution::longest_valid_parentheses(String::from(")()(((())))(")),
            10
        );
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

