pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        Vec::from_iter((0..num_rows).scan(vec![1], |row, _| {
            let new_row = Vec::from_iter(
                std::iter::once(1)
                    .chain(
                        row.iter()
                            .zip(row.iter().skip(1))
                            .map(|(l, r): (&i32, &i32)| (*l).checked_add(*r).unwrap()),
                    )
                    .chain(std::iter::once(1)),
            );
            Some(std::mem::replace(row, new_row))
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::generate(0), vec![vec![]; 0]);
        assert_eq!(Solution::generate(1), vec![vec![1]]);
        assert_eq!(Solution::generate(2), vec![vec![1], vec![1, 1]]);
        assert_eq!(
            Solution::generate(3),
            vec![vec![1], vec![1, 1], vec![1, 2, 1]]
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

