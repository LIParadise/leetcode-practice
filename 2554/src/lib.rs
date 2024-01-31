pub struct Solution;

impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let mut banned = banned;
        banned.sort_unstable();
        let mut cnt = 0;
        let mut search_start = 0;
        let mut thres = max_sum;
        let _ = (1..=n).try_for_each(|i| {
            if i > thres {
                Err(())
            } else {
                if let Ok(idx) = banned[search_start..].binary_search(&i) {
                    search_start = idx + 1;
                } else {
                    thres -= i;
                    cnt += 1;
                }
                Ok(())
            }
        });
        cnt
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(Solution::max_count(vec![1, 6, 5], 5, 6), 2);
        assert_eq!(Solution::max_count(vec![1, 2, 3, 4, 5, 6, 7], 8, 1), 0);
        assert_eq!(Solution::max_count(vec![11], 7, 50), 7);
    }
}
