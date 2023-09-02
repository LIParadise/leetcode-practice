pub struct Solution;

impl Solution {
    /// You are painting a fence of n posts with k different colors.
    /// You must paint the posts following these rules:
    ///
    /// Every post must be painted exactly one color.
    /// There cannot be three or more consecutive posts with the same color.
    ///
    /// Given the two integers n and k,
    /// return the number of ways you can paint the fence.
    ///
    /// 1 <= k <= 10^5
    /// 1 <= n <= 50
    // Example:
    // RRG, RGR, RGG, GRR, GRG, GGR (k = 2, n = 3)
    // Idea:
    // T(n, k) determined by T(n-1, k) and T(n-2, k)
    // Case 1: length n terminated with two same
    // Case 2: length n terminated with two different
    //
    // Case 1 could be viewed as being corresponding to those of T(n-2, k)
    // attached with some valid contiguous two.
    //
    // Case 2 corresponds to those of T(n-1, k) attached with one.
    // Notice we can't choose same color as last of T(n-1, k), or else
    // it shall be by definition case 1, contradiction.
    //
    // Hence both cases got multiplier (k-1).
    //
    // The keen eye among you might find this as a linear algebra problem,
    // and it indeed is; the matrix is 2 by 2 since the nature of no more than
    // two consecutive post color, meaning constant-time eigenvalue/eigenvector
    // computation, and the whole problem becomes constant time.
    pub fn num_ways(n: i32, k: i32) -> i32 {
        if k == 1 {
            if n > 2 {
                0
            } else if n == 0 {
                0
            } else {
                1
            }
        } else {
            if n == 1 {
                k
            } else if n == 2 {
                k * k
            } else {
                let mut pprreevv = k;
                let mut prev = k * k;
                (3..=n).for_each(|_| {
                    let p = prev;
                    prev = prev * (k - 1) + pprreevv * (k - 1);
                    pprreevv = p;
                });
                prev
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::num_ways(3, 2), 6);
        assert_eq!(Solution::num_ways(7, 2), 42);
    }
}
