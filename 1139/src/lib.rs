pub struct Solution;

impl Solution {
    /// Given a 2D grid of 0s and 1s, return the number of elements in the
    /// largest square subgrid that has all 1s on its border, or 0 if such a
    /// subgrid doesn't exist in the grid.
    ///
    /// Example:
    /// ```
    /// use lc_1139_largest_1__bordered_square::Solution as S;
    /// let grid = Vec::from([
    ///     Vec::from([1, 1, 1]),
    ///     Vec::from([1, 0, 1]),
    ///     Vec::from([1, 1, 1]),
    /// ]);
    /// assert_eq!(S::largest1_bordered_square(grid), 9);
    /// let grid = Vec::from([
    ///     Vec::from([1, 1, 0, 1]),
    /// ]);
    /// assert_eq!(S::largest1_bordered_square(grid), 1);
    /// ```
    // Idea: create two auxiliary grids, on which records the consecutive 1s
    // to the left and to the up, resp.
    // If there's a square with border being all 1s, we should be able to
    // tell directly via these auxiliary grids: check the left-bottom,
    // right-bottom, and top-right, on corresponding auxiliary grids.
    pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
        let is_non_empty_rectangle = match &grid[..] {
            [head, tail @ ..] => tail.iter().all(|row| row.len() == head.len()),
            [] => false,
        };
        if !is_non_empty_rectangle {
            0
        } else if grid.len() == 1 {
            // leetcode compiler too old...
            // grid[0]
            //     .iter()
            //     .all(|value| value == &0)
            //     .then_some(0)
            //     .map_or(1, |ans| ans)
            if grid[0].iter().all(|value| value == &0) {
                0
            } else {
                1
            }
        } else if grid[0].len() == 1 {
            // leetcode compiler too old...
            // grid.iter()
            //     .all(|row| row[0] == 0)
            //     .then_some(0)
            //     .map_or(1, |ans| ans)
            if grid.iter().all(|row| row[0] == 0) {
                0
            } else {
                1
            }
        } else {
            // rectangle, with both width and height >= 2

            // init
            let (mut consecutive_1s_upwards, mut consecutive_1s_left) = {
                let mut tmp = Vec::with_capacity(grid.len());
                grid.iter().for_each(|row| {
                    let mut tmptmp = Vec::with_capacity(row.len());
                    for _ in 0..row.len() {
                        tmptmp.push(0);
                    }
                    tmp.push(tmptmp);
                });
                (tmp.clone(), tmp.clone())
            };
            grid.iter().enumerate().for_each(|(row_idx, row)| {
                row.iter().enumerate().for_each(|(col_idx, val)| {
                    if val != &0 {
                        consecutive_1s_left[row_idx][col_idx] = 1 + consecutive_1s_left[row_idx]
                            .get(col_idx.wrapping_sub(1))
                            .map_or(0, |&tmp| tmp);
                        consecutive_1s_upwards[row_idx][col_idx] = 1 + consecutive_1s_upwards
                            .get(row_idx.wrapping_sub(1))
                            .map_or(0, |r| r[col_idx]);
                    }
                })
            });

            println!("l{:?}", consecutive_1s_left);
            println!("u{:?}", consecutive_1s_upwards);
            let max_possible_width_of_square = grid.len().min(grid[0].len());
            (1..=max_possible_width_of_square)
                .rev()
                .try_for_each(|side| {
                    (0..=grid.len() - side).try_for_each(|row_idx| {
                        (0..=grid[0].len() - side).try_for_each(|col_idx| {
                            if side <= consecutive_1s_upwards[row_idx + side - 1][col_idx]
                                && side
                                    <= consecutive_1s_upwards[row_idx + side - 1]
                                        [col_idx + side - 1]
                                && side <= consecutive_1s_left[row_idx][col_idx + side - 1]
                                && side
                                    <= consecutive_1s_left[row_idx + side - 1][col_idx + side - 1]
                            {
                                Err(side)
                            } else {
                                Ok(())
                            }
                        })
                    })
                })
                .map_or_else(|ans| (ans * ans) as i32, |_not_found| 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        let grid = Vec::from([Vec::from([0, 0]), Vec::from([0, 1])]);
        assert_eq!(Solution::largest1_bordered_square(grid), 1);
    }
}
