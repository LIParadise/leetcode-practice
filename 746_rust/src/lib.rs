pub struct Solution;

impl Solution {
    /// You are given an integer array cost where cost[i] is the cost of ith
    /// step on a staircase. Once you pay the cost, you can either climb one
    /// or two steps.
    ///
    /// You can either start from the step with index 0, or the step with index 1.
    ///
    /// Return the minimum cost to reach the top of the floor.
    ///
    /// Examples:
    /// ```
    /// use lc_746_min_cost_climbing_stairs::Solution as S;
    /// assert_eq!(S::min_cost_climbing_stairs(Vec::from(
    ///     [10, 15, 20])), 15);
    /// //       ^
    /// assert_eq!(S::min_cost_climbing_stairs(Vec::from(
    ///     [1, 100, 1, 1, 1, 100, 1, 1, 100, 1])), 6);
    /// //   ^       ^     ^       ^  ^       ^
    /// ```
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        if cost.len() <= 1 {
            0
        } else {
            let mut cost = cost;
            (2..cost.len()).for_each(|idx| {
                let (prev, after) = cost.split_at_mut(idx);
                after[0] += std::cmp::min(prev[prev.len() - 1], prev[prev.len() - 2]);
            });
            std::cmp::min(cost[cost.len() - 1], cost[cost.len() - 2])
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
