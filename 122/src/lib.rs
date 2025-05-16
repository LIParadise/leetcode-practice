pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        Self::max_profit_worker(&prices)
    }
    // idea: buy low, sell high
    // specifically, capture all the increasing sub-intervals
    fn max_profit_worker(prices: &[i32]) -> i32 {
        let mut iter = prices.iter().zip(prices.iter().skip(1)).peekable();
        let mut profit = 0;
        let mut bought_at;
        loop {
            'buy_at_low: loop {
                match iter.peek().map(|(a, b)| a.cmp(b)) {
                    None => return profit,
                    Some(std::cmp::Ordering::Less) => {
                        bought_at = *iter.next().unwrap().0;
                        break 'buy_at_low;
                    }
                    _ => _ = iter.next(),
                }
            }
            'sell_at_high: loop {
                match iter.peek().map(|(a, b)| a.cmp(b)) {
                    None => return profit + (*prices.last().unwrap() - bought_at),
                    Some(std::cmp::Ordering::Greater) => {
                        profit += *iter.next().unwrap().0 - bought_at;
                        break 'sell_at_high;
                    }
                    _ => _ = iter.next(),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit(vec![2, 2, 2, 2, 2]), 0);
        assert_eq!(Solution::max_profit(vec![]), 0);
        assert_eq!(Solution::max_profit(vec![1]), 0);
        assert_eq!(Solution::max_profit(vec![1, 2]), 1);
        assert_eq!(Solution::max_profit(vec![2, 1]), 0);
    }
}

macro_rules! lprintln {
    // Match when a format string and additional arguments are provided
    ($fmt:expr, $($arg:tt)*) => {{
        if cfg!(feature = "local_test") {
            println!($fmt, $($arg)*);
        }
    }};

    // Match when only a format string is provided
    ($fmt:expr) => {{
        if cfg!(feature = "local_test") {
            println!($fmt);
        }
    }};
}

