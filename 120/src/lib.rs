pub struct Solution;

impl Solution {
    /// Given a triangle array, return the minimum path sum from top to bottom.
    /// For each step, you may move to an adjacent number of the row below.
    /// More formally, if you are on index i on the current row, you may move
    /// to either index i or index i + 1 on the next row.
    /// ```
    /// use lc_120_triangle::*;
    /// let triangle = vec![vec![2],vec![3,4],vec![6,5,7],vec![4,1,8,3]];
    /// assert_eq!(Solution::minimum_total(triangle), 11);
    /// //    2
    /// //   3 4
    /// //  6 5 7
    /// // 4 1 8 3
    /// // Since 2 + 3 + 5 + 1 = 11, the answer is 11.
    /// let triangle = vec![vec![-10]];
    /// assert_eq!(Solution::minimum_total(triangle), -10);
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        // assume the lengths are valid,
        // i.e. first row is of length 1,
        // then row by row length is increased by 1.
        //
        // DP comes to rescue.
        // Consider the following:
        //    n
        //   A B
        // where A and B are triangles of equal size and n is some number.
        // Then the whole triangle starting at n have a minimum sum path via
        // whichever in A or B is smaller.
        //
        // We build the DP bottom-up, then row by row upwards, we consider the
        // sub-triangles.
        let mut dp_ans = triangle.last().unwrap().clone();
        for row in triangle.iter().rev().skip(1) {
            row.iter().enumerate().for_each(|(idx, num)| {
                let (left, right) = dp_ans.split_at_mut(idx + 1);
                *left.last_mut().unwrap() =
                    std::cmp::min(left.last().unwrap(), right.first().unwrap()) + num
            });
        }
        dp_ans.first().unwrap().clone()
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
