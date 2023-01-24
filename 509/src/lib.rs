pub struct Solution;

impl Solution {
    /// Fibonacci number
    /// 0th = 0
    /// 1st = 1
    /// 2nd = 1
    /// 3rd = 2
    pub fn fib(n: i32) -> i32 {
        let mut even = 0;
        let mut odd = 1;
        (2..=n).for_each(|i| {
            if i % 2 == 0 {
                even += odd;
            } else {
                odd += even;
            }
        });
        if n % 2 == 0 {
            even
        } else {
            odd
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::fib(0), 0);
        assert_eq!(Solution::fib(1), 1);
        assert_eq!(Solution::fib(2), 1);
        assert_eq!(Solution::fib(3), 2);
        assert_eq!(Solution::fib(4), 3);
        assert_eq!(Solution::fib(5), 5);
        assert_eq!(Solution::fib(6), 8);
        assert_eq!(Solution::fib(7), 13);
    }
}
