pub struct Solution;

impl Solution {
    /// Assuming all entries are non-negative
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        // dynamic programming:
        // we go only either right or down,
        // s.t. when we consider row by row and column by column,
        // cost of each entry is readily known via left/up neighbors.
        (0..grid.len()).for_each(|row_idx| {
            (0..grid[row_idx].len()).for_each(|col_idx| {
                let up = row_idx.checked_sub(1).and_then(|r| grid[r].get(col_idx));
                let left = col_idx.checked_sub(1).map(|c| grid[row_idx][c]);
                grid[row_idx][col_idx] += match (up, left.as_ref()) {
                    (None, None) => 0,
                    (Some(x), None) | (None, Some(x)) => *x,
                    (Some(x), Some(y)) => *std::cmp::min(x, y),
                }
            });
        });
        *grid.last().and_then(|row| row.last()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            12
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

