pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let ret = nums
            .iter()
            .enumerate()
            .fold([None; 2], |mut tgt_range, (idx, num)| {
                if tgt_range[0].is_none() {
                    if target == *num {
                        tgt_range[0] = Some(idx);
                    }
                } else {
                    if tgt_range[1].is_none() {
                        if target != *num {
                            tgt_range[1] = Some(idx - 1);
                        }
                    }
                }
                tgt_range
            });
        match ret {
            [Some(start), Some(end)] => vec![start as i32, end as i32],
            [Some(start), None] => vec![start as i32, (nums.len() - 1) as i32],
            _ => vec![-1, -1],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
    }
}
