pub struct Solution;

impl Solution {
    /// Given m by n 2D grid containing zero (included) to m times n (excluded),
    /// one moves from row to next row, column being arbitrary.
    ///
    /// Each movement from row to next row incurs a cost determined by
    /// (value of the current cell) modulo (which column of the next row we're heading to):
    /// the cost is indexing the second 2D array via (current value of cell)
    /// then (which column we're heading to),
    /// both zero-indexed.
    ///
    /// A path is one starting from the first row and ends up in the last row.
    /// Cost of such paths is cost of each movement plus the values of the cells.
    ///
    /// Given the grid and the cost map, determine a minimum cost path.
    pub fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
        Self::min_path_worker(&grid, &move_cost)
    }
    fn min_path_worker(grid: &[Vec<i32>], move_cost: &[Vec<i32>]) -> i32 {
        if grid.is_empty() {
            0
        } else {
            *grid
                .iter()
                .skip(1)
                .zip(grid)
                .fold(grid[0].clone(), |costs, (next_row, current_row)| {
                    Vec::from_iter(next_row.iter().enumerate().map(|(col, cost_from_grid)| {
                        *cost_from_grid
                            + current_row
                                .iter()
                                .zip(&costs)
                                .map(|(val, cost)| {
                                    move_cost[usize::try_from(*val).unwrap()][col] + *cost
                                })
                                .min()
                                .unwrap_or(0)
                    }))
                })
                .iter()
                .min()
                .unwrap_or(&0)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::min_path_cost(
                vec![vec![5, 3], vec![4, 0], vec![2, 1]],
                vec![
                    vec![9, 8],
                    vec![1, 5],
                    vec![10, 12],
                    vec![18, 6],
                    vec![2, 4],
                    vec![14, 3]
                ]
            ),
            17
        );

        assert_eq!(
            Solution::min_path_cost(
                vec![vec![5, 1, 2], vec![4, 0, 3]],
                vec![
                    vec![12, 10, 15],
                    vec![20, 23, 8],
                    vec![21, 7, 1],
                    vec![8, 1, 13],
                    vec![9, 10, 25],
                    vec![5, 3, 2]
                ]
            ),
            6
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
