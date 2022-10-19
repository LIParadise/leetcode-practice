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
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return false;
        } else {
            let mut search_end = matrix[0].len();
            for row in matrix.iter() {
                let candidate_idx = row
                    .get(0..search_end)
                    .unwrap()
                    .partition_point(|num| num < &target);
                if let Some(candidate) = row.get(candidate_idx) {
                    if candidate == &target {
                        return true;
                    }
                } else {
                    search_end = candidate_idx;
                }
            }
            return false;
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
