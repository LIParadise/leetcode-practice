pub struct Solution;

// This is the BinaryMatrix's API interface.
// You should not implement it, or speculate about its implementation
pub struct BinaryMatrix {
    matrix: Vec<Vec<i32>>,
}
impl BinaryMatrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        match matrix.get(0) {
            None => panic!(),
            Some(row) => match row.is_empty() {
                true => panic!(),
                false => {
                    let len = row.len();
                    assert!(matrix.iter().all(|vec| vec.len() == len));
                }
            },
        }
        Self { matrix }
    }
    pub fn get(&self, row: usize, col: usize) -> i32 {
        *self.matrix.get(row).unwrap().get(col).unwrap()
    }
    pub fn dimensions(&self) -> Vec<usize> {
        vec![self.matrix.len(), self.matrix[0].len()]
    }
}

impl Solution {
    pub fn left_most_column_with_one(binaryMatrix: &BinaryMatrix) -> i32 {
        if let &[rows, cols] = binaryMatrix.dimensions().as_slice() {
            let mut coordinate = (0, cols - 1);
            while coordinate.0 < rows && coordinate.1 < cols {
                match binaryMatrix.get(coordinate.0, coordinate.1) {
                    0 => coordinate.0 += 1,
                    1 => {
                        // Weird TODO below
                        if coordinate.1 == 0 {
                            // This if clause should not be required,
                            // but LC insists I add this line...
                            return 0;
                        }
                        // Weird TODO above
                        coordinate.1 = coordinate.1.wrapping_sub(1);
                    }
                    n => panic!("Invalid matrix element {n}"),
                }
            }
            if coordinate.1 == cols - 1 {
                -1
            } else {
                coordinate.1.wrapping_add(1) as i32
            }
        } else {
            panic!("Dimension not working as expected")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            0,
            Solution::left_most_column_with_one(&crate::BinaryMatrix::new(vec![
                vec![0, 0],
                vec![1, 1]
            ]))
        );
        assert_eq!(
            0,
            Solution::left_most_column_with_one(&crate::BinaryMatrix::new(vec![
                vec![1, 1],
                vec![0, 0]
            ]))
        );
        assert_eq!(
            1,
            Solution::left_most_column_with_one(&crate::BinaryMatrix::new(vec![
                vec![0, 0],
                vec![0, 1]
            ]))
        );
        assert_eq!(
            -1,
            Solution::left_most_column_with_one(&crate::BinaryMatrix::new(vec![
                vec![0, 0],
                vec![0, 0]
            ]))
        );
        assert_eq!(
            1,
            Solution::left_most_column_with_one(&crate::BinaryMatrix::new(vec![
                vec![0, 0, 0],
                vec![0, 1, 1],
                vec![0, 0, 1]
            ]))
        );
        assert_eq!(
            2,
            Solution::left_most_column_with_one(&crate::BinaryMatrix::new(vec![
                vec![0, 0, 0],
                vec![0, 0, 1],
                vec![0, 0, 1]
            ]))
        );
        assert_eq!(
            0,
            Solution::left_most_column_with_one(&crate::BinaryMatrix::new(vec![
                vec![1, 1, 1],
                vec![0, 0, 1],
                vec![0, 0, 1]
            ]))
        );
    }
}
