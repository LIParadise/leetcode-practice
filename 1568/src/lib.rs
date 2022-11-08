pub struct Solution;

impl Solution {
    /// You are given an m x n binary grid grid where 1 represents land and 0
    /// represents water. An island is a maximal 4-directionally (horizontal or
    /// vertical) connected group of 1's.
    ///
    /// The grid is said to be connected if we have exactly one island, otherwise is
    /// said disconnected.
    /// In one day, we are allowed to change any single land cell (1) into a water
    /// cell (0).
    ///
    /// Return the minimum number of days to disconnect the grid.
    pub fn min_days(grid: Vec<Vec<i32>>) -> i32 {
        // Given a land at center of 3x3 grid, define the following:
        // (land: O, sea: ~, undetermined: ?)
        // (the grid as a whole is deemed as on infinite sea)
        // (the n in type-n: # of neighbors in up, down, left, and right)
        // (weak: removal of center increases locally (on 3x3) # of isolated
        //        lands)
        // (strong: removal of center doesn't introduce local # island inc.)
        //
        // ?~?
        // ~O~ type-0; strong (#=2^4=16)
        // ?~?
        //
        // ?O?
        // ~O~ type-1; strong (#=4*2^4=64)
        // ?~?
        //
        // ?~?
        // OOO type-2-I; weak (#=2*2^4=32)
        // ?~?
        //
        // ?O~
        // ~OO type-2-pure-L; weak (#=4*2^3=32)
        // ?~?
        //
        // ?OO
        // ~OO type-2-square; strong (#=4*2^3=32)
        // ?~?
        //
        // ?O~
        // OOO type-3-T; weak (#=4*2^2*3=48)
        // ?~?
        //
        // OOO
        // OOO type-3-square; strong (#=4*2^2=16)
        // ?~?
        //
        // ?O~ ~O~
        // OOO OOO type-4-cross; weak (#=4*2-1+4=11)
        // ~O? OOO
        //
        // ~OO OOO
        // OOO OOO type-4-square; strong (#=4+1=5)
        // OOO OOO
        //
        // By # we have 16+64+32+32+32+48+16+11+5 = 256 = 2^8
        // A simple verification that our classification is well-defined
        //
        // Notice that the maximum required days is 2:
        // Assume genus is 0, i.e. local separation means global separation
        // Assume also that at the beginning the island is connected.
        // For grid point that's classified as weak, there exists trivial
        // 1-day solution, i.e. remove the center.
        // By genus 0 assumption such local separation means global separation.
        // We're done.
        // For grid config that's composed entirely of strong grids, notice
        // that all type-2, type-3, type-4 which are strong have trivial 2-day
        // soln leading to local separation, by genus 0 assumption that's done.
        // Are there configs composed entirely type-1 and type-0?
        // That's trivial config, i.e. one singleton type-0 or two adjacent
        // grids both of type-1; the former is 1-day soln while the latter is
        // 2-day.
        // Hence maximum is 2 day.
        //
        // Genus non-zero case is troublesome:
        // Local separation doesn't necessarily lead to global separation:
        // Example: grid 3x5, OOOOO   OOwOO
        //                    OO~OO = OO~OO
        //                    OOOOO   OOwOO
        // Here there are 2 weak grids marked as 'w' and removal of either
        // grid doesn't introduce separation.
        // Actually, not really.
        // Assume there's type-2, type-1, then just remove its neighbor at
        // right, up, down, or left, and we have a separated singleton.
        // There must be such grids, since the grid is finite.
        // The glitch happens when removal of 1/2 grids leads to singleton on
        // board, which is itself connected. These cases could be solved via
        // brute force (grid # <= 3) and verify that they indeed require at
        // most 2 days. Q.E.D.
        //
        // Define "peninsula": island that's composed entirely of either strong
        // grids or weak grids.
        // The grid can now be thought of being composed of several peninsulae.
        // Notice that a weak peninsula (peninsula composed entirely of weak
        // grids) is either attached to some (possibly more than 1) strong
        // peninsula or being detached from anyone and have non-zero genus.
        // Proof: notice weak grids must extend since they are type-n where
        // n no less than 2. Finite grid, must form loop, since any "solid"
        // chunk leads to strong node, violating the detached from strong
        // premise.
        // Hence if the grid has only 1 island composed of 1 weak peninsula,
        // it's two-day soln.
        // E.g. 3x5 wwwww
        //          w~w~w
        //          wwwww
        // Similarly if it's only 1 island composed of 1 strong peninsula,
        // it's 1 day soln iff it's singleton.
        //
        // One may be tempted to think that 1-day soln happen only on some
        // weak grid within some weak peninsula and that the weak grid must
        // be right next to some strong peninsula. WRONG.
        //
        // Consider this 7x11 grid: SSSSS~~~www
        //                          SSSSw~~~w~w
        //                          SSS~w~~~w~w
        //                          SSwwwwwww~w
        //                          SSS~w~~~w~w
        //                          SSSSw~~~w~w
        //                          SSSSS~~~w~w
        //
        // All in all, brute force on all weak grids, let's gooooooo

        struct WeakStrongHelper<'a> {
            grid: &'a mut Vec<Vec<i32>>,
            first_island_size: usize,
            total_island_size: usize,
        }
        impl<'a> WeakStrongHelper<'a> {
            /// Assuming all land have same non-zero value and all sea are 0, return if there's
            /// 1-day soln.
            /// If there's no 1-day soln, all land would still have the same value, except that the
            /// value might change, e.g. all 42 becomes all 69.
            /// If there's 1-day soln, the land would have two different values: based on the
            /// viable remove grid, 1-day soln separates the once united land into 2, and one part
            /// would have m, the other part have (m+1).
            /// E.g. if given all 42, and have 1-day soln, the land might become some connected 69
            /// and some connected 70.
            /// The grid of removal of the 1-day soln would have (m+1).
            fn is_one_day(&mut self) -> bool {
                (0..self.grid.len())
                    .try_for_each(|r| {
                        (0..self.grid[r].len()).try_for_each(|c| {
                            if self.grid[r][c] != 0 && self.is_weak(r, c) {
                                // loop-invariant:
                                // Grid have one connected land, all marked some value, n.
                                // If after removal of `self.grid[r][c]` (i.e. make it 0, sea) the
                                // grid is still one connected land, then the grid is unchanged,
                                // except that all land are now (n+1) (including `self.grid[r][c]`)
                                // If after removal of `self.grid[r][c]` the grid changes from
                                // united/connected to separated, then one part of it, modulo
                                // `self.grid[r][c]`, would now have (n+1), while the other parts
                                // have (n).
                                //
                                // Notice this function FAILS TO WORK UPON RETURN `true`:
                                // since if there's 1-day soln then this function makes the once
                                // same-valued-and-connected grids into different values
                                self.grid[r][c] = 0;
                                let (suspect_r, suspect_c) = if r > 0
                                    && c < self.grid[r - 1].len()
                                    && self.grid[r][c] != 0
                                {
                                    (r - 1, c)
                                } else if r + 1 < self.grid.len()
                                    && c < self.grid[r + 1].len()
                                    && self.grid[r + 1][c] != 0
                                {
                                    (r + 1, c)
                                } else if c > 0 && self.grid[r][c - 1] != 0 {
                                    (r, c - 1)
                                } else if c + 1 < self.grid[r].len() && self.grid[r][c + 1] != 0 {
                                    (r, c + 1)
                                } else {
                                    unreachable!()
                                };
                                let remaining_island_size =
                                    Self::island_size_from(self.grid, suspect_r, suspect_c);
                                // Need to update the marker: since `Self::island_size_from`
                                // modifies all those who are same value and connected by adding 1
                                // to them, i.e. it counts maximally connected component based on
                                // value rather than merely asking if it's 0,
                                // need to update the value s.t. subsequent walk could work.
                                // E.g. if we have a bunch of connected value 42, after calling
                                // `Self::island_size_from` they would be 43.
                                // If we don't update correspondingly to 43, for sake of
                                // contradiction say we update to 1 rather than 43, subsequent walk
                                // would deem this grid as NOT connected.
                                self.grid[r][c] = self.grid[suspect_r][suspect_c];
                                (self.first_island_size == remaining_island_size + 1)
                                    .then(|| ())
                                    .ok_or(())
                            } else {
                                Ok(())
                            }
                        })
                    })
                    .is_err()
            }
            fn is_weak(&self, r: usize, c: usize) -> bool {
                enum Type {
                    Type0,
                    Type1,
                    Type2I,
                    Type2LSquare,
                    Type2LPureL,
                    Type3T,
                    Type3Square,
                    Type4Cross,
                    Type4Square,
                }
                use Type::*;
                impl Type {
                    fn new(grid: &Vec<Vec<i32>>, r: usize, c: usize) -> Self {
                        let mut connections = 0;
                        let up = r > 0 && c < grid[r - 1].len() && grid[r - 1][c] != 0;
                        let down =
                            r + 1 < grid.len() && c < grid[r + 1].len() && grid[r + 1][c] != 0;
                        let left = c > 0 && grid[r][c - 1] != 0;
                        let right = c + 1 < grid[r].len() && grid[r][c + 1] != 0;
                        let left_down = c > 0
                            && r + 1 < grid.len()
                            && c - 1 < grid[r + 1].len()
                            && grid[r + 1][c - 1] != 0;
                        let right_down = r + 1 < grid.len()
                            && c + 1 < grid[r + 1].len()
                            && grid[r + 1][c + 1] != 0;
                        let right_up =
                            r > 0 && c + 1 < grid[r - 1].len() && grid[r - 1][c + 1] != 0;
                        let left_up =
                            r > 0 && c > 0 && c - 1 < grid[r - 1].len() && grid[r - 1][c - 1] != 0;
                        if up {
                            connections += 1;
                        }
                        if left {
                            connections += 1;
                        }
                        if right {
                            connections += 1;
                        }
                        if down {
                            connections += 1;
                        }
                        match connections {
                            0 => Type0,
                            1 => Type1,
                            2 if up && down || left && right => Type2I,
                            2 if up && right && !right_up
                                || up && left && !left_up
                                || down && right && !right_down
                                || down && left && !left_down =>
                            {
                                Type2LPureL
                            }
                            2 => Type2LSquare,
                            3 if up && right && left && !(right_up && left_up)
                                || down && right && left && !(right_down && left_down)
                                || right && up && down && !(right_up && right_down)
                                || left && up && down && !(left_up && left_down) =>
                            {
                                Type3T
                            }
                            3 => Type3Square,
                            4 if {
                                let mut corners_cnt = 0_usize;
                                if left_up {
                                    corners_cnt += 1;
                                }
                                if right_up {
                                    corners_cnt += 1;
                                }
                                if left_down {
                                    corners_cnt += 1;
                                }
                                if right_down {
                                    corners_cnt += 1;
                                }
                                corners_cnt >= 3
                            } =>
                            {
                                Type4Square
                            }
                            4 => Type4Cross,
                            _ => unreachable!(),
                        }
                    }
                }
                match Type::new(self.grid, r, c) {
                    Type0 | Type1 | Type2LSquare | Type3Square | Type4Square => false,
                    _ => true,
                }
            }
            /// Given grid with content n, count how many maximally connected
            /// neighbors are also n.
            /// When done, all those counted grids would be marked (n+1).
            fn island_size_from(grid: &mut Vec<Vec<i32>>, r: usize, c: usize) -> usize {
                let mut island_size = 0;
                let mut dfs_stack = Vec::new();
                let marker = grid[r][c] + 1;
                grid[r][c] = marker;
                dfs_stack.push((r, c));
                while !dfs_stack.is_empty() {
                    let top = dfs_stack.pop().unwrap();
                    island_size += 1;
                    if top.0 > 0
                        && top.1 < grid[top.0 - 1].len()
                        && grid[top.0 - 1][top.1] != marker
                        && grid[top.0 - 1][top.1] != 0
                    {
                        grid[top.0 - 1][top.1] = marker;
                        dfs_stack.push((top.0 - 1, top.1));
                    }
                    if top.0 + 1 < grid.len()
                        && top.1 < grid[top.0 + 1].len()
                        && grid[top.0 + 1][top.1] != marker
                        && grid[top.0 + 1][top.1] != 0
                    {
                        grid[top.0 + 1][top.1] = marker;
                        dfs_stack.push((top.0 + 1, top.1));
                    }
                    if top.1 > 0 && grid[top.0][top.1 - 1] != marker && grid[top.0][top.1 - 1] != 0
                    {
                        grid[top.0][top.1 - 1] = marker;
                        dfs_stack.push((top.0, top.1 - 1));
                    }
                    if top.1 + 1 < grid[top.0].len()
                        && grid[top.0][top.1 + 1] != marker
                        && grid[top.0][top.1 + 1] != 0
                    {
                        grid[top.0][top.1 + 1] = marker;
                        dfs_stack.push((top.0, top.1 + 1));
                    }
                }
                island_size
            }
            fn new(grid: &'a mut Vec<Vec<i32>>) -> Self {
                let first_island_size = (0..grid.len())
                    .try_for_each(|r| {
                        (0..grid[r].len()).try_for_each(|c| {
                            if grid[r][c] != 0 {
                                Err(Self::island_size_from(grid, r, c))
                            } else {
                                Ok(())
                            }
                        })
                    })
                    .map_or_else(|err| err, |_ok| 0);
                let total_island_size = (0..grid.len())
                    .map(|r| (0..grid[r].len()).filter(|&c| grid[r][c] != 0).count())
                    .sum();
                Self {
                    grid,
                    first_island_size,
                    total_island_size,
                }
            }
        }

        // first, check if obvious solution
        let mut grid = grid;
        let mut helper = WeakStrongHelper::new(&mut grid);
        if helper.total_island_size != helper.first_island_size {
            0
        } else if helper.total_island_size == 0 {
            0
        } else if helper.total_island_size == 1 {
            1
        } else if helper.total_island_size == 2 {
            2
        } else if helper.total_island_size == 3 {
            1
        } else {
            helper.is_one_day().then(|| 1).map_or(2, |i| i)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::min_days(vec![
                vec![1, 1, 0, 1, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 0, 1, 1],
                vec![1, 1, 0, 1, 1]
            ]),
            1
        );
        assert_eq!(
            Solution::min_days(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]]),
            2
        );
        // example of non-zero genus leads to 2-day soln
        assert_eq!(
            Solution::min_days(vec![
                vec![1, 1, 0, 1, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 0, 1, 1],
                vec![1, 1, 1, 1, 1]
            ]),
            2
        );
        assert_eq!(Solution::min_days(vec![vec![1, 1], vec![1, 0]]), 1);
        assert_eq!(
            Solution::min_days(vec![
                vec![1, 1, 1, 1, 0, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 0, 1, 1, 1, 1],
                vec![0, 0, 0, 1, 0, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1]
            ]),
            1
        );
        // example of non-zero genus but 1-day
        assert_eq!(
            Solution::min_days(vec![
                vec![1, 1, 1, 0, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 0, 1, 1, 0],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1]
            ]),
            1
        );
        assert_eq!(
            Solution::min_days(vec![
                vec![1, 1, 0, 1, 1, 1],
                vec![1, 1, 1, 1, 0, 1],
                vec![1, 1, 1, 0, 1, 1],
                vec![1, 1, 0, 1, 1, 1],
                vec![1, 1, 1, 1, 0, 1],
                vec![1, 1, 1, 1, 1, 1]
            ]),
            2
        );
    }
}
