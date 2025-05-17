pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 1 {
            0
        } else {
            Self::max_profit_worker(&prices)
        }
    }

    // Idea:
    // cut the array into two: find some index,
    // one subarray start at 0 and ends before that index,
    // one subarray start at that index and ends at array end,
    // each of which itself asking the best one-shot strategy in resp. subarray,
    // both of which may be calculated in linear time.
    //
    // assuming array length greater than 1
    fn max_profit_worker(prices: &[i32]) -> i32 {
        let sub_array_profit_from_front = prices
            .iter()
            .scan((0, *prices.first().unwrap()), |(profit, min), price| {
                *profit = std::cmp::max(*price - *min, *profit);
                *min = std::cmp::min(*price, *min);
                Some((*profit, *min))
            })
            .map(|(profit, _)| profit);

        // note this is reversed!
        let sub_array_profit_till_end = prices
            .iter()
            .rev()
            .scan((0, *prices.last().unwrap()), |(profit, max), price| {
                *profit = std::cmp::max(*max - *price, *profit);
                *max = std::cmp::max(*price, *max);
                Some((*profit, *max))
            })
            .map(|(profit, _)| profit)
            .collect::<Vec<_>>();

        std::iter::once(0)
            .chain(sub_array_profit_from_front)
            .zip(
                sub_array_profit_till_end
                    .into_iter()
                    // `Iterator::rev`, for the memoization is reversed
                    .rev()
                    .chain(std::iter::once(0)),
            )
            .map(|(profit_front, profit_back)| profit_front + profit_back)
            .max()
            .unwrap()
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
