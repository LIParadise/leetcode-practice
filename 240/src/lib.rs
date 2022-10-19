pub struct Solution;

/// Write an efficient algorithm that searches for a value target in an m x n
/// integer matrix matrix. This matrix has the following properties:
///
/// Integers in each row are sorted in ascending from left to right.
/// Integers in each column are sorted in ascending from top to bottom.
impl Solution {
    /// Performs search in 2D matrix with following criteria:
    /// 1. Matrix, i.e. each row equal length
    /// 2. For each row/col, sorted in strictly ascending order
    ///
    /// Time complexity: O(width + height), linear time.
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        // Claim:
        // Since the rows/cols are sorted in ascending order, we could start
        // from `matrix[0].last()` and do the following:
        // 1. If hit, return true
        // 2. Else if too large, move left
        // 3. Else if too small, move down
        // And we're guaranteed to find iff the target is in the matrix.
        //
        // Proof:
        // Note we only discard values in step 2. and step 3.;
        // in step 2. we discard those in the column *larger* than us,
        // in step 3. we discard those in the row *smller* than us.
        // Since we discard only numbers that are not equal to us, and
        // obviously the specified operation runs till either a hit or discard
        // whole matrix, we find the target iff the target is in the matrix.
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return false;
        } else {
            let (mut r, mut c) = (0_usize, matrix[0].len() - 1);
            loop {
                match matrix[r][c].cmp(&target) {
                    std::cmp::Ordering::Equal => return true,
                    std::cmp::Ordering::Less => {
                        if r == matrix.len() - 1 {
                            return false;
                        } else {
                            r += 1;
                        }
                    }
                    std::cmp::Ordering::Greater => {
                        if c == 0 {
                            return false;
                        } else {
                            c -= 1;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::search_matrix(
                vec![
                    vec![1, 4, 7, 11, 15],
                    vec![2, 5, 8, 12, 19],
                    vec![3, 6, 9, 16, 22],
                    vec![10, 13, 14, 17, 24],
                    vec![18, 21, 23, 26, 30]
                ],
                5
            ),
            true
        );
        assert_eq!(
            Solution::search_matrix(
                vec![
                    vec![1, 4, 7, 11, 15],
                    vec![2, 5, 8, 12, 19],
                    vec![3, 6, 9, 16, 22],
                    vec![10, 13, 14, 17, 24],
                    vec![18, 21, 23, 26, 30]
                ],
                20
            ),
            false
        );
    }
}
