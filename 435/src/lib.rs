pub struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable();
        (intervals.len()
            - intervals
                .iter()
                .scan(None, |last_interval, interval| match last_interval {
                    None => {
                        *last_interval = Some(interval.as_slice());
                        Some(1)
                    }
                    Some(l) => {
                        if interval[0] >= l[1] {
                            *l = interval;
                            Some(1)
                        } else {
                            if interval[1] < l[1] {
                                *l = interval;
                            }
                            Some(0)
                        }
                    }
                })
                .sum::<usize>())
        .try_into()
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
            1
        );
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2]]),
            2
        );
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]]),
            0
        );
        assert_eq!(
            Solution::erase_overlap_intervals(vec![
                vec![-52, 31],
                vec![-73, -26],
                vec![82, 97],
                vec![-65, -11],
                vec![-62, -49],
                vec![95, 99],
                vec![58, 95],
                vec![-31, 49],
                vec![66, 98],
                vec![-63, 2],
                vec![30, 47],
                vec![-40, -26]
            ]),
            7
        );
    }
}
