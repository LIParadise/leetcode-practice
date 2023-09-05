pub struct Solution;

use std::collections::BinaryHeap as BH;
#[derive(std::cmp::Eq, std::cmp::Ord)]
struct IntervalWithCmpByStartingTime<T>(Vec<T>)
where
    T: Sized + Ord;
use IntervalWithCmpByStartingTime as IWCBS;
impl<T> std::cmp::PartialEq for IWCBS<T>
where
    T: std::cmp::Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.0[0].eq(&other.0[0]) && self.0[1].eq(&other.0[1])
    }
}
impl<T> std::cmp::PartialOrd for IWCBS<T>
where
    T: std::cmp::Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0[0].partial_cmp(&other.0[0])
    }
}

impl Solution {
    /// A pair `p2 = [c, d]` follows a pair p1 = [a, b] if b < c.
    /// A chain of pairs can be formed in this fashion.
    /// Return the length longest chain which can be formed.
    // Idea: greedy.
    //
    // Suppose we have the longest chain C = [I_1, I_2, I_3, ..., I_n]
    // The interval with smallest right member (in some sense earliest
    // ending time) is I.
    // Then it's immediate that the chain C_I = [I, I_2, I_3, ..., I_n]
    // is also longest.
    //
    // Similar argument applies to "the latest to start";
    // which is kinda better since Rust std provides max heap.
    //
    // Algorithm:
    // Put them in heap, criteria being that max-heap by starting time.
    // Pop everyone and check if ending time (right member) allows.
    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        if pairs.len() == 0 {
            0
        } else {
            let mut max_heap_by_starting_time: BH<IWCBS<i32>> =
                pairs.into_iter().map(|i| IWCBS(i)).collect();
            let mut latest_start_time = max_heap_by_starting_time.pop().unwrap().0[0];
            let mut ret = 1;
            while let Some(interval) = max_heap_by_starting_time.pop() {
                if interval.0[1] < latest_start_time {
                    ret = ret + 1;
                    latest_start_time = interval.0[0];
                }
            }
            ret
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        let pairs = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(Solution::find_longest_chain(pairs), 2);
        let pairs = vec![vec![1, 2], vec![7, 8], vec![4, 5]];
        assert_eq!(Solution::find_longest_chain(pairs), 3);
    }
}
