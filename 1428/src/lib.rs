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
        self.matrix[row][col]
    }
    pub fn dimensions(&self) -> Vec<usize> {
        vec![self.matrix[0].len(), self.matrix.len()]
    }
}

impl Solution {
    pub fn left_most_column_with_one(binaryMatrix: &BinaryMatrix) -> i32 {
        if let [rows, cols] = binaryMatrix.dimensions().as_slice() {
            let (mut binary_search_start, mut binary_search_end) = (0, *cols);
            while binary_search_start < binary_search_end {
                let c = (binary_search_start + binary_search_end) / 2;
                match (0..*rows).all(|r| binaryMatrix.get(r, c) == 0) {
                    true => binary_search_start = c + 1,
                    false => binary_search_end = c,
                }
            }
            if binary_search_end == *cols {
                -1
            } else {
                binary_search_end as i32
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
        todo!()
    }
}
