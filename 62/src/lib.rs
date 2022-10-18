pub struct Solution;

/// There is a robot on an m x n grid. The robot is initially located at the
/// top-left corner (i.e., grid[0][0]). The robot tries to move to the
/// bottom-right corner (i.e., grid[m - 1][n - 1]). The robot can only move
/// either down or right at any point in time.
///
/// Upon observation one sees the answer is
/// $f_m(n) = \frac{\prod\limits_{i=1}^{m}{n+i-1}}{m\,!}$
/// Where f_m(n) denotes the number of paths, indexing from 1.
///
/// But since the number are large (up to 2*10^9 by LC spec) we get floating
/// point errors, hence we do it by hand.
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m <= 0 || n <= 0 {
            panic!("Cowardly refuse to answer negative input");
        } else if m == 1 {
            return 1;
        } else if m == 2 {
            return n;
        }
        let mut cache = Vec::with_capacity(n as usize);
        for i in 0..n {
            cache.push(i + 1);
        }
        for _ in 2..m {
            for i in 1..n as usize {
                cache[i] += cache[i - 1];
            }
        }
        cache.last().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
        assert_eq!(Solution::unique_paths(3, 2), 3);
    }
}
