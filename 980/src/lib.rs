pub struct Solution;

// Forbid accidental leaking private interface
// Do NOT add "///" comments to this module!
#[forbid(missing_docs)]
mod constants {
    pub const START: i32 = 1;
    pub const END: i32 = 2;
    pub const NO: i32 = -1;
    pub const BLANK: i32 = 0;
    pub const WORKING: i32 = i32::MAX;
}
use constants::*;

/// You are given an m x n integer array grid where grid[i][j] could be:
///
/// 1 representing the starting square. There is exactly one starting square.
/// 2 representing the ending square. There is exactly one ending square.
/// 0 representing empty squares we can walk over.
/// -1 representing obstacles that we cannot walk over.
///
/// Return the number of 4-directional walks from the starting square to the
/// ending square, that walk over every non-obstacle square exactly once.
///
/// Height * Width <= 20.
impl Solution {
    fn find_in_2d(grid: &Vec<Vec<i32>>, val: i32) -> (usize, usize) {
        grid.iter()
            .enumerate()
            .find_map(|(row_idx, row_arr)| {
                if let Some(col_idx) = row_arr.iter().enumerate().find_map(|(col_idx, &num)| {
                    if num == val {
                        Some(col_idx)
                    } else {
                        None
                    }
                }) {
                    Some((row_idx, col_idx))
                } else {
                    None
                }
            })
            .expect("No such element")
    }
    fn dfs(
        grid: &mut Vec<Vec<i32>>,
        (row, col): (usize, usize),
        cur_steps: usize,
        required_steps: usize,
    ) -> usize {
        match grid[row][col] {
            flag if flag == WORKING || flag == NO => 0,
            flag if flag == END && cur_steps == required_steps => 1,
            flag if flag == END => 0,
            flag if flag == BLANK || flag == START => {
                let flag_bak = flag;
                let up = (row.wrapping_sub(1), col);
                let down = (row + 1, col);
                let right = (row, col + 1);
                let left = (row, col.wrapping_sub(1));
                let mut ret = 0;
                // set DFS marker
                grid[row][col] = WORKING;
                if let Some(_) = grid.get_mut(up.0) {
                    ret += Self::dfs(grid, up, cur_steps + 1, required_steps);
                }
                if let Some(_) = grid.get_mut(down.0) {
                    ret += Self::dfs(grid, down, cur_steps + 1, required_steps);
                }
                if let Some(_) = grid[row].get(right.1) {
                    ret += Self::dfs(grid, right, cur_steps + 1, required_steps);
                }
                if let Some(_) = grid[row].get(left.1) {
                    ret += Self::dfs(grid, left, cur_steps + 1, required_steps);
                }
                // Remember to reset DFS marker!
                grid[row][col] = flag_bak;
                ret
            }
            _ => panic!("Invalid grid element"),
        }
    }
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        // Count valid DFS paths.
        // Assume only 1 start and 1 end as specified.
        let mut grid = grid;
        // Plus 1 since END
        let required_steps = grid.iter().flatten().filter(|&n| n == &BLANK).count() + 1;
        let start = Self::find_in_2d(&grid, START);
        Self::dfs(&mut grid, start, 0, required_steps) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]]),
            2
        );
        assert_eq!(
            Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]]),
            4
        );
        assert_eq!(Solution::unique_paths_iii(vec![vec![0, 1], vec![2, 0]]), 0);
    }
}
