pub struct Solution;

impl Solution {
    /// There is a row of n houses, where each house can be painted one of
    /// three colors: red, blue, or green.
    /// The cost of painting each house with a certain color is different.
    /// You have to paint all the houses such that no two adjacent houses have
    /// the same color.
    ///
    /// The cost of painting each house with a certain color is represented by
    /// an n x 3 cost matrix costs.
    ///
    /// For example, costs[0][0] is the cost of painting house 0
    /// with the color red;
    /// costs[1][2] is the cost of painting house 1 with color green.
    ///
    /// Return the minimum cost to paint all houses.
    // Idea:
    // DP, record the price with additional condition that last house be
    // R, G, or B, then bottom-up build the soln.
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut last_cost_by_color = (0, 0, 0);
        use std::cmp::min;
        costs.iter().for_each(|rbg| {
            // Yes, somehow leetcode choses RBG over RGB
            let r = rbg[0] + min(last_cost_by_color.1, last_cost_by_color.2);
            let b = rbg[1] + min(last_cost_by_color.0, last_cost_by_color.2);
            let g = rbg[2] + min(last_cost_by_color.0, last_cost_by_color.1);
            last_cost_by_color = (r, b, g);
        });
        let (r, b, g) = last_cost_by_color;
        min(r, min(b, g))
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::min_cost(Vec::from([
                vec![17, 2, 17],
                vec![16, 16, 5],
                vec![14, 3, 19]
            ])),
            10
        );
        assert_eq!(Solution::min_cost(Vec::from([vec![7, 6, 2]])), 2);
    }
}
