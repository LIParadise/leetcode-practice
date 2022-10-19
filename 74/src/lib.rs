pub struct Solution;

/// Write an efficient algorithm that searches for a value target in an m x n
/// integer matrix matrix. This matrix has the following properties:
///
/// Integers in each row are sorted from left to right.
/// The first integer of each row is greater than the last integer of the
/// previous row.
impl Solution {
    // Assumes if flatten the sequence would be strictly ascending order.
    // Complexity: O(#rows + lg(#cols))
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return false;
        } else {
            matrix.iter().any(|row| {
                match (
                    target.cmp(row.first().unwrap()),
                    target.cmp(row.last().unwrap()),
                ) {
                    (std::cmp::Ordering::Equal, _) => true,
                    (_, std::cmp::Ordering::Equal) => true,
                    (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => {
                        let idx = row.partition_point(|num| num < &target);
                        return row[idx] == target;
                    }
                    _ => false,
                }
            })
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
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                3
            ),
            true
        );
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                13
            ),
            false
        );
    }
}
