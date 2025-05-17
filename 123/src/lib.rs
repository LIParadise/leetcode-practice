pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 1 {
            0
        } else {
            Self::max_profit_worker(&prices)
        }
    }

    // https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/solutions/796990/c-worst-to-best-solution-explained-for-dummies-like-me
    //
    // Suppose you make some profit p1 by doing your first transaction in the stock market.
    // Now you are excited to purchase another stock to earn more profit.
    // Suppose the price of the second stock you aim to buy is x.
    // Now, for you,
    // the net effective price that you are spending from your pocket for this stock will be x-p1,
    // because you already have p1 bucks in your hand.
    // Now, if you sell the second stock at price y your net profit p2 will be p2 = y - (x-p1).
    // You have to do nothing but maximize this profit p2.
    //
    // E.g. consider              [3, 3, 5, 0, 0, 3, 1, 4]
    //        min hold cost        3  3  3  0  0  0  0  0
    //        max one-shot profit  0  0  2  2  2  3  3  4
    //                                   |              |
    //                                   |              \-> min hold cost was 0, price 4, max one-shot profit is thus 4
    //                                   \-> min hold cost was 3, price 5, max one-shot profit is thus 2
    //
    // The gist is to consider "what's the effective price, considering we've made some profit"?
    //
    // prices                                       [3, 3, 5, 0, 0, 3, 1, 4]
    // min hold cost                                 3  3  3  0  0  0  0  0
    // max one-shot profit                           0  0  2  2  2  3  3  4
    // eff. price considering we've made some profit 3  3  3 -2 -2  0 -2  0
    // min hold cost                                 3  3  3 -2 -2 -2 -2 -2
    // max profit                                    0  0  0  0  0  2  2  2
    //
    // In essence, solve for (one-shot transaction over array), twice!
    fn max_profit_worker(prices: &[i32]) -> i32 {
        prices
            .iter()
            .skip(1)
            .fold(
                (*prices.first().unwrap(), 0, *prices.first().unwrap(), 0),
                |(mut cost_0, mut profit_0, mut cost_1, mut profit_0_and_1), price| {
                    cost_0 = std::cmp::min(cost_0, *price);
                    profit_0 = std::cmp::max(profit_0, *price - cost_0);
                    let eff_price_if_hold = *price - profit_0;
                    cost_1 = std::cmp::min(cost_1, eff_price_if_hold);
                    profit_0_and_1 = std::cmp::max(profit_0_and_1, *price - cost_1);
                    (cost_0, profit_0, cost_1, profit_0_and_1)
                },
            )
            .3
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![]), 0);
        assert_eq!(Solution::max_profit(vec![1]), 0);
        assert_eq!(Solution::max_profit(vec![1, 1]), 0);
        assert_eq!(Solution::max_profit(vec![1, 2]), 1);
        assert_eq!(Solution::max_profit(vec![2, 1]), 0);
        assert_eq!(Solution::max_profit(vec![1, 2, 3]), 2);
        assert_eq!(Solution::max_profit(vec![1, 2, 2]), 1);
        assert_eq!(Solution::max_profit(vec![1, 2, 1]), 1);
        assert_eq!(Solution::max_profit(vec![2, 2, 1]), 0);
    }
}
