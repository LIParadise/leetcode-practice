pub struct Solution;

impl Solution {
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        let begin_1 = Self::hhmm_to_minute(&event1[0]);
        let end_1 = Self::hhmm_to_minute(&event1[1]);
        let begin_2 = Self::hhmm_to_minute(&event2[0]);
        let end_2 = Self::hhmm_to_minute(&event2[1]);
        Self::num_in_closed_int((begin_1, end_1), begin_2)
            || Self::num_in_closed_int((begin_1, end_1), end_2)
            || Self::num_in_closed_int((begin_2, end_2), begin_1)
            || Self::num_in_closed_int((begin_2, end_2), end_1)
    }
    fn hhmm_to_minute(s: &str) -> usize {
        let mut iter = s.split(':');
        let hour_part = iter.next().unwrap().parse::<usize>().unwrap() * 60;
        let minute_part = iter.next().unwrap().parse::<usize>().unwrap();
        hour_part + minute_part
    }
    fn num_in_closed_int((begin_inclusive, end_inclusive): (usize, usize), num: usize) -> bool {
        begin_inclusive <= num && num <= end_inclusive
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert_eq!(
            Solution::have_conflict(
                vec!["01:15".to_string(), "02:00".to_string()],
                vec!["02:00".to_string(), "03:00".to_string()]
            ),
            true
        );
        assert_eq!(
            Solution::have_conflict(
                vec!["01:00".to_string(), "02:00".to_string()],
                vec!["01:20".to_string(), "03:00".to_string()]
            ),
            true
        );
        assert_eq!(
            Solution::have_conflict(
                vec!["10:00".to_string(), "11:00".to_string()],
                vec!["14:00".to_string(), "15:00".to_string()]
            ),
            false
        );
        assert_eq!(
            Solution::have_conflict(
                vec!["15:19".to_string(), "17:56".to_string()],
                vec!["14:11".to_string(), "20:02".to_string()]
            ),
            true
        );
    }
}
