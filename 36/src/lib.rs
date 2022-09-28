struct Solution;

struct OneToNineChecklist {
    list: Vec<u8>,
}
impl OneToNineChecklist {
    fn new() -> Self {
        Self { list: vec![0; 9] }
    }
    fn reset(&mut self) {
        self.list.iter_mut().for_each(|b| *b = 0);
    }
    #[inline(always)]
    fn valid_sudoku_char(c: char) -> Result<Option<usize>, ()> {
        let dbg = match c {
            '0' => Err(()),
            '.' => Ok(None),
            ascii_num_char if c.is_ascii_digit() => {
                let dbg = Ok(Some(ascii_num_char.to_digit(10).unwrap() as usize - 1));
                return dbg;
            }
            _ => Err(()),
        };
        return dbg;
    }
    fn check(&mut self, c: char) -> Result<(), ()> {
        if let Ok(option_idx) = Self::valid_sudoku_char(c) {
            if let Some(idx) = option_idx {
                let dbg = self.list.get_mut(idx).ok_or(());
                *self.list.get_mut(idx).ok_or(())? += 1;
            }
            Ok(())
        } else {
            Err(())
        }
    }
    fn is_valid(&self) -> bool {
        self.list.iter().all(|u| u <= &1_u8)
    }
}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut check_list = OneToNineChecklist::new();
        if let true = Self::check_rows(&board, &mut check_list) {
            if let true = Self::check_cols(&board, &mut check_list) {
                let ugly_ranges = vec![(0..3), (3..6), (6..9)];
                for ugly_row_range in ugly_ranges.clone() {
                    for ugly_col_range in ugly_ranges.clone() {
                        match Self::check_blocks(
                            &board,
                            &mut check_list,
                            (ugly_row_range.clone(), ugly_col_range),
                        ) {
                            Some(true) => {}
                            _ => {
                                return false;
                            }
                        }
                    }
                }
                return true;
            }
        }
        false
    }
    /// Assume valid dimensions
    fn check_rows(board: &Vec<Vec<char>>, checklist: &mut OneToNineChecklist) -> bool {
        for row in board.iter() {
            checklist.reset();
            if let Err(_) = row
                .iter()
                .try_for_each(|ch| checklist.check(ch.clone()))
            {
                return false;
            }
            if !checklist.is_valid() {
                return false;
            }
        }
        true
    }
    /// Assume valid dimensions
    fn check_cols(board: &Vec<Vec<char>>, check_list: &mut OneToNineChecklist) -> bool {
        for col_idx in 0_usize..=8 {
            check_list.reset();
            if let None = board
                .iter()
                .try_for_each(|row| check_list.check(row.get(col_idx)?.clone()).ok())
            {
                return false;
            }
            if !check_list.is_valid() {
                return false;
            }
        }
        true
    }
    fn check_blocks(
        board: &Vec<Vec<char>>,
        check_list: &mut OneToNineChecklist,
        indices: (std::ops::Range<usize>, std::ops::Range<usize>),
    ) -> Option<bool> {
        check_list.reset();
        for row in board.get(indices.0)? {
            for col in row.get(indices.1.clone())? {
                check_list.check(col.clone()).ok()?;
            }
        }
        Some(check_list.is_valid())
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_soln() {
        let sudoku = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(true, Solution::is_valid_sudoku(sudoku));
        let sudoku = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(false, Solution::is_valid_sudoku(sudoku));
    }
}
