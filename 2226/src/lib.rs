pub struct Solution;

impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let mut candies = candies;
        candies.sort_unstable_by(|a, b| b.cmp(a));
        let max_possible = candies.iter().map(|&i| i as i64).sum::<i64>() / k;
        if max_possible <= 1 {
            return max_possible as i32;
        } else {
            // Binary search possible answers
            let mut end = (max_possible + 1) as i32;
            let mut start = 1_i32;
            while end - start > 1 {
                let try_give_candies = (end + start) / 2;
                let available_piles = candies
                    .get(0..candies.partition_point(|c| c >= &&try_give_candies))
                    .unwrap()
                    .iter()
                    .fold(0_i64, |accm, c| accm + (c / try_give_candies) as i64);
                if available_piles < k {
                    end = try_give_candies;
                } else {
                    start = try_give_candies;
                }
            }
            start as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::maximum_candies(vec![5, 8, 6], 3), 5);
        assert_eq!(Solution::maximum_candies(vec![2, 5], 11), 0);
        assert_eq!(Solution::maximum_candies(vec![2, 5], 11), 0);
    }
}
