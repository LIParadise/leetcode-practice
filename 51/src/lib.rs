pub struct Solution;

pub mod permutation {
    /// Assumes all elements are distinct, try permute them s.t. results
    /// in next dictionary order.
    /// If it's already last dictionary order i.e. largest, returns false
    /// and input is not modified.
    /// ```
    /// use lc_51_n__queens::permutation;
    /// let mut input = [1, 3, 4, 6, 5, 2];
    /// let ans = [1, 3, 5, 2, 4, 6];
    /// assert!(permutation::next_perm(&mut input));
    /// assert_eq!(input, ans);
    /// let mut input = [3, 2, 1];
    /// let ans = [3, 2, 1];
    /// assert!(!permutation::next_perm(&mut input));
    /// assert_eq!(input, ans);
    /// let mut input = [1, 3, 2];
    /// let ans = [2, 1, 3];
    /// assert!(permutation::next_perm(&mut input));
    /// assert_eq!(input, ans);
    /// ```
    // Consider [1, 3, 4, 6, 5, 2]
    // what's next dictionary order? [1, 3, 5, 2, 4, 6]
    // why?
    // notice that subarr [6, 5, 2] is in maximal config in dictionary order,
    // i.e. its next dictionary permutation require a *carry* over 4
    // how? in [6, 5, 2], 5 is larger than 4, so that's it!
    // after carry it's [4, 6, 5, 2], concate with remaining part we get
    // [1, 3, 5, 2, 4, 6] as desired.
    //
    // What if there's no element large enough?
    // That means the whole array is decreasing, which signifies end of
    // dictionary order, and we don't need to do anything.
    pub fn next_perm(vec: &mut [u8]) -> bool {
        let shall_carry = vec
            .iter()
            .rev()
            .zip(vec.iter().rev().skip(1))
            .position(|(r, l)| l < r);
        match shall_carry {
            None => false,
            Some(shall_carry) => {
                let (lhs, rhs) = vec.split_at_mut(vec.len() - 1 - shall_carry);
                let idx = rhs.partition_point(|r| r > lhs.last().unwrap()) - 1;
                my_swap(lhs.last_mut().unwrap(), rhs.get_mut(idx).unwrap());
                rhs.reverse();
                true
            }
        }
    }
    #[inline]
    fn my_swap<T: Clone>(i: &mut T, j: &mut T) {
        let tmp = i.clone();
        *i = j.clone();
        *j = tmp.clone();
    }
}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        if n > u8::MAX as usize {
            panic!("Really? N-Queen problem where N larger than {}?", u8::MAX);
        } else if n == 1 {
            vec![vec!["Q".to_string()]]
        } else if n == 2 || n == 3 {
            Vec::new()
        } else {
            // Notice that a valid N-Queen must be a permutation of [0..N]
            // What? You might say.
            // It's collecting the col idx of each row of an array,
            // e.g. [1, 3, 0, 2] is _ Q _ _
            //                      _ _ _ Q
            //                      Q _ _ _
            //                      _ _ Q _ i.e. a soln to 4-Queen
            // Related: [LC 31](https://leetcode.com/problems/next-permutation/)
            //
            // By nature of permutation, checking diagonoals is enough, no need
            // to check if two Queens on same row/col.
            let mut queen_cols = Vec::with_capacity(n);
            (0..n as u8).for_each(|i| queen_cols.push(i));
            let mut ret = Vec::new();
            while {
                if Self::check_diagonals(&queen_cols) {
                    ret.push(Self::make_board(&queen_cols));
                }
                permutation::next_perm(&mut queen_cols)
            } {}
            ret
        }
    }

    /// Input: col idx of queen of each row
    /// E.g. [1, 3, 0, 2] is a soln to 4-Queen
    /// Since _ Q _ _
    ///       _ _ _ Q
    ///       Q _ _ _
    ///       _ _ Q _ is a solution.
    /// Check if all Queens are not on diagonoals of each other.
    ///
    /// Yes this fn dosn't check if they are on same row.
    #[inline]
    fn check_diagonals(vec: &[u8]) -> bool {
        let ret = vec.iter().enumerate().all(|(row, col)| {
            vec.iter().enumerate().all(|(r, c)| {
                if r == row {
                    true
                } else {
                    let on_backslash =
                        (r as u8).wrapping_sub(row as u8) == c.wrapping_sub(*col as u8);
                    let on_slash = (row as u8).wrapping_sub(r as u8) == c.wrapping_sub(*col as u8);
                    !on_slash && !on_backslash
                }
            })
        });
        return ret;
    }
    fn make_board(vec: &[u8]) -> Vec<String> {
        vec.iter()
            .map(|&col| {
                let mut row = Vec::with_capacity(vec.len());
                while row.len() < vec.len() {
                    row.push('.' as u8);
                }
                row[col as usize] = 'Q' as u8;
                String::from_utf8(row).unwrap()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use std::collections::BTreeSet as BTS;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::solve_n_queens(4).into_iter().collect::<BTS<_>>(),
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."]
            ]
            .into_iter()
            .map(|vec_of_str| vec_of_str.into_iter().map(|s| s.to_string()).collect())
            .collect::<BTS<_>>()
        );
        assert_eq!(
            Solution::solve_n_queens(1).into_iter().collect::<BTS<_>>(),
            vec![vec!["Q"]]
                .into_iter()
                .map(|vec_of_str| vec_of_str.into_iter().map(|s| s.to_string()).collect())
                .collect::<BTS<_>>()
        );
    }
    #[test]
    fn test_check() {
        assert_eq!(Solution::check_diagonals(&vec![1, 3, 2, 0]), false);
    }
}
