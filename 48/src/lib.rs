pub struct Solution;

impl Solution {
    /// You are given an n x n 2D matrix representing an image, rotate the
    /// image by 90 degrees (clockwise).
    ///
    /// You have to rotate the image in-place, which means you have to modify
    /// the input 2D matrix directly. DO NOT allocate another 2D matrix and
    /// do the rotation.
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        // A square matrix could be coarsely viewed as, exclude the diagonals,
        // 4 triangles; they does NOT overlap.
        // E.g. Consider 5x5, the triagle is shaped ---, 4 parts.
        //                                           -
        // Together with the crossing diagonals.
        // For each element in the triangle, swap with corresponding triangle.
        // For diagonals do similar.

        // Diagonals
        for a in 0..matrix.len() / 2 {
            let tmp = matrix[a][a];
            let z = matrix.len() - a - 1;
            matrix[a][a] = matrix[z][a];
            matrix[z][a] = matrix[z][z];
            matrix[z][z] = matrix[a][z];
            matrix[a][z] = tmp;
        }
        // Triangles
        for row in 0..matrix.len() / 2 {
            for col in row + 1..matrix.len() - row - 1 {
                let wor = matrix.len() - row - 1;
                let loc = matrix.len() - col - 1;
                let tmp = matrix[row][col];
                matrix[row][col] = matrix[loc][row];
                matrix[loc][row] = matrix[wor][loc];
                matrix[wor][loc] = matrix[col][wor];
                matrix[col][wor] = tmp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_soln() {
        let output = {
            let mut ret = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
            Solution::rotate(&mut ret);
            ret
        };
        let ans = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        assert_eq!(ans, output);
        let output = {
            let mut ret = vec![
                vec![5, 1, 9, 11],
                vec![2, 4, 8, 10],
                vec![13, 3, 6, 7],
                vec![15, 14, 12, 16],
            ];
            Solution::rotate(&mut ret);
            ret
        };
        let ans = vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ];
        assert_eq!(ans, output);
    }
}
