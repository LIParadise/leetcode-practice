pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        Self::max_profit_worker(&prices)
    }
    // Note that the profit is equivalently to asking the max member after this index
    fn max_profit_worker(prices: &[i32]) -> i32 {
        if prices.is_empty() {
            0
        } else {
            prices
                .iter()
                .rev()
                .skip(1)
                .scan(*prices.last().unwrap(), |max_of_later_entries, current| {
                    let profit = std::cmp::max(0, *max_of_later_entries - *current);
                    *max_of_later_entries = std::cmp::max(*max_of_later_entries, *current);
                    Some(profit)
                })
                .max()
                .unwrap_or(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
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
