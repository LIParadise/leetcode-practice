pub struct Solution;

impl Solution {
    /// You are climbing a staircase. It takes n steps to reach the top.
    /// Each time you can either climb 1 or 2 steps. In how many distinct ways
    /// can you climb to the top?
    ///
    /// Examples:
    /// ```
    /// use lc_70_climbing_stairs::Solution as S;
    /// assert_eq!(S::climb_stairs(2), 2);
    /// assert_eq!(S::climb_stairs(3), 3);
    /// assert_eq!(S::climb_stairs(4), 5);
    /// assert_eq!(S::climb_stairs(5), 8);
    /// ```
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 3 {
            n
        } else {
            let mut even = 2;
            let mut odd = 3;
            (4..=n).for_each(|i| {
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
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        todo!()
    }
}
