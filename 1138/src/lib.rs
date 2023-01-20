pub struct Solution;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate {
    x: usize,
    y: usize,
}

trait FuckLeetcode {
    type T: Ord + std::ops::Sub<Output = Self::T>;
    fn abs_diff(x: Self::T, y: Self::T) -> Self::T {
        if x < y {
            y - x
        } else {
            x - y
        }
    }
}
impl FuckLeetcode for usize {
    type T = usize;
}

impl Coordinate {
    #[inline]
    pub fn new(c: char) -> Self {
        Self {
            x: ((c as u8 as usize) - ('a' as u8 as usize)) % 5,
            y: ((c as u8 as usize) - ('a' as u8 as usize)) / 5,
        }
    }
    pub fn clone_with_x_be_0(&self) -> Self {
        Self { x: 0, y: self.y }
    }
    // Additional care is required since board ain't rectangular.
    // If rectangular, we could just do y difference, then x difference,
    // then call it a day.
    //
    // BUT 'z' protrudes out from the 5 by 5 'a'..'y'.
    //
    // If destination were 'z', one should move to first column
    // (x coordinate is 0) first.
    pub fn record_difference(begin: Self, end: Self, record: &mut Vec<u8>) {
        let mid = if end == Self::new('z') {
            begin.clone_with_x_be_0()
        } else {
            end
        };
        for _ in 0..<usize as FuckLeetcode>::abs_diff(begin.y, mid.y) {
            // for _ in 0..begin.y.abs_diff(end.y) {
            record.push(match begin.y.cmp(&mid.y) {
                std::cmp::Ordering::Greater => 'U',
                std::cmp::Ordering::Less => 'D',
                _ => panic!("Unexpected range spec"),
            } as u8);
        }
        for _ in 0..<usize as FuckLeetcode>::abs_diff(begin.x, mid.x) {
            // for _ in 0..begin.x.abs_diff(end.x) {
            record.push(match begin.x.cmp(&mid.x) {
                std::cmp::Ordering::Greater => 'L',
                std::cmp::Ordering::Less => 'R',
                _ => panic!("Unexpected range spec"),
            } as u8);
        }
        for _ in 0..<usize as FuckLeetcode>::abs_diff(mid.y, end.y) {
            // for _ in 0..begin.y.abs_diff(end.y) {
            record.push(match mid.y.cmp(&end.y) {
                std::cmp::Ordering::Greater => 'U',
                std::cmp::Ordering::Less => 'D',
                _ => panic!("Unexpected range spec"),
            } as u8);
        }
        record.push('!' as u8);
    }
}

impl Solution {
    /// On an alphabet board, we start at position (0, 0), corresponding to
    /// character board[0][0].
    /// Here, board = ["abcde", "fghij", "klmno", "pqrst", "uvwxy", "z"],
    /// as shown in the diagram below.
    /// Return a sequence of moves ("DRLU!") that makes our answer equal
    /// to target in the minimum number of moves.
    /// 'D'/'R'/'L'/'U'/'!' correspond to resp. down/right/left/up/add.
    /// You may return any path that does so.
    /// ```
    /// use lc_1138_alphabet_board_path::Solution as soln;
    /// let tgt = "leet";
    /// assert_eq!("DDR!UURRR!!DDD!",
    ///     soln::alphabet_board_path(tgt.to_string()).as_str());
    /// ```
    // Since board is given as fixed, use inline function to translate.
    pub fn alphabet_board_path(target: String) -> String {
        let mut ret = Vec::new();
        let mut last_coordinate = Coordinate::new('a');
        target.as_str().chars().for_each(|c| {
            let new_coordinate = Coordinate::new(c);
            Coordinate::record_difference(last_coordinate, new_coordinate, &mut ret);
            last_coordinate = new_coordinate;
        });
        String::from_utf8(ret).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        todo!()
    }
}
