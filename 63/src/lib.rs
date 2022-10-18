pub struct Solution;

static ERR_EMPTY_BOARD: &'static str = "Cowardly to refuse empty board";

/// You are given an m x n integer array grid. There is a robot initially located at the top-left corner (i.e., grid[0][0]). The robot tries to move to the bottom-right corner (i.e., grid[m-1][n-1]). The robot can only move either down or right at any point in time.
///
/// An obstacle and space are marked as 1 or 0 respectively in grid. A path that
/// the robot takes cannot include any square that is an obstacle.
///
/// Return the number of possible unique paths that the robot can take to reach
/// the bottom-right corner.
///
/// The testcases are generated so that the answer will be less than or equal to
/// 2 * 109.
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut cache = Vec::with_capacity(obstacle_grid.get(0).expect(ERR_EMPTY_BOARD).len());
        let (height, width) = (obstacle_grid.len(), obstacle_grid[0].len());
        for _ in 0..width {
            cache.push(0);
        }

        cache[0] = 1;

        for r in 0..height {
            if obstacle_grid[r][0] == 1 {
                cache[0] = 0;
            }
            for c in 1..width {
                if obstacle_grid[r][c] == 1 {
                    cache[c] = 0;
                } else {
                    cache[c] += cache[c - 1];
                }
            }
        }
        cache.last().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            2
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]),
            1
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0], vec![1]]),
            0
        );
    }
}
