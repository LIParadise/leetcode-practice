pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        Self::max_profit_worker(&prices)
    }
    // https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/description/comments/1575155
    pub fn max_profit_worker(prices: &[i32]) -> i32 {
        if prices.len() <= 1 {
            return 0;
        }
        prices
            .iter()
            .skip(1)
            .fold((0, (0, prices[0]), 0), |(free, (cash, stock), cd), &p| {
                // Idea: somewhat close to state machine
                //
                // For each day, there are 3 possible situations:
                // just cooled down with some profit in hand,
                // has some money but part of it is dedicated to buy some stock,
                // and just sold some stock and thus in cooldown.
                //
                // We keep 3 counters for best possible scenario resp. for each of the 3 states:
                //
                // 1. The cooled down state: just follow yesterday's best "in cooldown" case.
                // 2. The (has some money but decided to buy more stock) case,
                //    it depends on the current stock price:
                //    suppose we had cash C, part of which spent on some stock at price S,
                //    the day comes and now that we have cash D and current stock price T,
                //    if (D-T) > (C-S), the best scenario is we buy the stock today.
                //    Why?
                //    Suppose later stock price becomes X, we have net property (D+(X-T))
                //    similarly if we don't buy today, net property is (C+(X-S)).
                //    N.B. one cannot update the first field without considering the second,
                //    since one cannot use today's cash to buy yesterday's stock,
                //    for it breaks causality and the dynamic programming loop invariant.
                // 3. This counter allows for cooldown state, i.e. it may _try_ to sold,
                //    based on yesterday's best possible (has some money but decided to buy more stock) state.
                (
                    cd,
                    if free - p >= cash - stock {
                        (free, p)
                    } else {
                        (cash, stock)
                    },
                    std::cmp::max(cd, cash + (p - stock)),
                )
            })
            .2
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
        assert_eq!(Solution::max_profit(vec![1]), 0);
        assert_eq!(Solution::max_profit(vec![]), 0);
        assert_eq!(Solution::max_profit(vec![4, 3, 2, 10, 11, 0, 11]), 19);
        assert_eq!(Solution::max_profit(vec![1, 2, 4]), 3);
        assert_eq!(Solution::max_profit(vec![6, 1, 3, 2, 4, 7]), 6);
        assert_eq!(Solution::max_profit(vec![1, 3, 4, 3, 6]), 5);
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

