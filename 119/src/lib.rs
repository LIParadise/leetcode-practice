pub struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = usize::try_from(row_index).unwrap();
        let mut ret = vec![1; row_index.checked_add(1).unwrap()];
        (1..row_index / 2 + 1)
            .zip((1..=row_index).rev())
            .for_each(|(col_idx, numerator)| {
                // Take the 4-th row (0-indexed) of Pascal's triangle for example: [1, 4, 6, 4, 1]
                // Note the relation between Pascal's triangle and combinatorial numbers:
                // they are resp. C(4, 0), C(4, 1), C(4, 2), C(4, 3), C(4, 4)
                //                1
                //                         4/1
                //                                  (4*3)/(1*2)
                //                                           (4*3*2)/(1*2*3)
                //                                                    (4*3*2*1)/(1*2*3*4)
                ret[col_idx] =
                    i32::try_from(ret[col_idx - 1] as i64 * numerator as i64 / col_idx as i64)
                        .unwrap();
                // `row_index` is basically the length of the row,
                // based on which we mirror what we've calculated
                ret[row_index - col_idx] = ret[col_idx];
            });
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::get_row(0), vec![1]);
        assert_eq!(Solution::get_row(1), vec![1, 1]);
        assert_eq!(Solution::get_row(2), vec![1, 2, 1]);
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
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

