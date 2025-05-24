pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() || matrix[0].is_empty() {
            vec![]
        } else {
            #[derive(Clone)]
            struct Position {
                row: usize,
                row_start: usize,
                row_end: usize,
                col: usize,
                col_start: usize,
                col_end: usize,
            }
            #[derive(Clone)]
            enum Directions {
                Left(Position),
                Right(Position),
                Up(Position),
                Down(Position),
            }
            impl Directions {
                fn step(self) -> Option<Self> {
                    match self {
                        Self::Right(
                            s @ Position {
                                row,
                                row_start,
                                row_end,
                                col,
                                col_end,
                                ..
                            },
                        ) => {
                            if col + 1 < col_end {
                                Some(Self::Right(Position { col: col + 1, ..s }))
                            } else if row_start + 1 < row_end {
                                Some(Self::Down(Position {
                                    row: row + 1,
                                    row_start: row_start + 1,
                                    ..s
                                }))
                            } else {
                                None
                            }
                        }
                        Self::Down(
                            s @ Position {
                                row,
                                row_end,
                                col,
                                col_start,
                                col_end,
                                ..
                            },
                        ) => {
                            if row + 1 < row_end {
                                Some(Self::Down(Position { row: row + 1, ..s }))
                            } else if col_start + 1 < col_end {
                                Some(Self::Left(Position {
                                    col: col - 1,
                                    col_end: col_end - 1,
                                    ..s
                                }))
                            } else {
                                None
                            }
                        }
                        Self::Left(
                            s @ Position {
                                row,
                                row_start,
                                row_end,
                                col,
                                col_start,
                                ..
                            },
                        ) => {
                            if col > col_start {
                                Some(Self::Left(Position { col: col - 1, ..s }))
                            } else if row_start + 1 < row_end {
                                Some(Self::Up(Position {
                                    row: row - 1,
                                    row_end: row_end - 1,
                                    ..s
                                }))
                            } else {
                                None
                            }
                        }
                        Self::Up(
                            s @ Position {
                                row,
                                row_start,
                                col,
                                col_start,
                                col_end,
                                ..
                            },
                        ) => {
                            if row > row_start {
                                Some(Self::Up(Position { row: row - 1, ..s }))
                            } else if col_start + 1 < col_end {
                                Some(Self::Right(Position {
                                    col: col + 1,
                                    col_start: col_start + 1,
                                    ..s
                                }))
                            } else {
                                None
                            }
                        }
                    }
                }
            }
            (0..)
                .scan(
                    Some(Directions::Right(Position {
                        row: 0,
                        col: 0,
                        row_start: 0,
                        col_start: 0,
                        row_end: matrix.len(),
                        col_end: matrix[0].len(),
                    })),
                    |state, _| {
                        let ret = state.take();
                        *state = ret.clone().and_then(|x| x.step());
                        ret
                    },
                )
                .map(|d| match d {
                    Directions::Right(Position { row, col, .. })
                    | Directions::Down(Position { row, col, .. })
                    | Directions::Left(Position { row, col, .. })
                    | Directions::Up(Position { row, col, .. }) => matrix[row][col],
                })
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
        assert_eq!(
            Solution::spiral_order(vec![vec![1], vec![2,], vec![3]]),
            vec![1, 2, 3,]
        );
        assert_eq!(Solution::spiral_order(vec![vec![0]]), vec![0]);
        assert_eq!(Solution::spiral_order(vec![vec![]]), vec![]);
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

