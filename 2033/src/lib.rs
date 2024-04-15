pub struct Solution;

impl Solution {
    // Lemma: try median
    // Proof:
    // WLOG supp. non-decreasing order and total number is odd
    // i.e. n_0 <= n_1 <= ... <= n_m where m is even
    // consider (i < median = m/2)
    // note that cost(i) > cost(i+1): supp. change from i to i+1,
    // there are (i+1) elements now cost more,
    // while there are (m-i) elements now cost less,
    // the gist is that delta of each individual element equal,
    // and since both i and m are integers,
    // we have $\frac{i}{2} < m \implies 2\times i+1 < m$
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut arr = grid.into_iter().flatten().collect::<Vec<_>>();
        arr.sort_unstable();
        let median = if arr.len() % 2 == 1 {
            arr[arr.len() / 2]
        } else {
            (arr[arr.len() / 2] + arr[arr.len() / 2]) / 2
        };
        if arr
            .iter()
            .zip(arr.iter().skip(1))
            .all(|(a, b)| a % x == b % x)
        {
            arr.into_iter()
                .fold(0, |ret, y| ret + i32::abs(y - median) / x)
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::min_operations(vec![vec![2, 4], vec![6, 8]], 2), 4);
        assert_eq!(Solution::min_operations(vec![vec![1, 5], vec![2, 3]], 1), 5);
        assert_eq!(
            Solution::min_operations(vec![vec![1, 2], vec![3, 4]], 2),
            -1
        );
    }
}
